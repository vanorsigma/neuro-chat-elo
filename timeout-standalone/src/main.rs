use rustpotter::RustpotterConfig;
use rustpotter::ScoreMode;
use timeout_standalone::chat::Chat;
use timeout_standalone::TimeoutWordDetector;
use tokio::select;
use std::{process::{Command, Stdio}, io::Read, thread::spawn};

#[tokio::main]
async fn main() {
    let chat = Chat::new("vedal987");
    let mut receiver = chat.get_receiver();
    loop {
        select! {
            Ok(message) = receiver.recv() => {
                println!("Message is {:#?}", message);
            }

            _ = tokio::signal::ctrl_c() => {
                println!("Kill received");
                break;
            }
        }
    }
}

fn main3() {
    let mut timedetector = TimeoutWordDetector::new(
        0.7,
        16000,
        rustpotter::SampleFormat::F32,
        "./models/unpolished_evil.rpw"
    );

    let mut recv_channel = timedetector.get_receiver();
    let thread = spawn(move || {
        loop {
            let result = recv_channel.blocking_recv();
            if let Ok(_) = result {
                println!("detected");
            } else {
                break;
            }
        }
    });

    let ffmpeg = Command::new("ffmpeg")
        .arg("-hide_banner")
        .arg("-loglevel")
        .arg("error")
        .arg("-i")
        .arg("evil_trimmed.wav")
        .arg("-ac")
        .arg("1")
        .arg("-ar")
        .arg("16000")
        .arg("-f")
        .arg("f32le")
        .arg("-")
        .stdout(Stdio::piped())
        .spawn()
        .expect("can make ffmpeg process");

    let stdout = ffmpeg.stdout.unwrap();
    stdout.bytes().for_each(|b| {
        timedetector.ingest_byte(b.unwrap())
    });

    let _ = thread.join();
}
