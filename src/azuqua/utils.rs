

// hashing, signing

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use std::str;
use std::str::from_utf8;
use std::str::FromStr;

use ::error::*;

use hyper::header::{
  Raw,
  Header,
  Headers,
  ContentType,
  ContentLength
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
use hyper::error::Error as HyperError;
use hyper::mime;

use futures::{
  Future,
  Stream
};

use ::twitch::channel::HttpsClient;

use json;
use json::JsonValue;


const POST_VERB: &'static str = "POST";
const TIMESTAMP_HEADER: &'static str = "x-api-timestamp";
const SIG_HEADER: &'static str = "x-api-hash";
const KEY_HEADER: &'static str = "x-api-accessKey";
const COLON: &'static [u8] = &[b':'];

fn flo_invoke_route(alias: &str) -> String {
  format!("https://api.azuqua.com/flo/{}/invoke", alias)
}

fn sign(secret: &str, path: &str, timestamp: &str, payload: &[u8]) -> String {
  let mut hasher = Sha256::new();

  hasher.input_str(POST_VERB);
  hasher.input(COLON);
  hasher.input_str(path);
  hasher.input(COLON);
  hasher.input_str(timestamp);

  hasher.result_str()
}

fn add_headers(req: &mut Request, timestamp: &str, sig: &str, key: &str, payload: &[u8]) {
  let mut headers = req.headers_mut();

  headers.set_raw(TIMESTAMP_HEADER, timestamp);
  headers.set_raw(SIG_HEADER, sig);
  headers.set_raw(KEY_HEADER, key);

  headers.set(ContentLength(payload.len() as u64));
  headers.set(ContentType::json());
}

fn make_request(client: &HttpsClient, request: Request) -> Box<Future<Item=JsonValue, Error=Error>> {
  Box::new(client.request(request).from_err::<Error>().and_then(|mut response| {
    response.body().concat2().from_err::<Error>().and_then(|mut body| {
      let body_str = str::from_utf8(&body)?;
      Ok(json::parse(body_str)?)
    })
  }))
}

pub fn make_invoke_request(client: &HttpsClient, alias: &str, key: &str, secret: &str, payload: JsonValue) -> Box<Future<Item=JsonValue, Error=Error>> {
  let uri_str = flo_invoke_route(alias);
  let uri = fry!(Uri::from_str(&uri_str));
  let mut request = Request::new(Method::Post, uri);

  let payload = json::stringify(payload);

  let now = ::utils::now_utc_iso8601();
  let sig = sign(secret, &uri_str, &now, payload.as_bytes());

  add_headers(&mut request, &now, &sig, key, payload.as_bytes());
  request.set_body(payload);

  make_request(client, request)
}

#[cfg(test)]
mod tests {
  use super::*;

  use tokio_core::reactor::{Core,Handle};

  #[test]
  fn should_invoke_fake_flo() {
    let alias = "foo";
    let key = "bar";
    let secret = "baz";

    let payload = object! {
      "foo" => "wibblewobble"
    };

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let client = ::twitch::utils::create_https_client(&handle, false)
      .expect("Couldnt create http client");


    let ft = make_invoke_request(&client, alias, key, secret, payload);

    if let Err(e) = core.run(ft) {
      panic!("Error invoking flo: {:?}", e);
    }
  }

}
