use ::error::*;

use json;
use json::JsonValue;

use std::fs::File;
use std::io::Read;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Twitch {
  pub username: String,
  pub token: String,
  pub channel: String
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Settings {
  pub twitch: Twitch,
  pub workers: u16,
  pub db: String
}

fn missing_field(group: &'static str, key: &'static str) -> Error {
  Error::new(ErrorKind::Config, format!("Missing or invalid `{}` field in `{}` settings.", key, group))
}

fn config_error<T, S: Into<String>>(message: S) -> Result<T, Error> {
  Err(Error::new(ErrorKind::Config,message))
}

fn read_twitch(data: &JsonValue) -> Result<Twitch, Error> {
  let username = match data["twitch"]["username"].as_str() {
    Some(s) => s.to_owned(),
    None => return Err(missing_field("twitch", "username"))
  };
  let token = match data["twitch"]["token"].as_str() {
    Some(s) => s.to_owned(),
    None => return Err(missing_field("twitch", "token"))
  };
  let channel = match data["twitch"]["channel"].as_str() {
    Some(s) => s.to_owned(),
    None => return Err(missing_field("twitch", "channel"))
  };

  Ok(Twitch {
    username,
    token,
    channel
  })
}

pub fn read_from_path(path: &str) -> Result<Settings, Error> {
  let mut file = File::open(path)?;
  let mut payload = String::new();
  file.read_to_string(&mut payload)?;

  let payload = match json::parse(&payload) {
    Ok(p) => p,
    Err(e) => return config_error("Error reading configuration settings.")
  };
  let workers = match payload["workers"].as_u16() {
    Some(w) => w,
    None => return config_error("Error reading workers from settings.")
  };
  let db = match payload["db"].as_str() {
    Some(d) => d.to_owned(),
    None => return config_error("Error reading database file path from settings.")
  };
  let twitch = read_twitch(&payload)?;

  Ok(Settings {
    workers, db, twitch
  })
}