extern crate slack;

use slack::{Event, Message, RtmClient};
use std::{env, process};

struct MyHandler;

#[allow(unused_variables)]
impl slack::EventHandler for MyHandler {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        println!("on_event(event: {:?})", event);

        let message = match event {
            Event::Message(message) => message,
            _ => return
        };
        let message_standard = match *message {
            Message::Standard(message_standard) => message_standard,
            _ => return
        };

        let bot_id = cli.start_response().slf.as_ref().unwrap().id.as_ref().unwrap();
        if &message_standard.user.unwrap() == bot_id {
            println!("Is own message");
            return
        }

        let text: String = message_standard.text.unwrap();
        if !text.contains(bot_id) {
            println!("Is not a mention");
            return
        }

        let _ = cli.sender().send_message(&message_standard.channel.unwrap(), "Hi!");
    }

    fn on_close(&mut self, cli: &RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, cli: &RtmClient) {
        println!("on_connect");
    }
}

fn main() {
    let api_key = match env::var("SLACK_API_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            println!("Required the SLACK_API_TOKEN environment variable");
            process::exit(1);
        }
    };
    let mut handler = MyHandler;
    let r = RtmClient::login_and_run(&api_key, &mut handler);

    match r {
        Ok(_) => {},
        Err(err) => panic!("Error: {}", err)
    }
}
