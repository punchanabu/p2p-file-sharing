mod network;
mod cli;
mod behaviour;
mod core;

use std::error::Error;
use crate::core::setup::setup;
use crate::core::event_loop::event_loop;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Application setup: parsing arguments and building swarm
    let (mut swarm, peer_id, mut stdin_rx) = setup().await?;

    println!("Local peer id: {:?}", peer_id);

    // Main event loop
    event_loop(&mut swarm, stdin_rx).await;

    Ok(())
}
