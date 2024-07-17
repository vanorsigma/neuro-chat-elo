///! A sliding window that supports "future" taking. So, something like:
///! take 20 items from a SlidingWindow of 10.
///! Calling the releavant functions on SlidingWindow will produce
///! thread-safe consumers.
use std::{
    cmp::{max, min}, collections::VecDeque, future::Future, sync::{atomic::AtomicUsize, Arc}
};

use tokio::sync::broadcast;
use tokio_stream::Stream;

#[derive(Debug)]
pub(crate) struct SlidingWindow<T> {
    master_sliding_window: VecDeque<T>,
    capacity: usize,
    internal_counter: usize,
    transmitter: broadcast::Sender<T>,
}

struct SlidingWindowIterator<T> {
    initial_items: Vec<T>,
    receiver: broadcast::Receiver<T>,
    max_amount: usize,
    processed_amount: Arc<AtomicUsize>,
}

impl<T: Clone + Unpin> Stream for SlidingWindowIterator<T> {
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

        // TODO: like literally why is this needed
        cx.waker().wake_by_ref();
        let func = self.receiver.recv();
        tokio::pin!(func);
        func.poll(cx).map(|result| {
            if result.is_ok() {
                processed_amount.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
            result.ok()
        })
    }
}

impl<T: Clone + Unpin> SlidingWindow<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        SlidingWindow {
            master_sliding_window: VecDeque::with_capacity(capacity),
            capacity,
            internal_counter: 0,
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
            receiver: self.transmitter.subscribe(),
            max_amount: amount,
            processed_amount: Arc::new(AtomicUsize::new(0)),
        }
    }
}
