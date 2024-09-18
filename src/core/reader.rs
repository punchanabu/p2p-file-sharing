use tokio::sync::mpsc;
use tokio::io::{BufReader, AsyncBufReadExt};
use tokio_stream::wrappers::LinesStream;
use tokio_stream::StreamExt;

pub fn spawn_reader(stdin_tx: mpsc::Sender<String>) {
    tokio::spawn(async move {
        let reader = BufReader::new(tokio::io::stdin());
        let lines = reader.lines();
        let mut lines_stream = LinesStream::new(lines);

        while let Some(line_result) = lines_stream.next().await {
            match line_result {
                Ok(line) => {
                    if stdin_tx.send(line).await.is_err() {
                        // The receiver has been dropped
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Error reading line: {}", e);
                    break;
                }
            }
        }
    });
}
