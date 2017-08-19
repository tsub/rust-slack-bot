extern crate slack;

use self::slack::RtmClient;

pub fn respond_hi(bot_id: &str, text: &str, channel: &str, cli: &RtmClient) {
    let pattern = format!("<@{}> {}", bot_id, "hi");

    if text.contains(&pattern) {
        let _ = cli.sender().send_message(channel, "Hi!");
    }
}
