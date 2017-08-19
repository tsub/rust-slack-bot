extern crate rust_slack_bot;
extern crate slack;

use rust_slack_bot::handler;
use slack::RtmClient;
use std::{env, process};

fn main() {
    let api_key = match env::var("SLACK_API_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            println!("Required the SLACK_API_TOKEN environment variable");
            process::exit(1);
        }
    };
    let mut handler = handler::Handler;
    let r = RtmClient::login_and_run(&api_key, &mut handler);

    match r {
        Ok(_) => {},
        Err(err) => panic!("Error: {}", err)
    }
}
