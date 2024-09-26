use lbo::filter::Filter;

pub struct DummyFilter<M> {
    _phantom: std::marker::PhantomData<M>,
}

impl<M> DummyFilter<M> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<M> Filter for DummyFilter<M> {
    type Message = M;
    type Closed = ();

    fn keep(&self, _: &Self::Message) -> bool {
        true
    }

    async fn close(self) -> Self::Closed {}
}
