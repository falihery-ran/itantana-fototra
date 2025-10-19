use std::{
    collections::HashSet,
    pin::Pin,
    sync::{Arc, LazyLock},
};

use tokio::sync::RwLock;

use crate::{
    dtos::{find_request::FindRequest, find_response::FindResponse},
    model::permission::{ALL_PERMISSIONS, Permission, error::PermissionError},
    traits::{
        find_option_trait::FindOptionTrait, initialize_trait::InitializeTrait,
        permission::permission_repository_trait::PermissionRepositoryTrait,
        repository_trait::RepositoryTrait,
    },
};

static DB: LazyLock<Arc<RwLock<HashSet<Permission>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(HashSet::new())));

#[derive(Debug, Clone)]
pub struct InMemoryPermissionRepository {
    data: Arc<RwLock<HashSet<Permission>>>,
}

impl InMemoryPermissionRepository {
    pub fn new() -> Self {
        Self {
            data: Arc::clone(&DB),
        }
    }
}

impl Default for InMemoryPermissionRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl RepositoryTrait for InMemoryPermissionRepository {
    type Id = Permission;
    type Entity = Permission;
    type Error = PermissionError;
    type FindOptions = FindRequest<String>;
    type FindResult = FindResponse<Permission>;

    fn save<'a>(
        &'a self,
        entity: &'a Self::Entity,
    ) -> Pin<
        Box<
            dyn std::future::Future<Output = Result<std::string::String, PermissionError>>
                + Send
                + 'a,
        >,
    > {
        Box::pin(async move {
            let mut data = self.data.write().await;
            data.insert(entity.clone());
            Ok(entity.clone())
        })
    }

    fn update<'a>(
        &'a self,
        _entity_id: &Self::Id,
        _entity: &Self::Entity,
    ) -> Pin<
        Box<
            dyn std::future::Future<Output = Result<std::string::String, PermissionError>>
                + Send
                + 'a,
        >,
    > {
        Box::pin(async move { unimplemented!() })
    }

    fn delete<'a>(
        &'a self,
        entity_id: &'a Self::Id,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(), PermissionError>> + Send + 'a>> {
        Box::pin(async move {
            let mut data = self.data.write().await;
            if data.contains(entity_id) {
                data.remove(entity_id);
                Ok(())
            } else {
                Err(PermissionError::PermissionNotExists {
                    name: entity_id.clone(),
                })
            }
        })
    }

    fn find_all<'a>(
        &'a self,
        options: &'a Self::FindOptions,
    ) -> Pin<
        Box<
            dyn std::future::Future<
                    Output = Result<FindResponse<std::string::String>, PermissionError>,
                > + Send
                + 'a,
        >,
    > {
        Box::pin(async {
            let query = options.get_query();
            let limit = options.get_limit();
            let _order_by = options.get_order_by();
            let offset = options.get_offset();
            let data = self.data.read().await;
            let filtered: Vec<Permission> = data
                .iter()
                .filter(|k| {
                    let mut found = true;

                    found &= query.eq(*k);

                    found
                })
                .cloned()
                .collect();

            let mut limited = filtered.chunks(limit as usize);
            let num_page = limited.len();
            let selected = limited
                .nth((offset as usize) - 1)
                .map_or(Vec::new(), |chunk| chunk.to_vec());
            Ok(FindResponse::<Permission>::new(selected, num_page as u64))
        })
    }

    fn find_by_id<'a>(
        &'a self,
        entity_id: &'a Self::Id,
    ) -> Pin<Box<dyn Future<Output = Result<std::string::String, PermissionError>> + Send + 'a>>
    {
        Box::pin(async move {
            match self.data.read().await.get(entity_id) {
                Some(u) => Ok(u.clone()),
                None => Err(PermissionError::PermissionNotExists {
                    name: entity_id.clone(),
                }),
            }
        })
    }
}

impl InitializeTrait for InMemoryPermissionRepository {
    fn initialize<'a>(&'a self) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + 'a>> {
        Box::pin(async move {
            for perm in ALL_PERMISSIONS {
                self.save(&perm.to_string()).await?;
            }
            Ok(())
        })
    }
}

impl PermissionRepositoryTrait for InMemoryPermissionRepository {}
