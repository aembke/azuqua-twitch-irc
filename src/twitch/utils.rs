
use futures::{
  future,
  Future,
  Stream
};

use std::str::FromStr;
use std::str::from_utf8;

use std::sync::Arc;
use parking_lot::RwLock;
use tokio_core::reactor::Handle;
use std::collections::BTreeMap;

use std::ops::{
  Deref,
  DerefMut
};

use irc::client::prelude::*;
use irc::proto::command::Command;
use irc::proto::command::CapSubCommand;

use hyper::client::{
  Client,
  HttpConnector
};
use hyper_tls::{
  HttpsConnector,
};
use hyper::{
  Uri,
  Method,
  Request,
  Response,
  Body,
  Chunk,
  StatusCode
};
use hyper::header::{
  Header,
  Headers,
  ContentType,
  ContentLength
};

use std::time::Duration;

use ::twitch::channel::HttpsClient;
use ::error::*;
use ::Dependencies;
use ::init::State;
use ::utils as bot_utils;

use json;
use json::JsonValue;

use tokio_timer::Timer;
use std::mem;

fn chunks_to_str(chunk: &Chunk) -> Result<&str, Error> {
  from_utf8(chunk).map_err(|e| e.into())
}

pub fn create_https_client(handle: &Handle, keep_alive: bool) -> Result<HttpsClient, Error> {
  let connector = match HttpsConnector::new(1, &handle) {
    Ok(c) => c,
    Err(e) => return Err(Error::new(
      ErrorKind::TwitchError, format!("{:?}", e)
    ))
  };

  Ok(Client::configure()
    .keep_alive(keep_alive)
    .keep_alive_timeout(Some(Duration::from_secs(90)))
    .connector(connector)
    .build(handle))
}


pub fn enable_capabilities(client: &IrcClient) -> Result<(), Error> {
  use irc::client::Client;

  let message = Message {
    tags: None,
    prefix: None,
    command: Command::CAP(None, CapSubCommand::REQ, Some(":twitch.tv/tags".into()), None)
  };
  let _ = client.send(message)?;

  Ok(())
}
