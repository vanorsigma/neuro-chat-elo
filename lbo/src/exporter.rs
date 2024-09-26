pub trait Exporter {
    type Performance;
    type AuthorId;
    type Closed;

    fn export(
        &mut self,
        author_id: Self::AuthorId,
        performance: Self::Performance,
    ) -> impl std::future::Future<Output = ()> + Send;
    fn close(self) -> impl std::future::Future<Output = Self::Closed>;
}
