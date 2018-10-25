

extern crate chrono;
extern crate ctrlc;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate irc;
extern crate native_tls;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_timer;
extern crate tokio_tls;
extern crate parking_lot;
extern crate crypto;

#[macro_use]
extern crate json;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate clap;

mod error;

#[macro_use]
mod utils;

mod init;
mod twitch;
mod azuqua;

use irc::client::prelude::*;
use twitch::channel::Channel;
use parking_lot::RwLock;
use azuqua::Azuqua;

use std::time::Duration;
use std::process;

use tokio_timer::Timer;

use ::init::State;
use ::init::argv::Argv;

use std::sync::Arc;
use std::ops::Deref;

use ::error::*;

#[doc(hidden)]
#[derive(Clone)]
pub struct Dependencies {
  pub channel: Channel,
  pub timer: Timer,
  pub argv: Arc<Argv>,
  pub state: Arc<RwLock<State>>,
}

fn init(argv: Arc<Argv>) -> Result<(), Error> {
  let twitch_channel = format!("#{}", argv.channel);
  let state = Arc::new(RwLock::new(State::default()));

  let twitch_config = Config {
    nickname: Some(argv.nickname.clone()),
    server: Some("irc.chat.twitch.tv".to_owned()),
    channels: Some(vec![twitch_channel]),
    port: Some(443),
    use_ssl: Some(true),
    password: Some(argv.token.clone()),
    ..Config::default()
  };
  let channel = Channel::new(twitch_config);
  let timer = Timer::default();

  let dependencies = Arc::new(Dependencies {
    channel, timer, state, argv
  });

  let exit_channel = dependencies.channel.clone();
  ctrlc::set_handler(move || {
    println!("Exiting...");
    exit_channel.close();

    process::exit(0);
  })
  .expect("Could not set exit handler.");

  twitch::start(dependencies)
}

fn main() {
  ::pretty_env_logger::init();

  let argv = match init::argv::read() {
    Ok(a) => a,
    Err(e) => panic!("Error reading argv: {:?}", e)
  };

  if let Err(e) = init(argv) {
    println!("Fatal error: {:?}", e);
  }
}














