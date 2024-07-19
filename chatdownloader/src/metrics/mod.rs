pub mod bits;
pub mod copypastaleader;
pub mod emote;
pub mod metrictrait;
pub mod subs;
pub mod text;

use futures::join;
use log::debug;
use log::warn;
use std::collections::HashMap;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use crate::_types::twitchtypes::Comment;
use crate::metrics::metrictrait::AbstractMetric;

pub struct MetricProcessor {
    metric_defaults: HashMap<String, f32>,
    broadcast_receiver: Option<broadcast::Receiver<(Comment, u32)>>,
    mpsc_sender: Option<mpsc::Sender<(String, HashMap<String, f32>)>>,
    bits: bits::Bits,
    subs: subs::Subs,
    text: text::Text,
    copypastaleader: copypastaleader::CopypastaLeader,
    emote: emote::Emote,
}

impl MetricProcessor {
    /// Create a new MetricProcessor
    /// get_defaults_and_setup_channels must be called before run
    pub async fn new() -> Self {
        let mut metric_defaults: HashMap<String, f32> = HashMap::new();

        let bits = bits::Bits::new().await;
        let subs = subs::Subs::new().await;
        let text = text::Text::new().await;
        let copypastaleader = copypastaleader::CopypastaLeader::new().await;
        let emote = emote::Emote::new().await;

        metric_defaults.insert(bits.get_name(), 0.0);
        metric_defaults.insert(subs.get_name(), 0.0);
        metric_defaults.insert(text.get_name(), 0.0);
        metric_defaults.insert(copypastaleader.get_name(), 0.0);
        metric_defaults.insert(emote.get_name(), 0.0);

        Self {
            metric_defaults,
            broadcast_receiver: None,
            mpsc_sender: None,
            bits,
            subs,
            text,
            copypastaleader,
            emote,
        }
    }

    #[allow(clippy::type_complexity)]
    /// Get the default values for the metrics and set up the channels
    /// This must be called before run
    pub fn get_defaults_and_setup_channels(
        &mut self,
    ) -> (
        HashMap<String, f32>,
        broadcast::Sender<(Comment, u32)>,
        mpsc::Receiver<(String, HashMap<String, f32>)>,
    ) {
        let (broadcast_sender, broadcast_receiver) = broadcast::channel(100000);
        let (mpsc_sender, mpsc_receiver) = mpsc::channel(100000);
        self.broadcast_receiver.replace(broadcast_receiver);
        self.mpsc_sender.replace(mpsc_sender);
        (
            self.metric_defaults.clone(),
            broadcast_sender,
            mpsc_receiver,
        )
    }

    pub async fn run(&mut self) {
        let mpsc_sender = match &self.mpsc_sender {
            Some(sender) => sender,
            None => {
                panic!("No mpsc sender found, did you run get_defaults_and_setup_channels?");
            }
        };
        let broadcast_receiver = match &self.broadcast_receiver {
            Some(receiver) => receiver,
            None => {
                panic!("No broadcast receiver found, did you run get_defaults_and_setup_channels?");
            }
        };
        join!(
            calc_metric(
                &mut self.bits,
                mpsc_sender.clone(),
                broadcast_receiver.resubscribe(),
            ),
            calc_metric(
                &mut self.subs,
                mpsc_sender.clone(),
                broadcast_receiver.resubscribe(),
            ),
            calc_metric(
                &mut self.text,
                mpsc_sender.clone(),
                broadcast_receiver.resubscribe(),
            ),
            calc_metric(
                &mut self.copypastaleader,
                mpsc_sender.clone(),
                broadcast_receiver.resubscribe(),
            ),
            calc_metric(
                &mut self.emote,
                mpsc_sender.clone(),
                broadcast_receiver.resubscribe(),
            ),
        );
        debug!("All metrics finished");
        drop(self.mpsc_sender.take());
    }
}

async fn calc_metric<M: AbstractMetric + Sync + Send + 'static>(
    metric: &mut M,
    sender: mpsc::Sender<(String, HashMap<String, f32>)>,
    mut reciever: broadcast::Receiver<(Comment, u32)>,
) {
    /*
    Calculate the metric based on chat messages sent by a tokio broadcast channel
    */
    loop {
        let (comment, sequence_no) = match reciever.recv().await {
            Ok((comment, sequence_no)) => (comment, sequence_no),
            Err(_) => break,
        };
        let metric_result = (*metric).get_metric(comment, sequence_no);
        if let Err(e) = sender.send(metric_result).await {
            warn!("Failed to send metric result: {}", e)
        };
        tokio::task::yield_now().await;
    }
    let metric_result = metric.finish();
    if let Err(e) = sender.send(metric_result).await {
        warn!("Failed to send final metric result: {}", e)
    };
}
