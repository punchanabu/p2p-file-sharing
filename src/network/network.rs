use libp2p::{
    development_transport,
    identity,
    mdns::tokio::Behaviour as TokioBehaviour,
    swarm::Swarm,
    PeerId,
    swarm::SwarmBuilder, // Import the SwarmBuilder
};
use crate::behaviour::behaviour::MyBehaviour;
use std::error::Error;

pub async fn build_swarm(port: u16) -> Result<(Swarm<MyBehaviour>, PeerId), Box<dyn Error>> {
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());

    let transport = development_transport(id_keys.clone()).await?;

    let mdns = TokioBehaviour::new(Default::default(), peer_id)?;
    let behaviour = MyBehaviour { mdns };

    // Use SwarmBuilder with_tokio_executor to create a new Swarm
    let mut swarm = SwarmBuilder::with_executor(transport, behaviour, peer_id, Box::new(|fut| {
            tokio::spawn(fut);
        }))
        .build();

    swarm.listen_on(format!("/ip4/0.0.0.0/tcp/{}", port).parse()?)?;

    Ok((swarm, peer_id))
}
