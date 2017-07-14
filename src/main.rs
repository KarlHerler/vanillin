extern crate discord;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

use discord::Discord;
use discord::model::Event;
use std::env;

mod vanillagaming;

fn vg_result_to_string(res: Option<String>) -> String {
  match res {
    None => "I'm sorry, I couldn't find anything like that".to_string(),
    Some(r) => ("I found this for you: ".to_string() + &r)
  }
}

fn main() {
  println!("Vanillin Discord bot starting");
  // Log in to Discord using a bot token from the environment
  let discord = Discord::from_bot_token(&env::var("DISCORD_TOKEN").expect("Expected token"),).expect("Login to the Discord API failed");

  // Establish and use a websocket connection
  let (mut connection, _) = discord.connect().expect("connect failed");
  println!("Successfully connected to the Discord API.");
  loop {
    match connection.recv_event() {
      Ok(Event::MessageCreate(message)) => {
        let split_message: Vec<&str> = message.content.splitn(2, ' ').collect();
        //println!("{} says: {} or: {:?}", message.author.name, message.content, split_message); // lets not log everything people are saying in the chat
        if split_message[0] == "!vg" || message.content == "!vanillagaming" || message.content == "!vanillin" {
          if split_message.len()>1 {
            let _ = discord.send_message(message.channel_id, &(vg_result_to_string(vanillagaming::find(split_message[1]))), "", false);
          } else {
            let _ = discord.send_message(message.channel_id, &("I'm sorry ".to_string() + &message.author.name + &", I didn't quite understand that"), "", false);
          }
        } else if split_message[0] == "!heywire" {
          println!("Quitting.");
          break
        }
      }
      Ok(_) => {}
      Err(discord::Error::Closed(code, body)) => {
        println!("Discord API Gateway closed on us with code {:?}: {}", code, body);
        break
      }
      Err(err) => println!("Receive error: {:?}", err)
    }
  }
}
