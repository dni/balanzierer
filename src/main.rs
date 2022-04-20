//! This is a test plugin used to verify that we can compile and run
//! plugins using the Rust API against Core Lightning.
#[macro_use]
extern crate serde_json;
use std::path::{Path, PathBuf};
use cln_plugin::{options, Builder};
mod balancing;
use tokio;

#[derive(Clone, Debug)]
pub struct PluginState {
    rpc_path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let path = Path::new("lightning-rpc");
    let state = PluginState {
        rpc_path: path.into()
    };
    if let Some(plugin) = Builder::new(state.clone(), tokio::io::stdin(), tokio::io::stdout())
        .option(options::ConfigOption::new(
            "test-option",
            options::Value::Integer(42),
            "a test-option with default 42",
        ))
        .rpcmethod("balance", "Balancing Method", balancing::balance_method)
        // .subscribe("connect", connect_handler)
        // .hook("peer_connected", peer_connected_handler)
        .start()
        .await?
    {
        plugin.join().await
    } else {
        Ok(())
    }
}

// async fn connect_handler(_p: Plugin<()>, v: serde_json::Value) -> Result<(), Error> {
//     log::info!("Got a connect notification: {}", v);
//     Ok(())
// }

// async fn peer_connected_handler(
//     _p: Plugin<()>,
//     v: serde_json::Value,
// ) -> Result<serde_json::Value, Error> {
//     log::info!("Got a connect hook call: {}", v);
//     Ok(json!({"result": "continue"}))
// }
