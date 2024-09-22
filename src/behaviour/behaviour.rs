use libp2p::{
    mdns::{Event as MdnsEvent, tokio::Behaviour as TokioBehaviour},
    swarm::NetworkBehaviour,
};

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MyBehaviorEvent")]
pub struct MyBehaviour {
    pub mdns: TokioBehaviour,
}

#[derive(Debug)]
pub enum MyBehaviorEvent {
    Mdns(MdnsEvent),
}

impl From<MdnsEvent> for MyBehaviorEvent {
    fn from(event: MdnsEvent) -> Self {
        MyBehaviorEvent::Mdns(event)
    }
}

// Handle: mDNS events
pub fn handle_mdns_event(event: MdnsEvent) {
    match event {
        MdnsEvent::Discovered(peers) => {
            for (peer_id, _multiaddr) in peers {
                println!("Discovered peer: {:?}", peer_id);
            }
        }
        MdnsEvent::Expired(list) => {
            for (peer_id, _multiaddr) in list {
                println!("Expired: {}", peer_id);
            }
        }
    }
}