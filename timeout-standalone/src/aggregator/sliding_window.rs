///! A sliding window that supports "future" taking. So, something like:
///! take 20 items from a SlidingWindow of 10.
///! Calling the releavant functions on SlidingWindow will produce
///! thread-safe consumers.
use std::{
    cmp::{max, min},
    collections::VecDeque,
    future::Future,
    sync::{atomic::AtomicUsize, Arc},
};

use ::futures::{poll, ready};
use tokio::{sync::broadcast, task::futures};
use tokio_stream::Stream;
use tokio_util::sync::ReusableBoxFuture;

#[derive(Debug)]
pub(crate) struct SlidingWindow<T> {
    master_sliding_window: VecDeque<T>,
    capacity: usize,
    transmitter: broadcast::Sender<T>,
}

struct SlidingWindowIterator<T> {
    initial_items: Vec<T>,
    inner: ReusableBoxFuture<
        'static,
        (
            Result<T, broadcast::error::RecvError>,
            broadcast::Receiver<T>,
        ),
    >,
    max_amount: usize,
    processed_amount: Arc<AtomicUsize>,
}

impl<T: Clone + Unpin + Send + 'static> Stream for SlidingWindowIterator<T> {
    type Item = T;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let processed_amount = self.processed_amount.clone();
        let processed_amount_val = self
            .processed_amount
            .load(std::sync::atomic::Ordering::Relaxed);

        if self.initial_items.len() > processed_amount_val {
            processed_amount.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            return std::task::Poll::Ready(Some(self.initial_items.pop().unwrap()));
        }

        if processed_amount_val >= self.max_amount {
            return std::task::Poll::Ready(None);
        }

        let (result, rx) = ready!(self.inner.poll(cx));
        self.inner.set(crate::make_future_from_rx(rx));

        match result {
            Ok(v) => {
                processed_amount.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                std::task::Poll::Ready(Some(v))
            },
            Err(broadcast::error::RecvError::Lagged(_)) => {
                cx.waker().wake_by_ref();
                std::task::Poll::Pending
            }
            Err(broadcast::error::RecvError::Closed) => std::task::Poll::Ready(None),
        }
    }
}

impl<T: Clone + Unpin + Send + 'static> SlidingWindow<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        SlidingWindow {
            master_sliding_window: VecDeque::with_capacity(capacity),
            capacity,
            transmitter: broadcast::channel(max(capacity, 1)).0,
        }
    }

    pub fn push(&mut self, item: T) {
        self.master_sliding_window.push_back(item.clone());

        if self.master_sliding_window.len() > self.capacity {
            self.master_sliding_window.pop_front();
        }

        if self.transmitter.receiver_count() > 0 {
            let _ = self.transmitter.send(item);
        }
    }

    /// This is not a standard take function. If the amount specified
    /// is more than the capacity of the sliding window, it will not
    /// panic; instead, as items are rotated through the sliding
    /// window, the Stream object will slowly return the relevant
    /// items.
    pub fn take(&self, amount: usize) -> impl Stream<Item = T> {
        SlidingWindowIterator::<T> {
            initial_items: self.master_sliding_window.clone().into_iter().collect(),
            inner: ReusableBoxFuture::new(crate::make_future_from_rx(self.transmitter.subscribe())),
            max_amount: amount,
            processed_amount: Arc::new(AtomicUsize::new(0)),
        }
    }
}
