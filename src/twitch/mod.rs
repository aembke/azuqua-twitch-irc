
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
use ::azuqua::Azuqua;

use std::time::Duration;

use ::irc::client::prelude::*;

use std::ops::{
  Deref,
  DerefMut
};

use futures::future;
use futures::lazy;

use ::twitch::channel::TwitchMessage;

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

  let https_client = utils::create_https_client(&handle, true)
    .expect("Couldn't create https client!");

  let azuqua = Azuqua::new(&dependencies.argv, https_client);
  let dur = Duration::from_secs(dependencies.argv.interval as u64);

  let memo = (azuqua, dependencies.clone());
  let timer_ft = dependencies.timer.interval(dur).from_err::<Error>().fold(memo, |(azuqua, dependencies), _| {
    let messages = dependencies.state.write().deref_mut().take_messages();

    trace!("Checking for saved messages.");
    if messages.is_empty() {
      return ::utils::future_ok((azuqua, dependencies));
    }

    Box::new(azuqua.invoke(messages).then(move |result| {
      if let Err(e) = result {
        error!("Error invoking flo: {:?}", e);
      }

      Ok::<_, Error>((azuqua, dependencies))
    }))
  })
  .map(|_| ())
  .map_err(|_| ());

  handle.spawn(lazy(move || {
    timer_ft
  }));

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

    let twitch_messages = match TwitchMessage::from_message(message) {
      Ok(m) => m,
      Err(original) => {
        warn!("Couldnt convert message: {:?}", original);
        return Ok(());
      }
    };

    dependencies.state.write().deref_mut().add_message(twitch_messages);

    Ok(())
  });

  if let Err(e) = reactor.run() {
    error!("Error running event loop: {:?}", e);
  }

  Ok(())
}