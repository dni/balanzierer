use cln_plugin::{anyhow, Plugin, Error};
use cln_rpc::{model::ListchannelsRequest, ClnRpc, Request};
use crate::PluginState;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Channel {
    alias: String,
    short_id: String,
    spendable_msatoshis: u64,
    receivable_msatoshis: u64,
}

impl Channel {
    // async fn new() -> Result<Channel, Error> {
    //     self.short_id = "lololol".to_string();
    //     Ok(self)
    // }
    fn calculate_ratio(&self) -> f64 {
        self.spendable_msatoshis as f64 / self.receivable_msatoshis as f64
    }
}

pub async fn balance_method(_p: Plugin<PluginState>, _v: serde_json::Value) -> Result<serde_json::Value, Error> {
    log::info!("Got a balance_method call: {}", _v);
    // let param = _p.option("test-option");
    // let message = format!("balancing... {:?}, {}, {:?}", param.unwrap(), _v, _p.state().rpc_path);
    // Ok(json!(message))

    let rpc = ClnRpc::new(&_p.state().rpc_path).await?;
    let channels = get_balance_status(rpc);
    Ok(json!(channels.await?))
}

async fn get_balance_status(mut rpc: ClnRpc) -> Result<Vec<Channel>, Error> {
    let response = rpc
        .call(Request::ListChannels(ListchannelsRequest {
          short_channel_id: None,
          source: None,
          destination: None,
        }))
        .await
        .map_err(|e| anyhow!("Error calling listchannels: {:?}", e))?;

    let mut channels = vec!();
    let channel = Channel{
        alias: "lol".to_string(),
        short_id: "lol".to_string(),
        spendable_msatoshis: 1000,
        receivable_msatoshis: 1000,
    };
    channels.push(channel);
    Ok(channels)
}
