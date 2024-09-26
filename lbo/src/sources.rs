pub trait Source {
    type Message;
    type Closed;

    fn next_message(&mut self) -> impl std::future::Future<Output = Option<Self::Message>> + Send;
    fn close(self) -> impl std::future::Future<Output = Self::Closed> + Send;
}
