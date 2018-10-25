
use ::utils;
use ::twitch::channel::TwitchMessage;

use std::collections::{
  VecDeque
};

use json;
use json::JsonValue;

use std::mem;

pub mod argv;

/// All the internal state to cache in memory and store on disk.
pub struct State {
  /// Whether or not the bot is responding to messages.
  pub running: bool,
  /// A running buffer of messages to be sent.
  pub messages: VecDeque<TwitchMessage>
}

impl Default for State {
  fn default() -> Self {
    State {
      running: true,
      messages: VecDeque::new()
    }
  }
}

impl State {

  pub fn take_messages(&mut self) -> VecDeque<TwitchMessage> {
    mem::replace(&mut self.messages, VecDeque::new())
  }

  pub fn add_message(&mut self, message: TwitchMessage) {
    self.messages.push_back(message)
  }

}

