
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use chrono::UTC;

use ::error::*;
use ::init::{
  State
};

use futures::{
  Future,
  Stream,
  future
};

macro_rules! j(
  ($($arg:tt)*) => { JsonValue::from($($arg)*) }
);


pub fn now_utc_ms() -> i64 {
  let time = UTC::now();
  time.timestamp() * 1000 + (time.timestamp_subsec_millis() as i64)
}

pub fn now_utc_iso8601() -> String {
  format!("{:?}", UTC::now())
}

pub fn decr_atomic(size: &Arc<AtomicUsize>) -> usize {
  size.fetch_sub(1, Ordering::SeqCst).saturating_sub(1)
}

pub fn incr_atomic(size: &Arc<AtomicUsize>) -> usize {
  size.fetch_add(1, Ordering::SeqCst).wrapping_add(1)
}

pub fn read_atomic(size: &Arc<AtomicUsize>) -> usize {
  size.load(Ordering::SeqCst)
}

pub fn set_atomic(size: &Arc<AtomicUsize>, val: usize) -> usize {
  size.swap(val, Ordering::SeqCst)
}

pub fn future_error<T: 'static>(err: Error) -> Box<Future<Item=T, Error=Error>> {
  Box::new(future::err(err))
}

pub fn future_ok<T: 'static>(d: T) -> Box<Future<Item=T, Error=Error>> {
  Box::new(future::ok(d))
}