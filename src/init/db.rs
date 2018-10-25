
use ::error::*;
use ::init::State;

use json;
use json::JsonValue;

use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind as IoErrorKind;


pub fn read_state(path: &str) -> Result<State, Error> {
  let mut file = match File::open(path) {
    Ok(f) => f,
    Err(e) => return match e.kind() {
      IoErrorKind::NotFound => Ok(State::default()),
      _ => Err(e.into())
    }
  };
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(State::from(json::parse(&contents)?))
}

pub fn write_state(path: &str, state: &State) -> Result<(), Error> {
  let obj = json::stringify(state.to_json());
  let mut file = File::create(path)?;
  file.write_all(obj.as_bytes())?;

  Ok(())
}
