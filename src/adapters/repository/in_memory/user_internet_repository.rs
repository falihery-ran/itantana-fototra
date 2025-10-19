use std::{
    collections::HashMap,
    fmt::Debug,
    pin::Pin,
    sync::{Arc, LazyLock},
};

use tokio::sync::RwLock;

use crate::{
    adapters::repository::in_memory::user_repository::InMemoryUserRepository,
    dtos::{
        find_request::FindRequest, find_response::FindResponse,
        user_internet::user_internet_find_request_filter::UserInternetFindRequestFilter,
    },
    model::{
        email_address::EmailAddress,
        user::{UserID, error::UserError},
        user_internet::{UserInternet, error::UserInternetError},
    },
    traits::{
        find_option_trait::FindOptionTrait, initialize_trait::InitializeTrait,
        repository_trait::RepositoryTrait,
        user_internet::user_internet_repository_trait::UserInternetRepositoryTrait,
    },
};

static DB: LazyLock<Arc<RwLock<HashMap<(UserID, EmailAddress), UserInternet>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

#[derive(Debug, Clone)]
pub struct InMemoryUserInternetRepository {
    data: Arc<RwLock<HashMap<(UserID, EmailAddress), UserInternet>>>,
    user_repository: InMemoryUserRepository,
}

impl Default for InMemoryUserInternetRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryUserInternetRepository {
    pub fn new() -> Self {
        Self {
            data: Arc::clone(&DB),
            user_repository: InMemoryUserRepository::new(),
        }
    }
}

impl RepositoryTrait for InMemoryUserInternetRepository {
    type Id = (UserID, EmailAddress);
    type Entity = UserInternet;
    type Error = UserInternetError;
    type FindOptions = FindRequest<UserInternetFindRequestFilter>;
    type FindResult = FindResponse<UserInternet>;

    fn save<'a>(
        &'a self,
        entity: &'a Self::Entity,
    ) -> Pin<
        Box<dyn std::future::Future<Output = Result<UserInternet, UserInternetError>> + Send + 'a>,
    > {
        Box::pin(async move {
            self.user_repository
                .find_by_id(entity.get_user_id())
                .await
                .map_err(|e| match e {
                    UserError::UserNotExists { id } => UserInternetError::UserNotExists { id },
                    ref e => UserInternetError::Unknown(anyhow::anyhow!(e.to_string())),
                })?;
            let user_internet = UserInternet::new(entity.get_user_id(), entity.get_email());
            let mut data = self.data.write().await;
            data.insert(
                (*entity.get_user_id(), entity.get_email().clone()),
                user_internet.clone(),
            );
            Ok(user_internet)
        })
    }

    fn update<'a>(
        &'a self,
        _entity_id: &'a Self::Id,
        _entity: &'a Self::Entity,
    ) -> Pin<
        Box<dyn std::future::Future<Output = Result<UserInternet, UserInternetError>> + Send + 'a>,
    > {
        Box::pin(async move { unimplemented!() })
    }

    fn delete<'a>(
        &'a self,
        entity_id: &'a Self::Id,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(), UserInternetError>> + Send + 'a>> {
        Box::pin(async move {
            let mut data = self.data.write().await;
            if data.contains_key(entity_id) {
                data.remove(entity_id);
                Ok(())
            } else {
                Err(UserInternetError::EmailNotAssociatedToUser {
                    email: entity_id.1.clone(),
                    user_id: entity_id.0,
                })
            }
        })
    }

    fn find_all<'a>(
        &'a self,
        options: &'a Self::FindOptions,
    ) -> Pin<
        Box<
            dyn std::future::Future<Output = Result<FindResponse<UserInternet>, UserInternetError>>
                + Send
                + 'a,
        >,
    > {
        Box::pin(async {
            let query = options.get_query();
            let limit = options.get_limit();
            let order_by = options.get_order_by();
            let offset = options.get_offset();
            let data = self.data.read().await;
            // il faut match email
            let email = match query.email {
                Some(value) => EmailAddress::try_from(&*value).ok(),
                None => None,
            };
            let mut filtered: Vec<UserInternet> = data
                .iter()
                .filter(|(k, _v)| {
                    let mut found = true;
                    if query.user_id.is_some() {
                        found &= query.user_id.unwrap().eq(&k.0);
                    }
                    if email.is_some() {
                        found &= email.as_ref().unwrap().eq(&k.1.clone().try_into().unwrap());
                    }
                    found
                })
                .map(|u| u.1.clone())
                .collect();
            filtered.sort_by(|a, b| match order_by.to_lowercase().as_str() {
                "email" => a.get_email().cmp(b.get_email()),
                _ => a.get_user_id().cmp(b.get_user_id()),
            });
            let mut limited = filtered.chunks(limit as usize);
            let num_page = limited.len();
            let selected = limited
                .nth((offset as usize) - 1)
                .map_or(Vec::new(), |chunk| chunk.to_vec());
            Ok(FindResponse::<UserInternet>::new(selected, num_page as u64))
        })
    }

    fn find_by_id<'a>(
        &'a self,
        _entity_id: &'a Self::Id,
    ) -> Pin<
        Box<dyn std::future::Future<Output = Result<UserInternet, UserInternetError>> + Send + 'a>,
    > {
        Box::pin(async { unimplemented!() })
    }
}

impl InitializeTrait for InMemoryUserInternetRepository {}

impl UserInternetRepositoryTrait for InMemoryUserInternetRepository {}
