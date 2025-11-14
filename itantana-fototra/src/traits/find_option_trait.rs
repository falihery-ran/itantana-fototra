pub trait FindOptionTrait: Sync + Send + 'static {
    type QueryFilter: Sync + Send + 'static;
    fn get_query(&self) -> Self::QueryFilter;
    fn get_order_by(&self) -> String {
        String::from("id")
    }
    fn get_limit(&self) -> u16 {
        25
    }
    fn get_offset(&self) -> u64 {
        1
    }
}
