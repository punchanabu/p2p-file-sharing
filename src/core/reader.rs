// Spawn a task to read input from stdin and send it through the provided channel.
fn spawn_reader(stdin_tx: mpsc::Sender<String>) {
    tokio::spawn(async move {
        let mut stdin = BufReader::new(tokio::io::stdin()).line();
        while let Some(line) = stdin.next_line().await.unwrap_or(None) {
            if stdin_tx.send(line).await.is_err() {
                break;
            }
        }
    })
}