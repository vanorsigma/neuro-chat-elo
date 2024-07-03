//! TimeoutWordDetector only supports i16 audio streams

use std::{future::Future, pin::Pin};

use std::task::{Context, Poll};

pub use rustpotter::SampleFormat;
use rustpotter::{Rustpotter, RustpotterConfig, ScoreMode};
use tokio::{
    stream,
    sync::broadcast::{channel, Receiver, Sender},
};
use tokio_stream::Stream;

const DETECTION_NAME: &str = "timeout";

pub struct TimeoutWordDetectorReceiver(Receiver<f32>);

pub struct TimeoutWordDetector {
    rustpotter: Rustpotter,
    buffer: Vec<u8>,
    sender: Sender<f32>,
    receiver: Receiver<f32>,
    bytes_per_frame: usize,
    chunk_number: usize,
    sample_rate: usize,
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
        config.fmt.sample_format = sample_format.clone();

        let mut rustpotter = Rustpotter::new(&config).unwrap();
        rustpotter
            .add_wakeword_from_file("wakeword_key", wakeword_file_path)
            .unwrap();

        // TODO: use rustpotter.get_bytes_per_frame(), cause it does the same thing
        let spf = rustpotter.get_samples_per_frame()
            * match sample_format {
                SampleFormat::I8 => 1,
                SampleFormat::I16 => 2,
                SampleFormat::I32 => 4,
                SampleFormat::F32 => 4,
            };
        let (sender, receiver) = channel(sample_rate * 5);

        TimeoutWordDetector {
            rustpotter,
            buffer: Vec::with_capacity(spf),
            sender,
            receiver,
            bytes_per_frame: spf,
            chunk_number: 0,
            sample_rate,
        }
    }

    fn actually_consume(&mut self) {
        let result = self.rustpotter.process_bytes(self.buffer.as_slice());
        // println!("{:#?}", self.buffer); TODO: remove
        if let Some(detection) = result {
            if detection.name == DETECTION_NAME {
                // println!("{:#?}", detection.score); // TODO: remove
                self.chunk_number += 1;
                let _ = self.sender.send(self.calculate_time_elapsed_in_seconds());
            }
        }
        self.buffer.clear();
    }

    pub fn ingest_byte(&mut self, b: u8) {
        self.buffer.push(b);
        if self.buffer.len() >= self.bytes_per_frame {
            self.actually_consume()
        }
    }

    /// The f32 is the time elapsed since the start of the stream
    pub fn get_receiver(&self) -> TimeoutWordDetectorReceiver {
        TimeoutWordDetectorReceiver(self.receiver.resubscribe())
    }

    fn calculate_time_elapsed_in_seconds(&self) -> f32 {
        (self.chunk_number * self.rustpotter.get_samples_per_frame()) as f32
            / self.sample_rate as f32
    }
}

impl Stream for TimeoutWordDetectorReceiver {
    type Item = f32;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let recv_future = self.0.recv();
        tokio::pin!(recv_future);
        cx.waker().wake_by_ref(); // TODO: Figure out why this is needed. This shouldn't be needed. wtf.
        recv_future.poll(cx).map(|v| v.ok())
    }
}
