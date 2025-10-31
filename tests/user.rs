use std::sync::Arc;

use fototra::{
    dtos::{
        find_request::FindRequest,
        user::{
            user_add_request::UserAddRequest, user_delete_request::UserDeleteRequest,
            user_find_request_filter::UserFindRequestFilter,
            user_update_request::UserUpdateRequest,
        },
    },
    model::user::{error::UserError, name::Name},
    security::error::SecurityError,
    service::{error::ServiceError, user::UserService},
    traits::{
        authentication_trait::AuthenticationTrait, authorization_trait::AuthorizationTrait,
        find_result_trait::FindResultTrait,
    },
};
use uuid::Uuid;

struct Token {
    pub authenticated: Option<Arc<Entity>>,
}

struct Entity {
    pub authorized: bool,
}

impl AuthorizationTrait for Entity {
    fn authorize<'a>(
        &'a self,
        _permission: &str,
    ) -> std::pin::Pin<Box<dyn Future<Output = Result<(), SecurityError>> + Send + 'a>> {
        Box::pin(async {
            if self.authorized {
                return Ok(());
            } else {
                return Err(SecurityError::NotAuthorized);
            }
        })
    }
}

impl AuthenticationTrait for Token {
    fn authenticate<'a>(
        &'a self,
    ) -> std::pin::Pin<
        Box<
            dyn Future<Output = Result<Arc<dyn AuthorizationTrait + 'static>, SecurityError>>
                + Send
                + 'a,
        >,
    > {
        Box::pin(async {
            match self.authenticated.clone() {
                Some(authorized) => Ok(authorized as Arc<dyn AuthorizationTrait + 'static>),
                None => Err(SecurityError::NotAuthenticated),
            }
        })
    }
}

pub async fn test_users() {
    // Create user
    let firstname = Name::new("Jules").unwrap();
    let lastname = Name::new("RAKOTOBE").unwrap();
    let user_add_request = UserAddRequest::new(&firstname, Some(&lastname));

    let entity = Entity { authorized: true };

    let token = Token {
        authenticated: Some(Arc::new(entity)),
    };

    let created_user = UserService::create(&token, &user_add_request)
        .await
        .map_err(|e| {
            let mut msg = String::new();
            if let Some(error) = e.get::<SecurityError>() {
                msg = error.to_string();
            } else if let Some(error) = e.get::<UserError>() {
                msg = error.to_string();
            }
            println!("error message: {}", msg);
            msg
        })
        .unwrap();

    assert!(!created_user.get_id().is_nil());
    assert_eq!(created_user.get_firstname(), &firstname);
    assert_eq!(created_user.get_lastname(), Some(lastname.clone()).as_ref());

    // update
    let firstname = Name::new("Ignace").unwrap();
    let user_update_request =
        UserUpdateRequest::new(created_user.get_id(), &firstname, Some(&lastname));
    let updated_user = UserService::update(&token, created_user.get_id(), &user_update_request)
        .await
        .unwrap();
    assert_eq!(updated_user.get_id(), created_user.get_id());
    assert_eq!(updated_user.get_firstname(), &firstname);
    assert_eq!(updated_user.get_lastname(), Some(lastname.clone()).as_ref());

    // try to update the wrong user
    let user_update_request =
        UserUpdateRequest::new(created_user.get_id(), &firstname, Some(&lastname));
    let wrong_user_id = Uuid::new_v4();
    let user_mismatch_id_err = UserService::update(&token, &wrong_user_id, &user_update_request)
        .await
        .unwrap_err();
    assert_eq!(
        format!("{:?}", user_mismatch_id_err),
        format!(
            "{:?}",
            ServiceError::new(UserError::MismatchUserId {
                id1: wrong_user_id,
                id2: updated_user.get_id().clone()
            })
        )
    );

    // find user
    let find_request = FindRequest::<UserFindRequestFilter>::default();
    let user_list = UserService::find(&token, &find_request).await.unwrap();

    println!("{:?}", user_list);

    assert_eq!(user_list.get_page_count(), 1);
    assert_eq!(user_list.get_result().count(), 2);
    assert!(
        user_list
            .get_result()
            .find(|user| *user == updated_user)
            .is_some()
    );

    // find one user
    let user = UserService::find_one(&token, &updated_user.get_id())
        .await
        .unwrap();
    assert_eq!(user, updated_user);

    // delete user
    let user_delete_request = UserDeleteRequest::new(updated_user.get_id());

    UserService::delete(&token, &user_delete_request)
        .await
        .unwrap();

    // try to find one deleted user
    let user_not_exist_err = UserService::find_one(&token, &updated_user.get_id())
        .await
        .unwrap_err();
    assert_eq!(
        format!("{:?}", user_not_exist_err),
        format!(
            "{:?}",
            ServiceError::new(UserError::UserNotExists {
                id: updated_user.get_id().clone()
            })
        )
    );

    // try to update deleted user
    let user_not_exist_err =
        UserService::update(&token, &updated_user.get_id(), &user_update_request)
            .await
            .unwrap_err();
    assert_eq!(
        format!("{:?}", user_not_exist_err),
        format!(
            "{:?}",
            ServiceError::new(UserError::UserNotExists {
                id: updated_user.get_id().clone()
            })
        )
    );

    // try to delete deleted user
    let user_delete_request = UserDeleteRequest::new(updated_user.get_id());
    let user_not_exist_err = UserService::delete(&token, &user_delete_request)
        .await
        .unwrap_err();
    assert_eq!(
        format!("{:?}", user_not_exist_err),
        format!(
            "{:?}",
            ServiceError::new(UserError::UserNotExists {
                id: updated_user.get_id().clone()
            })
        )
    );
}
