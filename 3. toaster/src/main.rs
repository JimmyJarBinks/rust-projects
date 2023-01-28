use chrono::{Local, Timelike};
use std::time::Duration;
use winrt_notification::{Sound, Toast};

const INTERVAL_TIME: u64 = 30;

enum Time {
    Morning,
    Day,
    Night,
    Late,
}

fn configure_message(time: Time) -> (&'static str, &'static str, &'static str) {
    match time {
        Time::Morning => (
            "Remember to sit up straight",
            "Or better yet, do some stretches.",
            "The day is still young!.",
        ),
        Time::Day => (
            "Remember to sit up straight",
            "Or better yet, take a quick break.",
            "Your back will thank you later.",
        ),
        Time::Night => (
            "Remember to sit up straight",
            "Or better yet, get some shuteye",
            "Bluelight can make it hard to fall asleep",
        ),
        Time::Late => (
            "Ok what are you doing.",
            "Go sleep now.",
            "Please.",
        ),
    }
}

fn create_notification(title: &str, text1: &str, text2: &str) -> Toast {
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title(title)
        .text1(text1)
        .text2(text2)
        .sound(Some(Sound::SMS))
        .duration(winrt_notification::Duration::Short)
}

#[tokio::main]
async fn main() {
    let mut interval = tokio::time::interval(Duration::from_secs(INTERVAL_TIME));
    loop {
        interval.tick().await;

        let date =Local::now();
        let time_of_day = match (date.hour(), date.minute()) {
            (6..=9, ..) => Time::Morning,
            (10..=21, ..) => Time::Day,
            (22 | 23 | 0, ..) => Time::Night,
            (1..=5, ..) => Time::Late,
            _ => Time::Day,
        };

        let toast_text = configure_message(time_of_day);
        let toast = create_notification(toast_text.0, toast_text.1, toast_text.2);
        toast.show().expect("unable to toast");
    }
}
