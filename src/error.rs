use std::fmt;
use std::error;

use hyper::Error as HyperError;
use hyper::error::UriError;
use irc::error::IrcError;
use std::io::Error as IoError;

use futures::sync::mpsc::SendError;
use futures::Canceled;
use tokio_timer::TimeoutError;
use tokio_timer::TimerError;
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use std::net::AddrParseError;
use std::num::ParseIntError;
use json::Error as JsonError;


fn print_error<T: fmt::Debug>(kind: ErrorKind, e: T) -> Error {
  Error::new(kind, format!("{:?}", e))
}



#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
  TwitchError,
  IO,
  Config,
  ParseError,
  Unknown
}

impl ErrorKind {
  pub fn to_str(&self) -> &'static str {
    match *self {
      ErrorKind::Config => "Config Error",
      ErrorKind::IO => "IO Error",
      ErrorKind::TwitchError => "Twitch Error",
      ErrorKind::ParseError => "Parse Error",
      ErrorKind::Unknown => "Unknown Error"
    }
  }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Error {
  kind: ErrorKind,
  details: String,
}

impl Error {
  pub fn new<S: Into<String>>(kind: ErrorKind, details: S) -> Error {
    Error { kind, details: details.into() }
  }

  pub fn details(&self) -> &str {
    &self.details
  }

  pub fn to_string(&self) -> String {
    format!("{}: {}", self.kind.to_str(), self.details)
  }
}

impl fmt::Debug for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: {}", self.kind.to_str(), self.details)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: {}", self.kind.to_str(), self.details)
  }
}

impl error::Error for Error {
  fn description(&self) -> &str {
    &self.details
  }
}

impl From<IrcError> for Error {
  fn from(e: IrcError) -> Self {
    print_error(ErrorKind::TwitchError, e)
  }
}

impl From<HyperError> for Error {
  fn from(e: HyperError) -> Self {
    print_error(ErrorKind::TwitchError, e)
  }
}

impl From<UriError> for Error {
  fn from(e: UriError) -> Self {
    print_error(ErrorKind::TwitchError, e)
  }
}

impl From<IoError> for Error {
  fn from(e: IoError) -> Self {
    print_error(ErrorKind::IO, e)
  }
}

impl From<ParseIntError> for Error {
  fn from(e: ParseIntError) -> Self {
    print_error(ErrorKind::ParseError, e)
  }
}

impl From<Canceled> for Error {
  fn from(e: Canceled) -> Self {
    print_error(ErrorKind::Unknown, e)
  }
}

impl From<()> for Error {
  fn from(e: ()) -> Self {
    print_error(ErrorKind::Unknown, "")
  }
}

impl From<TimerError> for Error {
  fn from(e: TimerError) -> Self {
    print_error(ErrorKind::Unknown, e)
  }
}

impl From<Utf8Error> for Error {
  fn from(e: Utf8Error) -> Self {
    print_error(ErrorKind::Unknown, e)
  }
}

impl From<JsonError> for Error {
  fn from(e: JsonError) -> Self {
    print_error(ErrorKind::Unknown, e)
  }
}