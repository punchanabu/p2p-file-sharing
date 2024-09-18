use futures::StreamExt;
use libp2p::swarm::SwarmEvent;
use crate::behaviour::behaviour::{MyBehaviorEvent, handle_mdns_event};
use tokio::sync::mpsc;

async fn event_loop(
    swarm: &mut (impl StreamExt<Item = SwarmEvent<MyBehaviorEvent>> + Unpin), 
    mut stdin_rx: mpsc::Receiver<String>
) {
    loop {
        tokio::select! {
            // Handle: stdin input
            line = stdin_rx.recv() => {
                match line {
                    Some(line) => {
                        println!("You entered: {}", line);
                    }
                    None => {
                        break;
                    }
                }
            }
            

            // Handle: swarm events
            event = swarm.next() => {
                if let Some(event) = event {
                    handle_swarm_event(event);
                }
            }
        }
    }
}

// Handle: individual swarm events
fn handle_swarm_event(event: SwarmEvent<MyBehaviorEvent>) {
    match event {
        SwarmEvent::NewListenAddr {address, ..} => {
            println!("Listening on: {}", address);
        }
        SwarmEvent::Behaviour(MyBehaviorEvent::Mdns(event)) => {
            handle_mdns_event(event);
        }
        _ => {}
    }
}