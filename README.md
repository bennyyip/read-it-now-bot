# Read it Later Bot

Pick random articles in your [Pocket](https://www.getpocket.com) list, and send it to you via a [telegram](https://t.me) bot.  
You may use crontab or systemd-timer to schedule this.


## Build

Install  *Rust Nightly* and *Cargo* (use [`rustup`](https://www.rustup.rs/)), then:

```
cargo build --release
```

The compiled binary will be in: `./target/release/rin`

## Config 

```
// config.example.json
{
  "pocket": {
    "access_token": "<access token>",
    "consumer_key": "<consumer key>",
    "article_count": 2
  },

  "telegram": {
    "bot_token": "<bot token>",
    "chat_id": <chat_id>
  }
}
```

Rename `config.example.json` to `config.json` and fill it.
- `consumer_key`: The Pocket API key, and you can apply it [here](https://getpocket.com/developer/apps/new)
- `access_token`: Token used to access your pocket account. Use `./retrieve_token <consumer_key>` to retrieve it.
- `bot token`: Ask [Bot Father](https://t.me/botfather) to create a new bot. He will give you back the bot token.
- `chat_id`: Ask [get id bot](https://t.me/get_id_bot) for it.

## Run

```
./target/release/rin config.json
```

## Schedule
See [Systemd Timer Wizard](https://github.com/fiveyellowmice/systemd-timer-wizard)

## License

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or distribute this software, either in source code form or as a compiled binary, for any purpose, commercial or non-commercial, and by any means.
