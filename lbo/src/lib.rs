pub mod exporter;
pub mod filter;
pub mod message;
pub mod performances;
pub mod scoring;
pub mod sources;

pub struct PipelineBuilder<
    Source,
    Filter,
    Performances,
    Message,
    SourceClosed,
    FilterClosed,
    PerformancesClosed,
> where
    Source: sources::Source<Message = Message, Closed = SourceClosed>,
    Filter: filter::Filter<Message = Message, Closed = FilterClosed>,
    Performances:
        performances::PerformanceProcessor<Message = Message, Closed = PerformancesClosed>,
{
    source: Option<Source>,
    filter: Option<Filter>,
    performances: Option<Performances>,
}

impl<Source, Filter, Performances, Message, SourceClosed, FilterClosed, PerformancesClosed>
    PipelineBuilder<
        Source,
        Filter,
        Performances,
        Message,
        SourceClosed,
        FilterClosed,
        PerformancesClosed,
    >
where
    Source: sources::Source<Message = Message, Closed = SourceClosed>,
    Filter: filter::Filter<Message = Message, Closed = FilterClosed>,
    Performances:
        performances::PerformanceProcessor<Message = Message, Closed = PerformancesClosed>,
{
    pub fn new() -> Self {
        Self {
            source: None,
            filter: None,
            performances: None,
        }
    }

    pub fn source(mut self, source: Source) -> Self {
        self.source = Some(source);
        self
    }

    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }

    pub fn performances(mut self, performances: Performances) -> Self {
        self.performances = Some(performances);
        self
    }

    pub fn build(
        self,
    ) -> Pipeline<
        Source,
        Filter,
        Performances,
        Message,
        SourceClosed,
        FilterClosed,
        PerformancesClosed,
    > {
        Pipeline::new(
            self.source.unwrap(),
            self.filter.unwrap(),
            self.performances.unwrap(),
        )
    }
}

impl<Source, Filter, Performances, Message, SourceClosed, FilterClosed, PerformancesClosed> Default
    for PipelineBuilder<
        Source,
        Filter,
        Performances,
        Message,
        SourceClosed,
        FilterClosed,
        PerformancesClosed,
    >
where
    Source: sources::Source<Message = Message, Closed = SourceClosed>,
    Filter: filter::Filter<Message = Message, Closed = FilterClosed>,
    Performances:
        performances::PerformanceProcessor<Message = Message, Closed = PerformancesClosed>,
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct Pipeline<
    Source,
    Filter,
    Performances,
    Message,
    SourceClosed,
    FilterClosed,
    PerformancesClosed,
> where
    Source: sources::Source<Message = Message, Closed = SourceClosed>,
    Filter: filter::Filter<Message = Message, Closed = FilterClosed>,
    Performances:
        performances::PerformanceProcessor<Message = Message, Closed = PerformancesClosed>,
{
    source: Source,
    filter: Filter,
    performances: Performances,
}

impl<Source, Filter, Performances, Message, SourceClosed, FilterClosed, PerformancesClosed>
    Pipeline<Source, Filter, Performances, Message, SourceClosed, FilterClosed, PerformancesClosed>
where
    Source: sources::Source<Message = Message, Closed = SourceClosed>,
    Filter: filter::Filter<Message = Message, Closed = FilterClosed>,
    Performances:
        performances::PerformanceProcessor<Message = Message, Closed = PerformancesClosed>,
{
    pub fn builder() -> PipelineBuilder<
        Source,
        Filter,
        Performances,
        Message,
        SourceClosed,
        FilterClosed,
        PerformancesClosed,
    > {
        PipelineBuilder::new()
    }

    pub fn new(source: Source, filter: Filter, performances: Performances) -> Self {
        Self {
            source,
            filter,
            performances,
        }
    }

    pub async fn run(mut self) -> Result<Self, ()> {
        while let Some(message) = self.source.next_message().await {
            if !self.filter.keep(&message) {
                continue;
            }

            self.performances.process_message(message).await;
        }

        Ok(self)
    }

    pub async fn close(
        self,
    ) -> ClosedPipeline<
        Source,
        Filter,
        Performances,
        Message,
        SourceClosed,
        FilterClosed,
        PerformancesClosed,
    > {
        ClosedPipeline {
            source: self.source.close().await,
            filter: self.filter.close().await,
            performances: self.performances.close().await,
            _phantom: std::marker::PhantomData,
        }
    }
}

pub struct ClosedPipeline<
    Source,
    Filter,
    Performances,
    Message,
    SourceClosed,
    FilterClosed,
    PerformancesClosed,
> where
    Source: sources::Source<Message = Message, Closed = SourceClosed>,
    Filter: filter::Filter<Message = Message, Closed = FilterClosed>,
    Performances:
        performances::PerformanceProcessor<Message = Message, Closed = PerformancesClosed>,
{
    pub source: SourceClosed,
    pub filter: FilterClosed,
    pub performances: PerformancesClosed,
    _phantom: std::marker::PhantomData<(Source, Filter, Performances)>,
}
