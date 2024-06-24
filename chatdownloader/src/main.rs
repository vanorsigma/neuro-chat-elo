mod _types;
mod metrics;
mod twitchdownloaderproxy;
mod twitch_utils;

fn main() {
    twitchdownloaderproxy::test().unwrap();
}
