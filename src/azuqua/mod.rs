

mod utils;

use ::init::argv::Argv;
use ::error::*;

use std::sync::Arc;
use parking_lot::RwLock;

use tokio_core::reactor::Handle;

use ::twitch::channel::HttpsClient;
use ::twitch::channel::TwitchMessage;

use std::collections::VecDeque;

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

    unimplemented!()
  }

}