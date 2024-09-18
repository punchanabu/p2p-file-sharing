// Setup application: parses arguments, build the swarms, initialize an event loop.
async fn setup() -> Result<(impl Stream<Item = SwarmEvent<MyBehaviorEvent>, String, mpsc::Receiver<String>>), Box<dyn Error>> {
    let args = Args::parse();
    let port = args.port.unwrap_or(0);

    let (swarm, peer_id) = build_swarm(port).await?;

    let (stdin_tx, stdin_rx) = mpsc::channel(100);

    spawn_reader(stdin_tx);

    Ok((swarm, format!("{:?}", peer_id), stdin_rx))
}