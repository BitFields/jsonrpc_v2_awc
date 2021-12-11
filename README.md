# JSON RPC Client - Based on awc crate (actix web client)

## Examples

```rust
// call to remote function `mul` taking 2 numbers, they can be u8, f32, etc
println!(
    "2 + 3 = {:?}",
    jsonrpc_v2_awc::Request::new("mul", jsonrpc_v2_awc::Params([2, 3]), 0)
        .send("http://localhost:8082/api")
        .await?
        .body()
        .await?
);

// call to remote function `timestamp` with no params, in this case params can be () or ""
println!(
    "2 + 3 = {:?}",
    jsonrpc_v2_awc::Request::new("timestamp", jsonrpc_v2_awc::Params(()), 1)
        .send("http://localhost:8082/api")
        .await?
        .body()
        .await?
);
```
