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
use tokio::select;
use tokio::sync::broadcast::channel;

use log::set_logger;

#[tokio::main]
async fn main() {
    env_logger::init();

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

    let aggregation_handle = perform_aggregation(stream, chat, timedetector, sender);
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
