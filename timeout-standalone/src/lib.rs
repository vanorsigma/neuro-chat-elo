pub mod aggregator;
pub mod chat;
pub mod detection;
pub mod stream;

pub use aggregator::*;
pub use chat::*;
pub use detection::*;
pub use stream::*;

use tokio::sync::broadcast;

// Utilities
// If this gets too unwieldy, please refactor
pub(crate) async fn make_future_from_rx<T: Clone>(
    mut rx: broadcast::Receiver<T>,
) -> (
    Result<T, broadcast::error::RecvError>,
    broadcast::Receiver<T>,
) {
    // solution from https://docs.rs/tokio-stream/latest/src/tokio_stream/wrappers/broadcast.rs.html#16-18
    let result = rx.recv().await;
    (result, rx)
}
