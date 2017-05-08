#[derive(Deserialize,Debug,Default)]
pub struct PocketConfig {
    pub access_token: String,
    pub consumer_key: String,
    pub article_count: usize,
}

#[derive(Deserialize,Debug,Default)]
pub struct TelegramConfig {
    pub chat_id: i64,
    pub bot_token: String,
}

#[derive(Deserialize,Debug,Default)]
pub struct Config {
    pub telegram: TelegramConfig,
    pub pocket: PocketConfig,
}
