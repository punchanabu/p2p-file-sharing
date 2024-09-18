use libp2p::{
    development_transport,
    identity,
    mdns::Mdns,
    swarm::Swarm,
    PeerId,
};
use crate::behaviour::MyBehaviour;
use std::error::Error;

pub async fn build_swarm(port: u16) -> Result<(Swarm<MyBehaviour>, PeerId), Box<dyn Error>> {
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());

    let transport = development_transport(id_keys.clone()).await?;

    let mdns = Mdns::new(Default::default()).await?;
    let behaviour = MyBehaviour { mdns };

    let mut swarm = Swarm::new(transport, behaviour, peer_id);

    swarm.listen_on(format!("/ip4/0.0.0.0/tcp/{}", port).parse()?)?;

    Ok((swarm, peer_id))
}
