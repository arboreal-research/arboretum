use cozo::NamedRows;
use std::{thread::sleep, time::Duration};
use tokio::{io::AsyncWriteExt, net::TcpStream, sync::mpsc::Receiver};
use tracing::{error, trace};

static CLIENT_MAX_RETRY_COUNT: usize = 10;

pub enum TcpClientWorkItem {
    SendData(String, NamedRows),
    Shutdown,
}

pub enum NetworkHandlerError {
    MissingShutdownSignal,
    IoError(std::io::Error),
}

impl From<std::io::Error> for NetworkHandlerError {
    fn from(e: std::io::Error) -> Self {
        NetworkHandlerError::IoError(e)
    }
}

async fn tcp_client_handler(
    stream: TcpStream,
    rx: &mut Receiver<TcpClientWorkItem>,
    item: &mut Option<TcpClientWorkItem>,
) -> Result<(), NetworkHandlerError> {
    let mut stream = tokio::io::BufStream::new(stream);

    loop {
        if item.is_none() {
            if let Some(new_item) = rx.recv().await {
                *item = Some(new_item);
            } else {
                return Err(NetworkHandlerError::MissingShutdownSignal);
            }
        }
        let message = match item.as_ref().unwrap() {
            TcpClientWorkItem::SendData(relation, rows) => (relation, rows),
            TcpClientWorkItem::Shutdown => {
                tracing::info!("TCP Worker recieved shutdown signal");

                // Write a length 0 to indicate we're done.
                stream.write_all(&(0 as u64).to_le_bytes()).await?;
                stream.flush().await?;
                break;
            }
        };

        let serialized_data = bincode::serialize(&message).expect("Failed to serialize object");

        // Get the length of the serialized data
        let length = serialized_data.len() as u64;

        // Write the length to the stream
        stream.write_all(&length.to_le_bytes()).await?;

        // Write the serialized data to the stream
        stream.write_all(&serialized_data).await?;

        // Fetch the next message.
        *item = None;
    }
    Ok(())
}

pub async fn tcp_client_worker(addr: String, mut rx: Receiver<TcpClientWorkItem>) {
    // Network connection retry loop.
    let mut retries_remaining = CLIENT_MAX_RETRY_COUNT;
    let mut item = None;
    while retries_remaining > 0 {
        // Connect to the Server
        let tcp_stream = TcpStream::connect(addr.clone()).await;
        if let Err(e) = tcp_stream {
            error!(?e);
        } else {
            retries_remaining = CLIENT_MAX_RETRY_COUNT;

            match tcp_client_handler(tcp_stream.unwrap(), &mut rx, &mut item).await {
                Ok(()) => {
                    trace!("Shutdown message recieved");
                    return;
                }
                Err(NetworkHandlerError::MissingShutdownSignal) => {
                    tracing::error!("Missing shutdown signal");
                    return;
                }
                Err(NetworkHandlerError::IoError(e)) => {
                    tracing::error!(?e);
                }
            }
        }
        sleep(Duration::from_secs(5));
        retries_remaining -= 1;
    }
    error!("Network worker out of retries.");
}
