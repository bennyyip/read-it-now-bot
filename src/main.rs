extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate rand;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

mod errors;

use std::collections::HashMap;

use errors::*;

#[derive(Deserialize,Debug,Default)]
pub struct Config {
    access_token: String,
    consumer_key: String,
    article_count: u32,
}

pub struct Pocket {
    config: Config,
    client: reqwest::Client,
}

#[derive(Serialize,Deserialize,Debug,Default,Clone)]
pub struct PocketItem {
    #[serde(rename="item_id")]
    id: String,
    given_url: String,
    given_title: String,
    resolved_url: Option<String>,
    resolved_title: Option<String>,
    excerpt: Option<String>,
    word_count: String,
}

#[derive(Deserialize,Debug,Default)]
pub struct PocketResponse {
    status: u32,
    complete: u32,
    list: HashMap<String, PocketItem>,
    error: Option<u32>,
}

#[derive(Deserialize,Serialize,Debug,Default)]
pub struct PocketModifyRequest {
    consumer_key: String,
    access_token: String,
    actions: Vec<Action>,
}

#[derive(Deserialize,Serialize,Debug,Default)]
pub struct Action {
    item_id: String,
    action: String,
}


static GET_API: &'static str = "https://getpocket.com/v3/get";
static SEND_API: &'static str = "https://getpocket.com/v3/send";

impl Pocket {
    pub fn new(config: Config) -> Self {
        let client = reqwest::Client::new().expect("cannot create client");
        Self {
            client: client,
            config: config,
        }
    }

    pub fn get_unread(&self) -> Result<Vec<PocketItem>> {
        let mut params = HashMap::new();
        params.insert("consumer_key", self.config.consumer_key.clone());
        params.insert("access_token", self.config.access_token.clone());
        params.insert("content_type", "article".to_owned());
        params.insert("detailed_type", "simple".to_owned());
        let response = self.client
            .post(GET_API)
            .form(&params)
            .send()
            .chain_err(|| "failed to retrive articles")?;

        Ok(serde_json::from_reader::<reqwest::Response, PocketResponse>(response)
               .chain_err(|| "failed to parse retrived data")?
               .list
               .into_iter()
               .map(|(_, value)| value)
               .collect())
    }

    pub fn archive(&self, items: &[&PocketItem]) -> Result<reqwest::Response> {
        let actions: Vec<_> = items
            .iter()
            .map(|item| {
                     Action {
                         item_id: item.id.to_owned(),
                         action: "archive".to_owned(),
                     }
                 })
            .collect();
        let params = PocketModifyRequest {
            consumer_key: self.config.consumer_key.clone(),
            access_token: self.config.access_token.clone(),
            actions: actions,
        };
        self.client
            .post(SEND_API)
            .json(&params)
            .send()
            .chain_err(|| "failed to archive items")
    }
}

fn main() {
    let json = include_str!("../config.json");
    let config = serde_json::from_str::<Config>(json).expect("cannot parse config.json");

    let pocket = Pocket::new(config);
    let items = pocket.get_unread().expect("failed to retrive items.");

    let mut rng = rand::thread_rng();
    let sample_items = rand::sample(&mut rng, &items, 2);
    for item in sample_items.iter().by_ref() {
        println!("{}", serde_json::to_string_pretty(item).unwrap());
    }
    let resp = pocket.archive(&sample_items).unwrap();
    println!("{:?}", resp);
}
