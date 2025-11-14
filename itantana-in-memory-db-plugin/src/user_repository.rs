use std::sync::LazyLock;
use std::{collections::HashMap, pin::Pin, sync::Arc};

use fototra::dtos::find_request::FindRequest;
use fototra::dtos::find_response::FindResponse;
use fototra::dtos::user::user_find_request_filter::UserFindRequestFilter;
use fototra::model::user::error::UserError;
use fototra::model::user::{DEFAULT_ADMIN_USER, User, UserID};
use fototra::traits::find_option_trait::FindOptionTrait;
use fototra::traits::initialize_trait::InitializeTrait;
use fototra::traits::repository_trait::RepositoryTrait;
use fototra::traits::user::user_initialize_trait::UserInitializeTrait;
use fototra::traits::user::user_repository_trait::UserRepositoryTrait;
use tokio::sync::RwLock;
use uuid::Uuid;

static DB: LazyLock<Arc<RwLock<HashMap<UserID, User>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

#[derive(Debug, Clone)]
pub struct InMemoryUserRepository {
    data: Arc<RwLock<HashMap<UserID, User>>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            data: Arc::clone(&DB),
        }
    }
}

impl Default for InMemoryUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl RepositoryTrait for InMemoryUserRepository {
    type Id = UserID;
    type Entity = User;
    type Error = UserError;
    type FindOptions = FindRequest<UserFindRequestFilter>;
    type FindResult = FindResponse<User>;

    fn save<'a>(
        &'a self,
        entity: &'a Self::Entity,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Entity, Self::Error>> + Send + 'a>> {
        Box::pin(async move {
            let user_id = if entity.get_id().is_nil() {
                Uuid::now_v7()
            } else {
                *entity.get_id()
            };
            let user = User::new(&user_id, entity.get_firstname(), entity.get_lastname());
            let mut data = self.data.write().await;
            data.insert(user_id, user.clone());
            Ok(user)
        })
    }

    fn update<'a>(
        &'a self,
        entity_id: &'a Self::Id,
        entity: &'a Self::Entity,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Entity, Self::Error>> + Send + 'a>> {
        Box::pin(async move {
            let mut data = self.data.write().await;
            if entity_id.ne(entity.get_id()) {
                Err(UserError::MismatchUserId {
                    id1: *entity_id,
                    id2: *entity.get_id(),
                })
            } else if data.contains_key(entity_id) {
                data.insert(*entity_id, entity.clone());
                Ok(entity.clone())
            } else {
                Err(UserError::UserNotExists { id: *entity_id })
            }
        })
    }

    fn delete<'a>(
        &'a self,
        entity_id: &'a Self::Id,
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>> {
        Box::pin(async move {
            let mut data = self.data.write().await;
            if data.contains_key(entity_id) {
                data.remove(entity_id);
                Ok(())
            } else {
                Err(UserError::UserNotExists { id: *entity_id })
            }
        })
    }

    fn find_all<'a>(
        &'a self,
        options: &'a Self::FindOptions,
    ) -> Pin<Box<dyn Future<Output = Result<Self::FindResult, Self::Error>> + Send + 'a>> {
        Box::pin(async {
            let query = options.get_query();
            let limit = options.get_limit();
            let order_by = options.get_order_by();
            let offset = options.get_offset();
            let data = self.data.read().await;
            let mut filtered: Vec<User> = data
                .iter()
                .filter(|(k, v)| {
                    let mut found = true;
                    if query.id.is_some() {
                        found &= query.id.unwrap().eq(*k);
                    }
                    if query.firstname.is_some() {
                        found &= query
                            .firstname
                            .as_ref()
                            .unwrap()
                            .contains(&v.get_firstname().to_string());
                    }
                    if query.lastname.is_some() {
                        found &= query
                            .lastname
                            .as_ref()
                            .unwrap()
                            .contains(&v.get_lastname().map_or("", |n| n).to_string());
                    }
                    found
                })
                .map(|u| u.1.clone())
                .collect();
            filtered.sort_by(|a, b| match order_by.to_lowercase().as_str() {
                "firstname" => a.get_firstname().cmp(b.get_firstname()),
                "lastname" => a.get_lastname().cmp(&b.get_lastname()),
                _ => a.get_id().cmp(b.get_id()),
            });
            let mut limited = filtered.chunks(limit as usize);
            let num_page = limited.len();
            let selected = limited
                .nth((offset as usize) - 1)
                .map_or(Vec::new(), |chunk| chunk.to_vec());
            Ok(FindResponse::<User>::new(selected, num_page as u64))
        })
    }

    fn find_by_id<'a>(
        &'a self,
        entity_id: &'a Self::Id,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Entity, Self::Error>> + Send + 'a>> {
        Box::pin(async {
            match self.data.read().await.get(entity_id) {
                Some(u) => Ok(u.clone()),
                None => Err(UserError::UserNotExists { id: *entity_id }),
            }
        })
    }
}

impl UserRepositoryTrait for InMemoryUserRepository {}

impl InitializeTrait for InMemoryUserRepository {
    fn initialize<'a>(&'a self) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + 'a>> {
        Box::pin(async {
            let user = DEFAULT_ADMIN_USER.clone();
            self.save(&user).await?;
            Ok(())
        })
    }
}

impl UserInitializeTrait for InMemoryUserRepository {}
