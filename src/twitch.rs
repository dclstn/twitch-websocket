use std::net::TcpStream;
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use url::Url;

const TWITCH_IRC_ENDPOINT: &str = "ws://irc-ws.chat.twitch.tv:80";

pub struct Client {
  pub nick: String,
  pub oauth: String,
  websocket: WebSocket<MaybeTlsStream<TcpStream>>,
}

pub enum Command {
  NICK,
  PASS,
  JOIN(String),
  PART(String),
  PRIVMSG(String),
}

impl Client {
  pub fn new(nick: String, oauth: String) -> Self {
    let url = Url::parse(TWITCH_IRC_ENDPOINT);
    let (socket, _) = connect(&url.unwrap()).expect("failed to connect to irc");

    return Self {
      nick,
      oauth,
      websocket: socket,
    };
  }

  pub fn read(&mut self) -> String {
    self
      .websocket
      .read_message()
      .expect("failed to irc message")
      .into_text()
      .unwrap()
  }

  pub fn send(&mut self, command: Command) -> Result<(), tungstenite::Error> {
    match command {
      // bot username
      Command::NICK => self
        .websocket
        .write_message(Message::Text(format!("NICK {}", self.nick))),
      // bot oauth token
      Command::PASS => self
        .websocket
        .write_message(Message::Text(format!("PASS {}", self.oauth))),
      // join specified channel
      Command::JOIN(channel) => self
        .websocket
        .write_message(Message::Text(format!("JOIN #{}", channel))),
      // part specified channel
      Command::PART(channel) => self
        .websocket
        .write_message(Message::Text(format!("PART {}", channel))),
      // send message to chat
      Command::PRIVMSG(content) => self
        .websocket
        .write_message(Message::Text(format!("PRIVMSG {}", content))),
    }
  }
}
