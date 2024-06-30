//! TimeoutWordDetector only supports i16 audio streams

pub use rustpotter::SampleFormat;
use rustpotter::{Rustpotter, RustpotterConfig, ScoreMode};
use tokio::sync::broadcast::{Sender, Receiver, channel};

const DETECTION_NAME: &str = "timeout";

pub struct TimeoutWordDetector {
    rustpotter: Rustpotter,
    buffer: Vec<u8>,
    sender: Sender<()>,
    receiver: Receiver<()>
}

impl TimeoutWordDetector {
    pub fn new(
        threshold: f32,
        sample_rate: usize,
        sample_format: SampleFormat,
        wakeword_file_path: &str,
    ) -> Self {
        let mut config = RustpotterConfig::default();
        config.detector.avg_threshold = 0.2;
        config.detector.threshold = threshold;
        config.detector.min_scores = 10;
        config.filters.gain_normalizer.enabled = false;
        config.filters.band_pass.enabled = false;
        config.detector.score_mode = ScoreMode::Max;
        config.fmt.sample_rate = sample_rate;
        config.fmt.sample_format = sample_format;

        let mut rustpotter = Rustpotter::new(&config).unwrap();
        rustpotter
            .add_wakeword_from_file("wakeword_key", wakeword_file_path)
            .unwrap();
        let spf = rustpotter.get_samples_per_frame() * 4;

        let (sender, receiver) = channel(sample_rate * 5);

        TimeoutWordDetector { rustpotter,
                              buffer: Vec::with_capacity(spf),
                              sender, receiver }
    }

    fn actually_consume(&mut self) {
        let result = self.rustpotter.process_bytes(self.buffer.as_slice());
        // println!("{:#?}", self.buffer);
        if let Some(detection) = result {
            if detection.name == DETECTION_NAME {
                println!("{:#?}", detection.score);
                let _ = self.sender.send(());
            }
        }
        self.buffer.clear();
    }

    pub fn ingest_byte(&mut self, b: u8) {
        self.buffer.push(b);
        if self.buffer.len() >= self.rustpotter.get_samples_per_frame() * 4 {
            self.actually_consume()
        }
    }

    pub fn get_receiver(&mut self) -> Receiver<()> {
        self.receiver.resubscribe()
    }
}
