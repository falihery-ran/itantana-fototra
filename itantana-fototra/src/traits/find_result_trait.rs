pub trait FindResultTrait: Sync + Send + 'static {
    type Entity: Sync + Send + 'static;
    fn get_page_count(&self) -> u64;
    fn get_result<'a>(&'a self) -> Box<dyn Iterator<Item = Self::Entity> + Send + 'a>;
}
