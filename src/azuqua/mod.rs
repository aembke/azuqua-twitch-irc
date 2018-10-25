

mod utils;

use ::init::argv::Argv;
use ::error::*;

use std::sync::Arc;
use parking_lot::RwLock;

use tokio_core::reactor::Handle;

use ::twitch::channel::HttpsClient;
use ::twitch::channel::TwitchMessage;

use std::collections::VecDeque;


use json;
use json::JsonValue;

use futures::{
  Future,
  Stream
};

#[derive(Clone)]
pub struct Azuqua {
  argv: Arc<Argv>,
  client: HttpsClient
}

impl Azuqua {

  pub fn new(argv: &Arc<Argv>, client: HttpsClient) -> Self {
    Azuqua {
      argv: argv.clone(),
      client
    }
  }

  pub fn invoke(&self, data: VecDeque<TwitchMessage>) -> Box<Future<Item=(), Error=Error>> {
    let mut messages = Vec::with_capacity(data.len());

    for message in data.into_iter() {
      messages.push(message.into_json());
    }
    let messages = JsonValue::Array(messages);

    debug!("Uploading {} messages to Azuqua.", messages.len());

    Box::new(utils::make_invoke_request(&self.client, &self.argv.flo, &self.argv.key, &self.argv.secret, messages).and_then(|response| {
      debug!("Invoke response {}", json::stringify_pretty(response, 2));

      Ok(())
    }))
  }

}