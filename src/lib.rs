#![allow(clippy::disallowed_names)]

use axum::{extract::Request, response::Response, serve};

mod test_client;
pub use self::test_client::*;

pub mod tracing_helpers;

pub fn assert_send<T: Send>() {}
pub fn assert_sync<T: Sync>() {}

#[allow(dead_code)]
pub struct NotSendSync(*const ());
