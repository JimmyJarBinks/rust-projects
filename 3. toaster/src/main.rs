use std::time::Duration;
use chrono::Local;
use winrt_notification::{Sound, Toast};

const INTERVAL_TIME: u64 = 10;

/*
fn configure_message() -> (String, String, String) {
    (String::new(), String::new(), String::new())
}
*/

async fn create_notification(title: &str, text1: &str, text2: &str) -> Toast {
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
        let toast = create_notification(
            "Remember to sit up straight",
            "Or better yet, take a quick break.",
            "Your back will thank you later.",
        );

        let dt = Local::now();
        println!("{}", dt);

        toast.await.show().expect("unable to toast");
    }
}
