use serde::Serialize;

use crate::traits::find_result_trait::FindResultTrait;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FindResponse<T>
where
    T: Clone + Send + Sync + 'static,
{
    data: Vec<T>,
    num_pages: u64,
}

impl<T> FindResponse<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new(data: Vec<T>, num_pages: u64) -> Self {
        Self { data, num_pages }
    }
}

impl<T> FindResultTrait for FindResponse<T>
where
    T: Clone + Send + Sync + 'static,
{
    type Entity = T;
    fn get_result<'a>(&'a self) -> Box<dyn Iterator<Item = Self::Entity> + Send + 'a> {
        Box::new(self.data.clone().into_iter())
    }
    fn get_page_count(&self) -> u64 {
        self.num_pages
    }
}
