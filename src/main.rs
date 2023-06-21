use ethers::{
    abi::RawLog,
    core::types::Filter,
    prelude::{abigen, EthEvent},
    providers::{Middleware, Provider, StreamExt, Ws},
    types::BlockNumber,
};
use eyre::Result;
use std::sync::Arc;

// The name of the event was changed in
// https://github.com/matter-labs/zksync-2-contracts/commit/ef3517270f0a38928a25976e39eb03a1c92d07ae
abigen!(
    OldL2StandardToken,
    r#"[
        event BridgeInitialization(address indexed l1Token, string name, string symbol, uint8 decimals)
    ]"#
);

#[tokio::main]
async fn main() -> Result<()> {
    let client =
        Provider::<Ws>::connect_with_reconnects("wss://testnet.era.zksync.dev/ws", 0).await?;
    let client = Arc::new(client);

    let latest_block = client
        .get_block(BlockNumber::Latest)
        .await
        .unwrap()
        .expect("last block number always exists in a live network; qed")
        .number
        .expect("last block always has a number; qed");

    let erc20_transfer_filter = Filter::new()
        .from_block(600000)
        .to_block(latest_block)
        .topic0(vec![BridgeInitializationFilter::signature()]);

    let mut stream = client.get_logs_paginated(&erc20_transfer_filter, 256);

    while let Some(log) = stream.next().await {
        let log = log?;
        let raw_log: RawLog = log.clone().into();

        if let Ok(decoded_log) = BridgeInitializationFilter::decode_log(&raw_log) {
            println!("{log:#?}\n{decoded_log:#?}");
        }
    }

    Ok(())
}
