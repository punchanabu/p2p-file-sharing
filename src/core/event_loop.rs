mod core;

async fn event_loop(swarm: &mut impl Stream<Item = SwarmEvent<MyBehaviorEvent>>, stdin_rx: &mut mpsc::Receiver<String>) {
    loop {
        tokio::select! {
            // Handle: stdin input
            line = stdin_rx.recv() => {
                if let Some(line) = line {
                    println("You entered: {}", line);
                }
            } else {
                break;
            }

            // Handle: swarm events
            event = swarm.select_next_some() => {
                handle_swarm_event(event);
            }
        }
    }
}

// Handle: individual swarm events
fn handle_swarm_event(event: SwarmEvent<MyBehaviorEvent>) {
    match event {
        SwarmEvent::NewListenAddr {address, ..} => {
            println("Listening on: {}", address);
        }
        SwarmEvent::Behaviour(MyBehaviorEvent::Mdns(event)) => {
            handle_mdns_event(event);
        }
        _ => {}
    }
}