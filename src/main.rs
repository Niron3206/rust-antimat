use std::fs::File;
use std::io::{BufReader, BufRead};
use std::{env, io};
use dotenv::dotenv;

use utils::{extract_regular_chars, is_bad};

use serenity::async_trait;
use serenity::model::prelude::{Message, Ready};
use serenity::prelude::Context;
use serenity::{prelude::{GatewayIntents, EventHandler}, Client};

mod utils;

fn read_file(path: &str) -> io::Result<Vec<String>> {
    BufReader::new(File::open(path)?).lines().collect()
}

static mut BAD_WORDS: Vec<String> = vec![];

struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn message(&self, ctx: Context, msg: Message)  {

        let msg_text = extract_regular_chars(msg.content.to_lowercase().as_str());

        for word in msg_text.split(" ").into_iter() {

            let is_bad = unsafe{is_bad(&word, &BAD_WORDS, &msg.author)};

            if is_bad {     
                return msg.delete(&ctx.http).await.expect("Couldn't delete the message.");            
            }
        }

    }

    async fn ready(&self, _: Context, ready: Ready) {
        unsafe {BAD_WORDS = read_file("bad_words.txt").expect("Couldn't read lines from file."); }
        println!("{} — is online", ready.user.tag());
    }

}

#[tokio::main]
async fn main() -> Result<(), serenity::Error>{

    dotenv().ok();

    let token = env::var("token").expect("Didn't find token in `.env` file");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    Client::builder(token, intents).event_handler(Handler).await?.start().await

}

#[cfg(test)]
mod tests {
    use crate::{extract_regular_chars};

    #[test]
    fn extr() {
        let test = extract_regular_chars("t&%@^%(*^e)№$^(st");
        println!("{}", test);
    }
}
