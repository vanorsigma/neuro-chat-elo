pub trait ScoringSystem {
    type Message;
    type Performance;
    type Closed;

    fn score_message(&self, message: Self::Message) -> Self::Performance;
    fn close(self) -> impl std::future::Future<Output = Self::Closed>;
}
