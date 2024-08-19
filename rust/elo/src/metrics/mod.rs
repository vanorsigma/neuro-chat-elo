pub mod bits;
pub mod copypastaleader;
pub mod emote;
pub mod metrictrait;
pub mod sentiment;
pub mod subs;
pub mod text;

use futures::join;
use log::debug;
use log::warn;
use std::collections::HashMap;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use crate::_types::clptypes::Message;
use crate::_types::clptypes::MetricUpdate;
use crate::metrics::metrictrait::AbstractMetric;

pub struct MetricProcessor {
    pub defaults: HashMap<String, f32>,
    broadcast_receiver: broadcast::Receiver<(Message, u32)>,
    mpsc_sender: mpsc::Sender<MetricUpdate>,
    bits: bits::Bits,
    subs: subs::Subs,
    text: text::Text,
    copypastaleader: copypastaleader::CopypastaLeader,
    emote: emote::Emote,
    sentiment: sentiment::Sentiment,
}

impl MetricProcessor {
    /// Create a new MetricProcessor
    /// get_defaults_and_setup_channels must be called before run
    pub async fn new(
        broadcast_receiver: broadcast::Receiver<(Message, u32)>,
        mpsc_sender: mpsc::Sender<MetricUpdate>,
    ) -> Self {
        let mut defaults: HashMap<String, f32> = HashMap::new();

        let bits = bits::Bits::new().await;
        let subs = subs::Subs::new().await;
        let text = text::Text::new().await;
        let copypastaleader = copypastaleader::CopypastaLeader::new().await;
        let emote = emote::Emote::new().await;
        let sentiment = sentiment::Sentiment::new().await;

        defaults.insert(bits.get_name(), 0.0);
        defaults.insert(subs.get_name(), 0.0);
        defaults.insert(text.get_name(), 0.0);
        defaults.insert(copypastaleader.get_name(), 0.0);
        defaults.insert(emote.get_name(), 0.0);
        defaults.insert(sentiment.get_name(), 0.0);

        Self {
            defaults,
            broadcast_receiver,
            mpsc_sender,
            bits,
            subs,
            text,
            copypastaleader,
            emote,
            sentiment,
        }
    }

    pub async fn run(&mut self) {
        join!(
            calc_metric(
                &mut self.bits,
                self.mpsc_sender.clone(),
                self.broadcast_receiver.resubscribe(),
            ),
            calc_metric(
                &mut self.subs,
                self.mpsc_sender.clone(),
                self.broadcast_receiver.resubscribe(),
            ),
            calc_metric(
                &mut self.text,
                self.mpsc_sender.clone(),
                self.broadcast_receiver.resubscribe(),
            ),
            calc_metric(
                &mut self.copypastaleader,
                self.mpsc_sender.clone(),
                self.broadcast_receiver.resubscribe(),
            ),
            calc_metric(
                &mut self.emote,
                self.mpsc_sender.clone(),
                self.broadcast_receiver.resubscribe(),
            ),
            calc_metric(
                &mut self.sentiment,
                self.mpsc_sender.clone(),
                self.broadcast_receiver.resubscribe(),
            ),
        );
        debug!("All metrics finished");
    }
}

async fn calc_metric<M: AbstractMetric + Sync + Send + 'static>(
    metric: &mut M,
    sender: mpsc::Sender<MetricUpdate>,
    mut reciever: broadcast::Receiver<(Message, u32)>,
) {
    /*
    Calculate the metric based on chat messages sent by a tokio broadcast channel
    */
    loop {
        if let Ok((message, sequence_no)) = reciever.recv().await {
            let metric_result = (*metric).get_metric(message, sequence_no);
            if let Err(e) = sender.send(metric_result).await {
                warn!("Failed to send metric result: {}", e)
            };
            tokio::task::yield_now().await;
        } else {
            break;
        };
    }
    let metric_result = metric.finish();
    if let Err(e) = sender.send(metric_result).await {
        warn!("Failed to send final metric result: {}", e)
    };
}

#[allow(clippy::type_complexity)]
/// Get the default values for the metrics and set up the channels
pub async fn setup_metrics_and_channels() -> (
    MetricProcessor,
    broadcast::Sender<(Message, u32)>,
    mpsc::Receiver<MetricUpdate>,
) {
    let (broadcast_sender, broadcast_receiver) = broadcast::channel(100000);
    let (mpsc_sender, mpsc_receiver) = mpsc::channel(100000);
    let metric_processor = MetricProcessor::new(broadcast_receiver, mpsc_sender).await;
    (metric_processor, broadcast_sender, mpsc_receiver)
}
