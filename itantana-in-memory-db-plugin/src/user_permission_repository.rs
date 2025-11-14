use std::{
    collections::HashMap,
    pin::Pin,
    sync::{Arc, LazyLock},
};

use tokio::sync::RwLock;

use fototra::{
    dtos::{
        find_request::FindRequest, find_response::FindResponse,
        user_permission::user_permission_find_request_filter::UserPermissionFindRequestFilter,
    },
    model::{
        permission::{ALL_PERMISSIONS, Permission},
        user::{DEFAULT_ADMIN_USER, UserID, error::UserError},
        user_permission::{UserPermission, error::UserPermissionError},
    },
    traits::{
        find_option_trait::FindOptionTrait, initialize_trait::InitializeTrait,
        repository_trait::RepositoryTrait,
        user_permission::user_permission_repository::UserPermissionRepositoryTrait,
    },
};

use crate::{
    permission_repository::InMemoryPermissionRepository, user_repository::InMemoryUserRepository,
};

static DB: LazyLock<Arc<RwLock<HashMap<(UserID, Permission), UserPermission>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

#[derive(Debug, Clone)]
pub struct InMemoryUserPermissionRepository {
    data: Arc<RwLock<HashMap<(UserID, Permission), UserPermission>>>,
    permission_repository: InMemoryPermissionRepository,
    user_repository: InMemoryUserRepository,
}

impl Default for InMemoryUserPermissionRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryUserPermissionRepository {
    pub fn new() -> Self {
        Self {
            data: DB.clone(),
            permission_repository: InMemoryPermissionRepository::new(),
            user_repository: InMemoryUserRepository::new(),
        }
    }
}

impl RepositoryTrait for InMemoryUserPermissionRepository {
    type Id = (UserID, Permission);
    type Entity = UserPermission;
    type Error = UserPermissionError;
    type FindOptions = FindRequest<UserPermissionFindRequestFilter>;
    type FindResult = FindResponse<UserPermission>;

    fn save<'a>(
        &'a self,
        entity: &'a Self::Entity,
    ) -> Pin<
        Box<
            dyn std::future::Future<Output = Result<UserPermission, UserPermissionError>>
                + Send
                + 'a,
        >,
    > {
        Box::pin(async move {
            self.user_repository
                .find_by_id(entity.get_user_id())
                .await
                .map_err(|e| match e {
                    UserError::UserNotExists { id } => UserPermissionError::UserNotExists { id },
                    ref e => UserPermissionError::Unknown(anyhow::anyhow!(e.to_string())),
                })?;
            self.permission_repository
                .find_by_id(&entity.get_permission().to_string())
                .await
                .map_err(|e| UserPermissionError::Unknown(anyhow::anyhow!(e.to_string())))?;
            let user_permission =
                UserPermission::new(entity.get_user_id(), entity.get_permission());
            let mut data = self.data.write().await;
            data.insert(
                (*entity.get_user_id(), entity.get_permission().to_string()),
                user_permission.clone(),
            );
            Ok(user_permission)
        })
    }

    fn update<'a>(
        &'a self,
        _entity_id: &'a Self::Id,
        _entity: &'a Self::Entity,
    ) -> Pin<
        Box<
            dyn std::future::Future<Output = Result<UserPermission, UserPermissionError>>
                + Send
                + 'a,
        >,
    > {
        Box::pin(async move { unimplemented!() })
    }

    fn delete<'a>(
        &'a self,
        entity_id: &'a Self::Id,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(), UserPermissionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let mut data = self.data.write().await;
            if data.contains_key(entity_id) {
                data.remove(entity_id);
                Ok(())
            } else {
                Err(UserPermissionError::PermissionAlreadyNotAssigned {
                    permission: entity_id.1.clone(),
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
                    Output = Result<FindResponse<UserPermission>, UserPermissionError>,
                > + Send
                + 'a,
        >,
    > {
        Box::pin(async {
            let query = options.get_query();
            let limit = options.get_limit();
            let order_by = options.get_order_by();
            let offset = options.get_offset();
            let data = self.data.read().await;
            let mut filtered: Vec<UserPermission> = data
                .iter()
                .filter(|(k, _v)| {
                    let mut found = true;
                    if query.user_id.is_some() {
                        found &= query.user_id.unwrap().eq(&k.0);
                    }
                    if query.permission.is_some() {
                        found &= query.permission.as_ref().unwrap().eq(&k.1);
                    }
                    found
                })
                .map(|u| u.1.clone())
                .collect();
            filtered.sort_by(|a, b| match order_by.to_lowercase().as_str() {
                "permission" => a.get_permission().cmp(b.get_permission()),
                _ => a.get_user_id().cmp(b.get_user_id()),
            });
            let mut limited = filtered.chunks(limit as usize);
            let num_page = limited.len();
            let selected = limited
                .nth((offset as usize) - 1)
                .map_or(Vec::new(), |chunk| chunk.to_vec());
            Ok(FindResponse::<UserPermission>::new(
                selected,
                num_page as u64,
            ))
        })
    }

    fn find_by_id<'a>(
        &'a self,
        entity_id: &'a Self::Id,
    ) -> Pin<
        Box<
            dyn std::future::Future<Output = Result<UserPermission, UserPermissionError>>
                + Send
                + 'a,
        >,
    > {
        Box::pin(async {
            match self.data.read().await.get(entity_id) {
                Some(u) => Ok(u.clone()),
                None => Err(UserPermissionError::PermissionAlreadyNotAssigned {
                    permission: entity_id.1.clone(),
                }),
            }
        })
    }
}

impl UserPermissionRepositoryTrait for InMemoryUserPermissionRepository {}

impl InitializeTrait for InMemoryUserPermissionRepository {
    fn initialize<'a>(
        &'a self,
    ) -> std::pin::Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + 'a>> {
        Box::pin(async {
            let default_admin = &DEFAULT_ADMIN_USER;
            let all_permissions = ALL_PERMISSIONS;
            for perm in all_permissions {
                self.save(&UserPermission::new(default_admin.get_id(), perm))
                    .await
                    .unwrap();
            }
            Ok(())
        })
    }
}
