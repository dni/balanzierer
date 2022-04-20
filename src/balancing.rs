use cln_plugin::{Plugin, Error};
use crate::PluginState;
pub async fn balance_method(_p: Plugin<PluginState>, _v: serde_json::Value) -> Result<serde_json::Value, Error> {
    log::info!("Got a balance_method call: {}", _v);
    let param = _p.option("test-option");
    let message = format!("balancing... {:?}, {}", param.unwrap(), _v);
    Ok(json!(message))
}
