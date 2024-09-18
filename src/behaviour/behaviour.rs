use libp2p::{
    mdns::{Mdns, MdnsEvent},
    swarm::{NetworkBehaviour, NetworkBehaviourActionEventProcess}
};

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MyBehaviourEvent")]
pub struct MyBehaviour {
    pub mdns: Mdns
}

#[derive(Debug)]
pub enum MyBehaviorEvent {
    Mdns(MdnsEvent),
}

impl From<MdnsEvent> for MyBehaviourEvent {
    fn from(event: MdnsEvent) -> Self {
        MyBehaviourEvent::Mdns(event)
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
