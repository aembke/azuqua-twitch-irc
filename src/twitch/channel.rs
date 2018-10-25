
use futures::{
  Future,
  Stream
};
use futures::sync::mpsc::{
  UnboundedSender,
  UnboundedReceiver,
  unbounded
};
use futures::sync::oneshot::{
  Sender as OneshotSender,
  Receiver as OneshotReceiver,
  channel as oneshot_channel
};

use std::sync::Arc;
use parking_lot::RwLock;

use std::ops::{
  DerefMut,
  Deref
};

use ::Dependencies;
use tokio_core::reactor::Handle;
use super::utils;
use ::utils as bot_utils;

use irc::client::prelude::*;
use irc::proto::message::Tag;
use irc::proto::command::Command;

use ::init::State;
use ::error::*;

use hyper::client::{
  Client,
  HttpConnector
};
use hyper_tls::{
  HttpsConnector,
};
use hyper::{
  Body,
};

use std::time::Duration;

use json;
use json::JsonValue;

pub type HttpsClient = Client<HttpsConnector<HttpConnector>, Body>;

lazy_static! {

  static ref MOD_TAG: Tag = {
    Tag("mod".into(), Some("1".into()))
  };

}

#[derive(Clone, Debug)]
pub struct TwitchMessage {
  pub from: String,
  pub message: String,
  pub moderator: bool,
  pub timestamp: String
}

impl TwitchMessage {

  pub fn from_message(mut message: Message) -> Result<TwitchMessage, Message> {
    let user = match message.prefix {
      Some(ref prefix) => prefix.split("!").next()
        .map(|s| s.to_owned()),
      None => None
    };
    let user = match user {
      Some(u) => u,
      None => return Err(message)
    };

    let body = match message.command {
      Command::PRIVMSG(_, ref mut payload) => payload.to_owned(),
      _ => return Err(message)
    };

    Ok(TwitchMessage {
      from: user,
      message: body,
      moderator: message.tags
        .map(|t| t.contains(&MOD_TAG))
        .unwrap_or(false),
      timestamp: bot_utils::now_utc_iso8601()
    })
  }

  pub fn into_json(self) -> JsonValue {
    object! {
      "from"      => self.from,
      "message"   => self.message,
      "moderator" => self.moderator,
      "timestamp" => self.timestamp
    }
  }

}

#[derive(Clone)]
pub struct Channel {
  config: Arc<RwLock<Config>>,
  room: Arc<String>,
  client: Arc<RwLock<Option<IrcClient>>>
}

impl Channel {

  pub fn new(config: Config) -> Channel {
    let room = if let Some(ref channels) = config.channels {
      match channels.first() {
        Some(s) => s.to_owned(),
        None => panic!("Missing Twitch channel to monitor!")
      }
    }else{
      panic!("Missing Twitch channel to monitor!");
    };

    Channel {
      config: Arc::new(RwLock::new(config)),
      room: Arc::new(room),
      client: Arc::new(RwLock::new(None))
    }
  }

  pub fn config(&self) -> Config {
    let config_guard = self.config.read();
    config_guard.deref().clone()
  }

  pub fn set_client(&self, client: IrcClient) {
    let mut client_guard = self.client.write();
    let mut client_ref = client_guard.deref_mut();
    *client_ref = Some(client);
  }

  pub fn inner(&self) -> Option<IrcClient> {
    let client_guard = self.client.read();
    client_guard.deref().clone()
  }

  pub fn send_message<M: Into<String>>(&self, recipient: Option<String>, message: M) -> Result<(), Error> {
    use irc::client::Client;

    let message = message.into();
    let client = {
      let client_guard = self.client.read();
      let client_ref = client_guard.deref();

      match *client_ref {
        Some(ref c) => c.clone(),
        None => return Err(Error::new(
          ErrorKind::TwitchError, "Twitch IRC client not initialized."
        ))
      }
    };
    let message = if let Some(recipient) = recipient {
      format!("@{} - {}", recipient, message)
    }else{
      message
    };
    let room = self.room.as_ref().to_owned();

    let message = Message {
      command: Command::PRIVMSG(room, message),
      tags: None,
      prefix: None
    };

    client.send(message).map_err(|e| e.into())
  }

  pub fn close(&self) {
    use irc::client::Client;

    let client = {
      let client_guard = self.client.read();
      let client_ref = client_guard.deref();

      match *client_ref {
        Some(ref c) => c.clone(),
        None => {
          error!("Twitch client not initialized!");
          return;
        }
      }
    };
    let room = self.room.as_ref().to_owned();

    let message = Message {
      command: Command::PART(room, None),
      tags: None,
      prefix: None
    };

    if let Err(e) = client.send(message) {
      error!("Error leaving channel: {:?}", e);
    }
  }

}