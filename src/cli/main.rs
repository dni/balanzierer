use tokio;
use std::time::Duration;
use console::Style;
use std::path::{Path};
use serde::{Serialize, Deserialize};
use anyhow::Error;

use indicatif::{ProgressBar, ProgressStyle};
use cln_plugin::{anyhow};
use cln_rpc::{model::ListpeersRequest, ClnRpc, Request, Response};

use balanzierer::{Channel};


#[tokio::main]
async fn main() -> Result<(), Error> {
    let red = Style::new().red();
    let yellow = Style::new().yellow();

    let bar = ProgressBar::new(1);
    bar.set_style(ProgressStyle::default_bar()
        .template("[{bar:40.green}] {pos:>7}/{len:7} {msg}")
        .progress_chars("##-"));

    println!("{}", yellow.apply_to("===================================================="));
    println!("Channel {}: Morphy, sid=724596x418x1", red.apply_to("#1"));
    bar.finish();
    println!("{}", yellow.apply_to("===================================================="));
    println!("Channel {}: Morphy, sid=724596x418x1", red.apply_to("#2"));
    println!("{}", yellow.apply_to("===================================================="));
    bar.finish();
    println!("{}", yellow.apply_to("===================================================="));
    println!("Channel {}: Morphy, sid=724596x418x1", red.apply_to("#3"));
    println!("{}", yellow.apply_to("===================================================="));
    bar.finish();
    println!("{}", yellow.apply_to("===================================================="));

    Ok(())

    // let path = Path::new("lightning-rpc");
    // let mut rpc = ClnRpc::new(&path).await?;

    // let response = rpc
    //     .call(Request::BalanceStatus(ListpeersRequest {
    //         id: None,
    //         level: None,
    //     }))
    //     .await
    //     .map_err(|e| anyhow!("Error calling listpeers: {:?}", e))?;

    // match response {
    //     Response::ListPeers(res) => {
    //         let mut channels: Vec<Channel> = vec!();
    //         for peer in &res.peers {
    //             let channel = Channel{
    //                 alias: "lol".to_string(),
    //                 short_id: "lol".to_string(),
    //                 spendable_msatoshis: 1000,
    //                 receivable_msatoshis: 1000,
    //             };
    //             channels.push(channel);
    //         }
    //         println!("{}", yellow.apply_to("===================================================="));
    //         println!("Channel {}: Morphy, sid=724596x418x1", red.apply_to("#1"));
    //         bar.finish();
    //         println!("{}", yellow.apply_to("===================================================="));
    //         println!("Channel {}: Morphy, sid=724596x418x1", red.apply_to("#2"));
    //         println!("{}", yellow.apply_to("===================================================="));
    //         bar.finish();
    //         println!("{}", yellow.apply_to("===================================================="));
    //         println!("Channel {}: Morphy, sid=724596x418x1", red.apply_to("#3"));
    //         println!("{}", yellow.apply_to("===================================================="));
    //         bar.finish();
    //         println!("{}", yellow.apply_to("===================================================="));
    //         Ok(())
    //     },
    //     _ => Err(anyhow!("Response not ListPeers"))
    // }
}
