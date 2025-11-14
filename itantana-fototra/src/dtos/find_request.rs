use std::fmt::Debug;

use serde::Deserialize;

use crate::{
    dtos::error::find_request_error::FindRequestError,
    traits::{find_option_trait::FindOptionTrait, find_request_trait::FindRequestTrait},
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct FindRequest<T>
where
    T: Default + 'static,
{
    filters: T,
    order_by: String,
    per_page: u16,
    offset: u64,
}

impl<T> FindRequest<T>
where
    T: Default + for<'de> Deserialize<'de> + Debug + Clone + Send + Sync + 'static,
{
    pub fn new(
        filters: &T,
        order_by: &str,
        per_page: &u16,
        offset: &u64,
    ) -> Result<Self, FindRequestError> {
        let per_page = Self::validate_per_page(per_page)?;
        let offset = Self::validate_offset(offset)?;
        Ok(Self {
            filters: filters.clone(),
            order_by: order_by.to_string(),
            per_page: *per_page,
            offset: *offset,
        })
    }
}

impl<T> Default for FindRequest<T>
where
    T: Default + for<'de> Deserialize<'de> + Debug + Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self {
            filters: T::default(),
            order_by: String::new(),
            per_page: 25,
            offset: 1,
        }
    }
}

impl<T> FindRequestTrait for FindRequest<T>
where
    T: Default + for<'de> Deserialize<'de> + Debug + Clone + Send + Sync + 'static,
{
    type Filter = T;

    fn set_filters(&mut self, filters: &Self::Filter) {
        self.filters = filters.clone();
    }

    fn set_per_page(&mut self, per_page: &u16) {
        self.per_page = *per_page;
    }

    fn set_offset(&mut self, offset: &u64) {
        self.offset = *offset;
    }
}

impl<T> FindOptionTrait for FindRequest<T>
where
    T: Default + for<'de> Deserialize<'de> + Debug + Clone + Send + Sync + 'static,
{
    type QueryFilter = T;
    fn get_query(&self) -> Self::QueryFilter {
        self.filters.clone()
    }
    fn get_order_by(&self) -> String {
        self.order_by.clone()
    }
    fn get_offset(&self) -> u64 {
        self.offset
    }
    fn get_limit(&self) -> u16 {
        self.per_page
    }
}
