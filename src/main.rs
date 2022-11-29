use std::sync::Arc;
use std::time::SystemTime;

use ethers::prelude::{
    abigen, Address, LocalWallet, Middleware, Provider, SignerMiddleware, StreamExt, Ws,
};
use ethers::signers::Signer;
use ethers::types::{Bytes, U256};
use eyre::Result;

abigen!(IBC, "./src/IBC.json");

#[tokio::main]
async fn main() -> Result<()> {
    // prepare websocket endpoint provider
    let provider = Provider::<Ws>::connect("ws://127.0.0.1:8545").await?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let tip_number = provider.get_block_number().await?;
    println!("chain_id = {}, tip_number = {}\n", chain_id, tip_number);

    // prepare IBC contract
    let wallet = "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d"
        .parse::<LocalWallet>()?
        .with_chain_id(chain_id);
    let signer = SignerMiddleware::new(provider, wallet);
    let address = "0x9fe46736679d2d9a65f0992f2272de9f3c7fa6e0".parse::<Address>()?;
    let ibc = Arc::new(IBC::new(address, Arc::new(signer)));

    // spawn a new task to monitor events from IBC contract
    let ibc_events = Arc::clone(&ibc);
    let handler = tokio::spawn(async move {
        println!("start listening IBC events...");
        if let Ok(stream) = ibc_events.events().from_block(tip_number).stream().await {
            let mut meta_stream = stream.with_meta();
            while let Some(Ok((event, meta))) = meta_stream.next().await {
                if meta.block_number > tip_number {
                    println!("[event] = {:?}", event);
                    println!("[event_meta] = {:?}\n", meta);
                }
            }
        }
    });

    // use ibc to send transaction to trigger IBC contract events
    println!("start sending IBC command of CreateClient...");
    let msg_client_create = MsgClientCreate {
        client: ClientState {
            chain_id: "5".to_owned(),
            client_type: 4,
            latest_height: [0u8; 32],
            frozen_height: [0u8; 32],
            trusting_period: U256::from(10000),
            max_clock_drift: U256::from(10000),
            extra_payload: Bytes::from([]),
        },
        consensus: ConsensusState {
            timestamp: U256::from(SystemTime::now().elapsed()?.as_secs()),
            commitment_root: [0u8; 32],
            extra_payload: Bytes::from([]),
        },
    };
    let receipt = ibc.client_create(msg_client_create).send().await?.await?;
    println!("[receipt] = {:?}", receipt);

    handler.await?;
    Ok(())
}
