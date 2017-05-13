extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate rand;
extern crate futures;
extern crate tokio_core;
extern crate telebot;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
mod pocket;
mod errors;
mod config;
use pocket::*;
use config::*;
use futures::*;
use telebot::bot;
use telebot::functions::*;
use tokio_core::reactor::Core;

use std::fs::File;

fn main() {
    let config_file = std::env::args()
        .nth(1)
        .expect("need a config file as argument");

    let json = File::open(config_file).unwrap();
    let config = serde_json::from_reader::<File, Config>(json).expect("cannot parse config.json");
    let pocket = Pocket::new(config.pocket);

    let token = config.telegram.bot_token;
    let chat_id = config.telegram.chat_id;

    let mut lp = Core::new().unwrap();

    let handle = lp.handle();
    let bot = lp.run(bot::RcBot::new(handle, &token)).unwrap();
    let mut msgs = vec![];

    let items = pocket.get_unread().expect("failed to retrive items.");
    let mut rng = rand::thread_rng();
    let sample_items = rand::sample(&mut rng, &items, pocket.article_count());

    for item in sample_items.iter().by_ref() {
        let original_link = match item.resolved_url {
            Some(ref url) => url.clone(),
            None => item.given_url.clone(),
        };
        let pocket_link = format!("https://getpocket.com/a/read/{}", item.id);
        let title = match item.resolved_title {
            Some(ref title) => title.clone(),
            None => item.given_title.clone(),
        };
        let excerpt = match item.excerpt {
            Some(ref excerpt) => excerpt.clone() + "\n",
            None => "".to_owned(),
        };

        msgs.push(bot.message(chat_id,
                              format!("{}\n{}{}\n{}", title, excerpt, original_link, pocket_link)));
    }

    let mut future = futures::future::ok(()).boxed() as
                     Box<Future<Item = (), Error = telebot::Error>>;
    for msg in msgs {
        future = Box::new(future.and_then(|_| msg.send().and_then(|_| Ok(())))) as
                 Box<Future<Item = (), Error = telebot::Error>>
    }
    lp.run(future).unwrap();

    let resp = pocket.archive(&sample_items).unwrap();
    println!("{:?}", resp);
}
