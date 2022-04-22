use std::fs;
use tungstenite::{connect, Message};
use url::Url;

const TWITCH_IRC_ENDPOINT: &str = "ws://irc-ws.chat.twitch.tv:80";

struct Authorization {
  nickname: String,
  oauth: String,
}

fn build_auth_from_file() -> Result<Authorization, &'static str> {
  let file = fs::read_to_string("auth.txt").expect("Failed to read auth.txt");
  let contents: Vec<&str> = file.lines().collect();

  return match contents.len() {
    2 => Ok(Authorization {
      nickname: String::from(contents[0]),
      oauth: String::from(contents[1]),
    }),
    _ => Err("invalid auth.txt configuration"),
  };
}

fn parse_response(content: &str) -> Option<String> {
  if content.starts_with("!ping") {
    return Some(String::from("Pong! 🏓"));
  }

  return None;
}

fn main() {
  let auth = build_auth_from_file().expect("failed to load");
  let url = Url::parse(TWITCH_IRC_ENDPOINT);
  let (mut socket, _) = connect(&url.unwrap()).expect("Can't connect");

  let Authorization { nickname, oauth } = auth;

  socket
    .write_message(Message::Text(format!("PASS {oauth}").into()))
    .unwrap();

  socket
    .write_message(Message::Text(format!("NICK {nickname}").into()))
    .unwrap();

  socket
    .write_message(Message::Text(format!("JOIN #{nickname}").into()))
    .unwrap();

  loop {
    let socket_message: Message = socket.read_message().expect("Error reading message");
    let messages: String = socket_message.into_text().unwrap();

    for message in messages.lines() {
      let contents: Vec<&str> = message.split(" :").collect();

      let res: Option<String> = match contents.len() {
        2 => parse_response(contents[1]),
        _ => None,
      };

      if res.is_none() {
        continue;
      };

      let response_content: String = format!("PRIVMSG #{} :{}", nickname, res.unwrap());
      socket
        .write_message(Message::Text(response_content))
        .unwrap()
    }
  }
}
