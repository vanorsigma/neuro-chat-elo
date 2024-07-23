use clap::arg;
use clap::command;
use clap::Parser;
use log::info;
use rustpotter::RustpotterConfig;
use rustpotter::ScoreMode;
use std::{
    io::Read,
    process::{Command, Stdio},
    thread::spawn,
};
use timeout_standalone::aggregator::perform_aggregation;
use timeout_standalone::chat::Chat;
use timeout_standalone::livestream::TwitchLiveStream;
use timeout_standalone::local::Local;
use timeout_standalone::vod::TwitchVOD;
use timeout_standalone::FFMPEGDecorator;
use timeout_standalone::TimeoutWordDetector;
use timeout_standalone::TrainingDataSettings;
use tokio::select;
use tokio::sync::broadcast::channel;

use log::set_logger;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        default_value_t = 30,
        help = "If the newest item in either chat / wakeword detection queue is this duration away from the oldest item in the queue, purge it from the queue unconditionally"
    )]
    unconditional_purge_duration: u32,

    #[arg(
        long,
        default_value_t = false,
        help = "Whether to output training data"
    )]
    output_training_data: bool,

    #[arg(long, default_value_t = 1.2, help = "Duration of audio clip to dump")]
    output_training_duration: f32,

    #[arg(
        long,
        default_value = "./training_files",
        help = "Output location of audio clips and supporting JSON files"
    )]
    output_training_path: String,
}

#[tokio::main]
async fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let args = Args::parse();

    let mut chat = Chat::new("vedal987");
    let mut timedetector = TimeoutWordDetector::new(
        0.7,
        16000,
        rustpotter::SampleFormat::F32,
        "./models/unpolished_neuro.rpw",
    );
    let mut stream = FFMPEGDecorator::wrap_around(TwitchLiveStream::new("vedal987", None));
    // let mut stream = FFMPEGDecorator::wrap_around(TwitchVOD::new("2182332760"));
    // let mut stream = FFMPEGDecorator::wrap_around(TwitchVOD::new("2188296968"));
    /*let mut stream = FFMPEGDecorator::wrap_around(Local::new("./evil_trimmed.wav"));*/
    let (sender, mut receiver) = channel(1000);

    // TODO: It should be possible to do this without spawning a blocking thread,
    // but that requires some trial and error and I'm not willing to do that at the moment
    let aggregation_handle = perform_aggregation(
        stream,
        chat,
        timedetector,
        sender,
        TrainingDataSettings {
            training_data_output_enabled: args.output_training_data,
            duration_per_clip_in_seconds: args.output_training_duration,
            directory: args.output_training_path.to_string(),
        },
        args.unconditional_purge_duration
    );
    tokio::task::spawn_blocking(move || {
        while let Ok(data) = receiver.blocking_recv() {
            info!("FULL DETECTION: {:#?}", data);
        }
    });

    aggregation_handle.await
}

// #[tokio::main]
// async fn main4() {
//     let chat = Chat::new("vedal987");
//     let mut receiver = chat.get_receiver();
//     loop {
//         select! {
//             Ok(message) = receiver.recv() => {
//                 println!("Message is {:#?}", message);
//             }

//             _ = tokio::signal::ctrl_c() => {
//                 println!("Kill received");
//                 break;
//             }
//         }
//     }
// }

// fn main3() {
//     let mut timedetector = TimeoutWordDetector::new(
//         0.7,
//         16000,
//         rustpotter::SampleFormat::F32,
//         "./models/unpolished_evil.rpw"
//     );

//     let mut recv_channel = timedetector.get_receiver();
//     let thread = spawn(move || {
//         loop {
//             let result = recv_channel.blocking_recv();
//             if let Ok(_) = result {
//                 println!("detected");
//             } else {
//                 break;
//             }
//         }
//     });

//     let ffmpeg = Command::new("ffmpeg")
//         .arg("-hide_banner")
//         .arg("-loglevel")
//         .arg("error")
//         .arg("-i")
//         .arg("evil_trimmed.wav")
//         .arg("-ac")
//         .arg("1")
//         .arg("-ar")
//         .arg("16000")
//         .arg("-f")
//         .arg("f32le")
//         .arg("-")
//         .stdout(Stdio::piped())
//         .spawn()
//         .expect("can make ffmpeg process");

//     let stdout = ffmpeg.stdout.unwrap();
//     stdout.bytes().for_each(|b| {
//         timedetector.ingest_byte(b.unwrap())
//     });

//     let _ = thread.join();
// }
