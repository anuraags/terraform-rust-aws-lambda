// This example requires the following input to succeed:
// { "command": "do something" }
mod secrets;

use lambda_runtime::{handler_fn, Context, Error};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;

/// This is also a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
struct Request {
    command: String,
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

pub(crate) async fn my_handler(event: Request, ctx: Context) -> Result<Response, Error> {
    // extract some useful info from the request
    let command = event.command;

    // prepare the response
    let resp = Response {
        req_id: ctx.request_id,
        msg: format!("Command {} executed.", command),
    };

    // return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let secrets = secrets::get_secrets().await;
    // // required to enable CloudWatch error logging by the runtime
    // // can be replaced with any other method of initializing `log`
    // SimpleLogger::new()
    //     .with_level(LevelFilter::Info)
    //     .init()
    //     .unwrap();

    // let func = handler_fn(my_handler);
    // lambda_runtime::run(func).await?;
    match secrets {
        Ok(_result) => return Ok(()),
        Err(e) => panic!("{:?}", e),
    }
}
