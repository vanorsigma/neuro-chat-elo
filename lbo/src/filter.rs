pub trait Filter {
    type Message;
    type Closed;

    fn keep(&self, message: &Self::Message) -> bool;
    fn close(self) -> impl std::future::Future<Output = Self::Closed>;
}
