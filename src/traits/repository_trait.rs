use std::{
    fmt::{Debug, Display},
    pin::Pin,
};

use crate::traits::{find_option_trait::FindOptionTrait, find_result_trait::FindResultTrait};

pub trait RepositoryTrait: Sync + Send + 'static {
    type Id: Sync + Send + 'static;
    type Entity: Sync + Send + 'static;
    type Error: Debug + Display;
    type FindOptions: FindOptionTrait;
    type FindResult: FindResultTrait;
    fn save<'a>(
        &'a self,
        entity: &'a Self::Entity,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Entity, Self::Error>> + Send + 'a>>;

    fn update<'a>(
        &'a self,
        entity_id: &'a Self::Id,
        entity: &'a Self::Entity,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Entity, Self::Error>> + Send + 'a>>;

    fn delete<'a>(
        &'a self,
        entity_id: &'a Self::Id,
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>>;

    fn find_by_id<'a>(
        &'a self,
        entity_id: &'a Self::Id,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Entity, Self::Error>> + Send + 'a>>;

    fn find_all<'a>(
        &'a self,
        options: &'a Self::FindOptions,
    ) -> Pin<Box<dyn Future<Output = Result<Self::FindResult, Self::Error>> + Send + 'a>>;
}
