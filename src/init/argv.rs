use std::str;
use std::cmp;

use ::error::*;

use clap::{
  App,
  ArgMatches
};

use std::sync::Arc;


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Argv {
  pub key: String,
  pub secret: String,
  pub flo: String,
  pub channel: String,
  pub interval: u32,
  pub nickname: String,
  pub token: String
}

impl Default for Argv {
  fn default() -> Self {
    Argv {
      key: "".into(),
      secret: "".into(),
      flo: "".into(),
      channel: "".into(),
      nickname: "".into(),
      token: "".into(),
      interval: 3
    }
  }
}


fn merge_string<F>(matches: &ArgMatches, key: &str, mut func: F) -> Result<(), Error>
  where F: FnMut(String)
{
  if let Some(val) = matches.value_of(key) {
    func(val.to_owned());
  }

  Ok(())
}

fn merge_int<F>(matches: &ArgMatches, key: &str, mut func: F) -> Result<(), Error>
  where F: FnMut(u32)
{
  if let Some(val) = matches.value_of(key) {
    func(val.parse::<u32>()?);
  }

  Ok(())
}

pub fn read() -> Result<Arc<Argv>, Error> {
  let yaml = load_yaml!("../../config/cli.yml");
  let matches = App::from_yaml(yaml).get_matches();
  let mut argv = Argv::default();

  merge_string(&matches, "k", |k| { argv.key = k; })?;
  merge_string(&matches, "s", |s| { argv.secret = s; })?;
  merge_string(&matches, "f", |f| { argv.flo = f; })?;
  merge_string(&matches, "c", |c| { argv.channel = c; })?;
  merge_int(&matches, "i", |i| { argv.interval = i; })?;
  merge_string(&matches, "n", |n| { argv.nickname = n; })?;

  if let Some(token) = ::utils::read_token_env() {
    argv.token = token;
  }else{
    merge_string(&matches, "t", |t| { argv.token = t; });
  }

  Ok(Arc::new(argv))
}