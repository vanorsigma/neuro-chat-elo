use app_state::AppState;

use crate::{message::AuthoredMesasge, scoring::ScoringSystem};

pub trait PerformanceProcessor: Send {
    type Message;
    type Closed;

    fn process_message(
        &mut self,
        message: Self::Message,
    ) -> impl std::future::Future<Output = ()> + Send;
    fn close(self) -> impl std::future::Future<Output = Self::Closed>;
}

pub struct StandardLeaderboard<
    Scoring,
    Exporter,
    Message,
    Id,
    Performance,
    ScoringClosed,
    ExporterClosed,
    State,
> where
    Message: AuthoredMesasge<Id = Id>,
    Scoring: ScoringSystem<Message = Message, Performance = Performance, Closed = ScoringClosed>,
    Exporter: crate::exporter::Exporter<
        AuthorId = Id,
        Performance = Performance,
        Closed = ExporterClosed,
    >,
    State: AppState
{
    scoring_system: Scoring,
    exporter: Exporter,
    app_state: State
}

impl<Scoring, Exporter, Message, Id, Performance, ScoringClosed, ExporterClosed, State>
    StandardLeaderboard<Scoring, Exporter, Message, Id, Performance, ScoringClosed, ExporterClosed, State>
where
    Message: AuthoredMesasge<Id = Id>,
    Scoring: ScoringSystem<Message = Message, Performance = Performance, Closed = ScoringClosed>,
    Exporter: crate::exporter::Exporter<
        AuthorId = Id,
        Performance = Performance,
        Closed = ExporterClosed,
    >,
    State: AppState,
{
    pub fn new(scoring_system: Scoring, exporter: Exporter, app_state: State) -> Self {
        Self {
            scoring_system,
            exporter,
            app_state,
        }
    }
}

impl<Scoring, Exporter, Message, Id, Performance, ScoringClosed, ExporterClosed, State>
    PerformanceProcessor
    for StandardLeaderboard<
        Scoring,
        Exporter,
        Message,
        Id,
        Performance,
        ScoringClosed,
        ExporterClosed,
        State
    >
where
    Message: AuthoredMesasge<Id = Id> + Send,
    Scoring: ScoringSystem<Message = Message, Performance = Performance, Closed = ScoringClosed>,
    Exporter: crate::exporter::Exporter<AuthorId = Id, Performance = Performance, Closed = ExporterClosed>
        + Send,
    Scoring: Send,
    State: AppState + Send,
{
    type Message = Message;
    type Closed = ClosedStandardLeaderboard<
        Scoring,
        Exporter,
        Message,
        Id,
        Performance,
        ScoringClosed,
        ExporterClosed,
    >;

    async fn process_message(&mut self, message: Self::Message) {
        let message_author_id = message.author_id();
        let score = self.scoring_system.score_message(message);
        self.app_state.do_nothing();
        self.exporter.export(message_author_id, score).await;
    }

    async fn close(self) -> Self::Closed {
        ClosedStandardLeaderboard {
            scoring: self.scoring_system.close().await,
            exporter: self.exporter.close().await,
            _phantom: std::marker::PhantomData,
        }
    }
}

pub struct ClosedStandardLeaderboard<
    Scoring,
    Exporter,
    Message,
    Id,
    Performance,
    ScoringClosed,
    ExporterClosed,
> where
    Scoring: ScoringSystem<Message = Message, Performance = Performance, Closed = ScoringClosed>,
    Exporter: crate::exporter::Exporter<
        AuthorId = Id,
        Performance = Performance,
        Closed = ExporterClosed,
    >,
{
    pub scoring: ScoringClosed,
    pub exporter: ExporterClosed,
    _phantom: std::marker::PhantomData<(Scoring, Exporter, Message, Id, Performance)>,
}
