use winrt_notification::{Duration, Sound, Toast};

fn create_notification(title: &str, text1: &str, text2: &str) -> Toast {
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title(title)
        .text1(text1)
        .text2(text2)
        .sound(Some(Sound::SMS))
        .duration(Duration::Short)
}

fn main() {
    let toast = create_notification(
        "Remember to Sit Up Straight",
        "Or better yet, take a quick break",
        "Your back will thank you later.",
    );
    toast.show().expect("unable to toast");
}
