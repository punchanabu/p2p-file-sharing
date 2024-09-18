use futures::stream::Stream;
use libp2p::swarm::SwarmEvent;
use crate::behaviour::behaviour::MyBehaviorEvent;
use tokio::sync::mpsc;
use clap::Parser;
use crate::cli::cli::Args;
use crate::core::reader::spawn_reader;
use crate::network::network::build_swarm;
use std::error::Error;
use void::Void;

// Setup application: parses arguments, build the swarms, initialize an event loop.
pub async fn setup() -> Result<(impl Stream<Item = SwarmEvent<MyBehaviorEvent, Void>>, String, mpsc::Receiver<String>), Box<dyn Error>> {    
    let args = Args::parse();
    let port = args.port.unwrap_or(0);

    let (swarm, peer_id) = build_swarm(port).await?;

    let (stdin_tx, stdin_rx) = mpsc::channel(100);

    spawn_reader(stdin_tx);

    Ok((swarm, format!("{:?}", peer_id), stdin_rx))
}