//! JSON RPC Client
//! 
//! This library provides a way to send JSON RPC requests, there are 2 options,
//! without api_key and with api_key which sets request header API_KEY
//! 
//! Examples
//! ``` no_run
//! // call to remote function `mul` taking 2 numbers, they can be u8, f32, etc
//! println!(
//!     "2 + 3 = {:?}",
//!     jsonrpc_v2_awc::Request::new("mul", jsonrpc_v2_awc::Params([2, 3]), 0)
//!         .send("http://localhost:8082/api")
//!         .await?
//!         .body()
//!         .await?
//! );
//!
//! // call to remote function `timestamp` with no params, in this case params can be () or ""
//! println!(
//!     "2 + 3 = {:?}",
//!     jsonrpc_v2_awc::Request::new("timestamp", jsonrpc_v2_awc::Params(()), 1)
//!         .send("http://localhost:8082/api")
//!         .await?
//!         .body()
//!         .await?
//! );
//! 
//! // call to remote function `timestamp` with no params, using api_key
//! println!(
//!     "2 + 3 = {:?}",
//!     jsonrpc_v2_awc::Request::new("timestamp", jsonrpc_v2_awc::Params(()), 1)
//!         .send_with_api_key("http://localhost:8082/api", "API_KEY", "ds09ds9d-0d9s0d.xxx.yyy")
//!         .await?
//!         .body()
//!         .await?
//! );
//! ```
use awc::Client;
use serde::Serialize;

pub const JSONRPC_VERSION: &str = "2.0";

#[derive(Debug, Serialize)]
pub struct Params<T: Serialize>(pub T);

#[derive(Debug, Serialize)]
pub struct Request<T: Serialize> {
    jsonrpc: String,
    pub method: String,
    pub params: Params<T>,
    pub id: u64,
}

impl<T: Serialize> Request<T> {
    pub fn new(method: &str, params: Params<T>, id: u64) -> Request<T> {
        Request {
            jsonrpc: JSONRPC_VERSION.to_string(),
            method: method.to_string(),
            params: params,
            id: id,
        }
    }
    pub fn send(self, url: &str) -> awc::SendClientRequest {
        return Client::new()
            .post(url)
            .header("User-Agent", "actix-web/3.0")
            .header("Content-Type", "application/json")
            .send_json(&self);
    }

    pub fn send_with_api_key(self, url: &str, key_name: &str, key_value: &str) -> awc::SendClientRequest {
        return Client::new()
            .post(url)
            .header("User-Agent", "actix-web/3.0")
            .header("Content-Type", "application/json")
            .header(key_name, key_value)
            .send_json(&self);
    }
}
