use libp2p::{
    development_transport,
    identity,
    mdns::tokio::Behaviour as TokioBehaviour,
    swarm::Swarm,
    PeerId,
};
use crate::behaviour::behaviour::MyBehaviour;
use std::error::Error;

pub async fn build_swarm(port: u16) -> Result<(Swarm<MyBehaviour>, PeerId), Box<dyn Error>> {
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());

    let transport = development_transport(id_keys.clone()).await?;

    let mdns = TokioBehaviour::new(Default::default(), peer_id).await?;
    let behaviour = MyBehaviour { mdns };

    let mut swarm = Swarm::new(transport, behaviour, peer_id);

    swarm.listen_on(format!("/ip4/0.0.0.0/tcp/{}", port).parse()?)?;

    Ok((swarm, peer_id))
}
