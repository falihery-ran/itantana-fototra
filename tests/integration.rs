mod user;

use itantana_fototra::{
    adapters::{Adapter, repository::in_memory::InMemoryRepository},
    runtime::Runtime,
};
use user::test_users;

#[tokio::test]
async fn tests() {
    Adapter::insert(Box::new(InMemoryRepository)).await;
    Runtime::init().await.unwrap();

    test_users().await;
}
