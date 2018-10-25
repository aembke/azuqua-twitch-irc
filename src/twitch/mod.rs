
pub mod channel;
pub mod utils;

use futures::{
  Future,
  Stream
};
use futures::sync::oneshot::{
  Sender as OneshotSender,
  Receiver as OneshotReceiver,
  channel as oneshot_channel
};

use std::sync::Arc;

use irc::client::prelude::*;

use ::error::*;
use ::Dependencies;

use ::irc::client::prelude::*;

pub fn start(dependencies: Arc<Dependencies>) -> Result<(), Error> {
  let channel = dependencies.channel.clone();
  let config = channel.config();

  let mut reactor = match IrcReactor::new() {
    Ok(r) => r,
    Err(e) => panic!("Fatal error creating IRC client for Twitch chat: {:?}", e)
  };
  let client = match reactor.prepare_client_and_connect(&config) {
    Ok(c) => c,
    Err(e) => panic!("Fatal error creating IRC client for Twitch chat: {:?}", e)
  };
  channel.set_client(client.clone());

  if let Err(e) = client.identify() {
    panic!("Fatal error creating IRC client for Twitch chat: {:?}", e);
  }

  trace!("Setting up membership capabilities...");
  if let Err(e) = utils::enable_capabilities(&client) {
    panic!("Fatal error setting up membership features for Twitch chat: {:?}", e);
  }

  info!("Registering Twitch IRC client, listening for messages...");

  let handle = reactor.inner_handle();

  // TODO init http clients
  // init azuqua client
  // create timer future to invoke the flo


  let message_dependencies = dependencies.clone();
  reactor.register_client_with_handler(client.clone(), move |client, message| {
    trace!("Recv IRC message: {:?}", message);

    if let Command::PING(ref s, _) = message.command {
      let resp = Message {
        command: Command::PONG(s.to_owned(), None),
        tags: None,
        prefix: None
      };
      if let Err(e) = client.send(resp) {
        panic!("Fatal error responding to Twitch IRC ping request: {:?}", e);
      }

      return Ok(());
    }

    // TODO buffer message in state

    Ok(())
  });

  if let Err(e) = reactor.run() {
    error!("Error running event loop: {:?}", e);
  }

  Ok(())
}