pub mod bits;
pub mod copypastaleader;
pub mod emote;
pub mod emoteuse;
pub mod metrictrait;
pub mod score;
pub mod subs;
pub mod text;

use discord_utils::DiscordClient;
use futures::join;
use log::debug;
use log::warn;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use twitch_utils::seventvclient::SevenTVClient;
use twitch_utils::TwitchAPIWrapper;

use crate::_types::clptypes::Message;
use crate::_types::clptypes::MetricUpdate;
use crate::metrics::metrictrait::AbstractMetric;

struct WithReceiver<M: AbstractMetric> {
    pub metadata: M,
    pub receiver: broadcast::Receiver<(Message, u32)>,
    pub sender: mpsc::Sender<MetricUpdate>,
}

impl<M: AbstractMetric> WithReceiver<M> {
    fn new(
        metadata: M,
        receiver: &broadcast::Receiver<(Message, u32)>,
        sender: &mpsc::Sender<MetricUpdate>,
    ) -> Self {
        Self {
            metadata,
            receiver: receiver.resubscribe(),
            sender: sender.clone(),
        }
    }

    fn get_name(&self) -> String {
        self.metadata.get_name()
    }

    async fn get_metric(&mut self, message: Message, sequence_no: u32) -> MetricUpdate {
        self.metadata.get_metric(message, sequence_no).await
    }

    fn finish(&mut self) -> MetricUpdate {
        self.metadata.finish()
    }
}

pub struct MetricProcessor {
    pub defaults: HashMap<String, f32>,
    bits: WithReceiver<bits::Bits>,
    subs: WithReceiver<subs::Subs>,
    text: WithReceiver<text::Text>,
    copypastaleader: WithReceiver<copypastaleader::CopypastaLeader>,
    emote: WithReceiver<emote::Emote>,
    score: WithReceiver<score::Score>,
    emote_use: WithReceiver<emoteuse::EmoteUse>,
}

impl MetricProcessor {
    /// Create a new MetricProcessor
    /// get_defaults_and_setup_channels must be called before run
    pub fn new(
        seventv_client: Arc<SevenTVClient>,
        twitch_client: Arc<TwitchAPIWrapper>,
        discord_client: Arc<DiscordClient>,
        broadcast_receiver: broadcast::Receiver<(Message, u32)>,
        mpsc_sender: mpsc::Sender<MetricUpdate>,
    ) -> Self {
        let mut defaults: HashMap<String, f32> = HashMap::new();

        let bits = WithReceiver::new(bits::Bits::new(), &broadcast_receiver, &mpsc_sender);
        let subs = WithReceiver::new(subs::Subs::new(), &broadcast_receiver, &mpsc_sender);
        let text = WithReceiver::new(text::Text::new(), &broadcast_receiver, &mpsc_sender);
        let copypastaleader = WithReceiver::new(
            copypastaleader::CopypastaLeader::new(),
            &broadcast_receiver,
            &mpsc_sender,
        );
        let emote = WithReceiver::new(
            emote::Emote::new(seventv_client.clone()),
            &broadcast_receiver,
            &mpsc_sender,
        );
        let score = WithReceiver::new(
            score::Score::new(twitch_client, discord_client),
            &broadcast_receiver,
            &mpsc_sender,
        );
        let emote_use = WithReceiver::new(
            emoteuse::EmoteUse::new(seventv_client),
            &broadcast_receiver,
            &mpsc_sender,
        );

        defaults.insert(bits.get_name(), 0.0);
        defaults.insert(subs.get_name(), 0.0);
        defaults.insert(text.get_name(), 0.0);
        defaults.insert(copypastaleader.get_name(), 0.0);
        defaults.insert(emote.get_name(), 0.0);
        defaults.insert(score.get_name(), 0.0);
        defaults.insert(emote_use.get_name(), 0.0);

        Self {
            defaults,
            bits,
            subs,
            text,
            copypastaleader,
            emote,
            score,
            emote_use,
        }
    }

    pub async fn run(&mut self) {
        join!(
            calc_metric(&mut self.bits),
            calc_metric(&mut self.subs),
            calc_metric(&mut self.text),
            calc_metric(&mut self.copypastaleader),
            calc_metric(&mut self.emote),
            calc_metric(&mut self.emote_use),
        );
        debug!("All metrics finished");
    }
}

async fn calc_metric<M: AbstractMetric + Sync + Send + 'static>(metric: &mut WithReceiver<M>) {
    /*
    Calculate the metric based on chat messages sent by a tokio broadcast channel
    */
    while let Ok((message, sequence_no)) = metric.receiver.recv().await {
        let metric_result = (*metric).get_metric(message, sequence_no).await;
        if let Err(e) = metric.sender.send(metric_result).await {
            warn!("Failed to send metric result: {}", e)
        };
        tokio::task::yield_now().await;
    }
    let metric_result = metric.finish();
    if let Err(e) = metric.sender.send(metric_result).await {
        warn!("Failed to send final metric result: {}", e)
    };
}

#[allow(clippy::type_complexity)]
/// Get the default values for the metrics and set up the channels
pub fn setup_metrics_and_channels(
    seventv_client: Arc<SevenTVClient>,
    twitch_client: Arc<TwitchAPIWrapper>,
    discord_client: Arc<DiscordClient>,
) -> (
    MetricProcessor,
    broadcast::Sender<(Message, u32)>,
    mpsc::Receiver<MetricUpdate>,
) {
    let (broadcast_sender, broadcast_receiver) = broadcast::channel(100000);
    let (mpsc_sender, mpsc_receiver) = mpsc::channel(100000);
    let metric_processor = MetricProcessor::new(
        seventv_client,
        twitch_client,
        discord_client,
        broadcast_receiver,
        mpsc_sender,
    );
    (metric_processor, broadcast_sender, mpsc_receiver)
}
