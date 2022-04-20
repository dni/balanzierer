use cln_plugin::{anyhow, Plugin, Error};
use cln_rpc::{model::GetinfoRequest, ClnRpc, Request};

use crate::PluginState;
pub async fn balance_method(_p: Plugin<PluginState>, _v: serde_json::Value) -> Result<serde_json::Value, Error> {
    log::info!("Got a balance_method call: {}", _v);

    // let param = _p.option("test-option");
    // let message = format!("balancing... {:?}, {}, {:?}", param.unwrap(), _v, _p.state().rpc_path);
    // Ok(json!(message))

    let mut rpc = ClnRpc::new(&_p.state().rpc_path).await?;
    let response = rpc
        .call(Request::Getinfo(GetinfoRequest {}))
        .await
        .map_err(|e| anyhow!("Error calling getinfo: {:?}", e))?;
    Ok(json!(response))
}
