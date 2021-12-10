use awc::client::Client;
use serde::Serialize;

pub const JSONRPC_VERSION: &str = "2.0";

#[derive(Debug, Serialize)]
pub struct Params<T: Serialize>(pub T);

#[derive(Debug, Serialize)]
pub struct Request<T: Serialize> {
    pub jsonrpc: String,
    pub method: String,
    pub params: Params<T>,
    pub id: u64,
}

impl<T: Serialize> Request<T> {
    pub fn new(method: String, params: Params<T>, id: u64) -> Request<T> {
        Request {
            jsonrpc: JSONRPC_VERSION.to_string(),
            method: method,
            params: params,
            id: id,
        }
    }
    pub fn send(self) -> awc::SendClientRequest {
        return Client::new()
            .post("http://localhost:8082/api")
            .header("User-Agent", "actix-web/3.0")
            .header("Content-Type", "application/json")
            .send_json(&self);
    }

    pub fn send_with_api_key(self, key: String, value: String) -> awc::SendClientRequest {
        return Client::new()
            .post("http://localhost:8082/api")
            .header("User-Agent", "actix-web/3.0")
            .header("Content-Type", "application/json")
            .header(key, value)
            .send_json(&self);
    }
}
