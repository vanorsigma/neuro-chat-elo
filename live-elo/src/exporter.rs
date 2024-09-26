pub mod elo_calculator;
pub mod shared_processor;
pub mod websocket;

use lbo::exporter::Exporter;
use websocket::PerformancePoints;
use websocket_shared::AuthorId;

pub struct DummyExporter {}

impl DummyExporter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Exporter for DummyExporter {
    type Performance = PerformancePoints;
    type AuthorId = AuthorId;
    type Closed = ();

    async fn export(&mut self, author_id: Self::AuthorId, performance: Self::Performance) {
        println!("got performance for {author_id:?}: {performance:?}");
    }

    async fn close(self) -> Self::Closed {}
}

pub struct MultiExporter<Head, Tail, Performance, AuthorId, HeadClosed, TailClosed>
where
    Head: Exporter<Performance = Performance, AuthorId = AuthorId, Closed = HeadClosed>,
    Tail: Exporter<Performance = Performance, AuthorId = AuthorId, Closed = TailClosed>,
    Performance: Clone,
    AuthorId: Clone,
{
    head: Head,
    tail: Tail,
}

impl<Head, Tail, Performance, AuthorId, HeadClosed, TailClosed>
    MultiExporter<Head, Tail, Performance, AuthorId, HeadClosed, TailClosed>
where
    Head: Exporter<Performance = Performance, AuthorId = AuthorId, Closed = HeadClosed> + Send,
    Tail: Exporter<Performance = Performance, AuthorId = AuthorId, Closed = TailClosed> + Send,
    Performance: Clone + Send,
    AuthorId: Clone + Send,
{
    pub fn pair(head: Head, tail: Tail) -> Self {
        Self { head, tail }
    }

    pub fn append<T, C>(
        self,
        value: T,
    ) -> MultiExporter<
        T,
        MultiExporter<Head, Tail, Performance, AuthorId, HeadClosed, TailClosed>,
        Performance,
        AuthorId,
        C,
        ClosedMultiExporter<Head, Tail, Performance, AuthorId, HeadClosed, TailClosed>,
    >
    where
        T: Exporter<Performance = Performance, AuthorId = AuthorId, Closed = C> + Send,
    {
        MultiExporter::pair(value, self)
    }
}

impl<Head, Tail, Performance, AuthorId, HeadClosed, TailClosed> Exporter
    for MultiExporter<Head, Tail, Performance, AuthorId, HeadClosed, TailClosed>
where
    Head: Exporter<Performance = Performance, AuthorId = AuthorId, Closed = HeadClosed> + Send,
    Tail: Exporter<Performance = Performance, AuthorId = AuthorId, Closed = TailClosed> + Send,
    Performance: Clone + Send,
    AuthorId: Clone + Send,
{
    type Performance = Performance;
    type AuthorId = AuthorId;
    type Closed = ClosedMultiExporter<Head, Tail, Performance, AuthorId, HeadClosed, TailClosed>;

    async fn export(&mut self, author_id: Self::AuthorId, performance: Self::Performance) {
        self.head
            .export(author_id.clone(), performance.clone())
            .await;
        self.tail.export(author_id, performance).await;
    }

    async fn close(self) -> Self::Closed {
        ClosedMultiExporter {
            head: self.head.close().await,
            tail: self.tail.close().await,
            _phantom: std::marker::PhantomData,
        }
    }
}

pub struct ClosedMultiExporter<Head, Tail, Performance, AuthorId, HeadClosed, TailClosed>
where
    Head: Exporter<Performance = Performance, AuthorId = AuthorId, Closed = HeadClosed>,
    Tail: Exporter<Performance = Performance, AuthorId = AuthorId, Closed = TailClosed>,
    Performance: Clone,
    AuthorId: Clone,
{
    pub head: HeadClosed,
    pub tail: TailClosed,
    _phantom: std::marker::PhantomData<(Head, Tail)>,
}
