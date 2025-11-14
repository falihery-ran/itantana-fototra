use std::pin::Pin;

use crate::{
    dtos::{find_request::FindRequest, find_response::FindResponse},
    model::{
        password::password_level::PasswordLevel,
        user_password::user_password_policy::{UserPasswordPolicy, error::UserPasswordPolicyError},
    },
    traits::repository_trait::RepositoryTrait,
};

pub trait UserPasswordPolicyRepositoryTrait:
    RepositoryTrait<
        Id = PasswordLevel,
        Entity = UserPasswordPolicy,
        Error = UserPasswordPolicyError,
        FindOptions = FindRequest<String>,
        FindResult = FindResponse<UserPasswordPolicy>,
    >
{
    fn get_all_policies(&self) -> &'static [&'static str] {
        &["basic", "medium", "strong", "very_strong"]
    }

    fn change_policy<'a>(
        &'a self,
        password_level: &'a PasswordLevel,
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>>;

    fn get_policy<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Result<UserPasswordPolicy, Self::Error>> + Send + 'a>> {
        Box::pin(async { Ok(UserPasswordPolicy(PasswordLevel::default())) })
    }
}
