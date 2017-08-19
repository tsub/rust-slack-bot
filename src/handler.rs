extern crate slack;

use action;
use self::slack::{Event, EventHandler, Message, RtmClient};

pub struct Handler;

#[allow(unused_variables)]
impl EventHandler for Handler {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        println!("on_event(event: {:?})", event);

        match event.clone() {
            Event::Message(message) => self.handle_message(*message, cli, &event),
            _ => return
        };
    }

    fn on_close(&mut self, cli: &RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, cli: &RtmClient) {
        println!("on_connect");
    }
}

#[allow(unused_variables)]
impl Handler {
    fn handle_message(&mut self, message: Message, cli: &RtmClient, event: &Event) {
        let message_standard = match message {
            Message::Standard(message_standard) => message_standard,
            _ => return
        };
        let channel: String = message_standard.channel.unwrap();

        let bot_id: &str = cli.start_response().slf.as_ref().unwrap().id.as_ref().unwrap();
        if &message_standard.user.unwrap() == bot_id {
            println!("Is own message");
            return
        }

        let text: String = message_standard.text.unwrap();
        if !text.contains(bot_id) {
            println!("Is not a mention");
            return
        }

        action::respond_hi(&bot_id, &text, &channel, &cli);
    }
}
