mod twitch;

fn main() {
  let mut client = twitch::Client::new(String::from("vasp"), String::from("oauth"));

  // request authorization
  client.send(twitch::Command::PASS).expect("invalid oauth");
  client.send(twitch::Command::NICK).expect("invalid nick");

  // join a channel
  client
    .send(twitch::Command::JOIN(String::from("vasp")))
    .expect("failed to join channel");

  loop {
    let message: String = client.read();
    println!("{}", message);
  }
}
