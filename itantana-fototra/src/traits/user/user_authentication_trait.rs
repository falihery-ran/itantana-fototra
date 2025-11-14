use crate::model::user::User;

pub trait UserAuthenticationTrait: Clone {
    fn authenticate(&self) -> User;
}
