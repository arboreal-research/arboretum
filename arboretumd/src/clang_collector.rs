use std::{net::ToSocketAddrs, sync::Arc};

use actix_server::Server;
use actix_service::{fn_service, ServiceFactoryExt as _};
use reify_rs::ffi::Record;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::{mpsc::Sender, Mutex},
};

pub enum CollectorSessionMessage {
    Open(u32),
    Record(u32, Record),
    OkClose(u32),
    ErrClose(u32),
}

pub async fn clang_collector<U>(
    next_subgraph_id: Arc<Mutex<u32>>,
    tx: Sender<CollectorSessionMessage>,
    bind_addr: U,
) -> Result<(), std::io::Error>
where
    U: ToSocketAddrs,
{
    Server::build()
        .bind("clang-collector", bind_addr, move || {
            let tx = tx.clone();
            let next_subgraph_id = next_subgraph_id.clone();
            fn_service(move |stream: TcpStream| {
                tracing::info!("Accepted connection from {}", stream.peer_addr().unwrap());

                let mut stream = tokio::io::BufReader::new(stream);

                let tx = tx.clone();
                let next_subgraph_id = next_subgraph_id.clone();
                async move {
                    let subgraph_id;
                    {
                        let mut next_subgraph_id = next_subgraph_id.lock().await;
                        subgraph_id = *next_subgraph_id;
                        *next_subgraph_id += 1;
                    }
                    if let Err(e) = tx.send(CollectorSessionMessage::Open(subgraph_id)).await {
                        tracing::error!("Failed to send Open message: {:?}", e);
                        return Err::<(), ()>(());
                    }

                    let mut size_buf = [0u8; 8];

                    loop {
                        // Write the subgraph id.
                        if let Err(e) = stream.write_all(&subgraph_id.to_le_bytes()).await {
                            tracing::error!("Stream Error writing subgraph id: {:?}", e);
                            if let Err(e) = tx
                                .send(CollectorSessionMessage::ErrClose(subgraph_id))
                                .await
                            {
                                tracing::error!("Failed to send ErrClose message: {:?}", e);
                            }
                            return Err(());
                        }

                        // Read the size byte.
                        let size = match stream.read_exact(&mut size_buf).await {
                            Ok(_) => u64::from_le_bytes(size_buf),

                            Err(e) => {
                                tracing::error!("Stream Error reading size: {:?}", e);
                                if let Err(e) = tx
                                    .send(CollectorSessionMessage::ErrClose(subgraph_id))
                                    .await
                                {
                                    tracing::error!("Failed to send ErrClose message: {:?}", e);
                                }
                                return Err(());
                            }
                        };

                        // If we get a size 0 object, we're done.
                        if size == 0 {
                            if let Err(e) =
                                tx.send(CollectorSessionMessage::OkClose(subgraph_id)).await
                            {
                                tracing::error!("Failed to send OkClose message: {:?}", e);
                            }
                            return Ok(());
                        }

                        // Read the data.
                        let mut data_buf = vec![0u8; size as usize];

                        if let Err(e) = stream.read_exact(&mut data_buf).await {
                            tracing::error!("Stream Error reading data: {:?}", e);
                            if let Err(e) = tx
                                .send(CollectorSessionMessage::ErrClose(subgraph_id))
                                .await
                            {
                                tracing::error!("Failed to send ErrClose message: {:?}", e);
                            }
                            return Err(());
                        }

                        match bincode::deserialize(&data_buf) {
                            Ok(message) => {
                                if let Err(e) = tx
                                    .send(CollectorSessionMessage::Record(subgraph_id, message))
                                    .await
                                {
                                    tracing::error!("Failed to send rows to consumer: {:?}", e);
                                    if let Err(e) = tx
                                        .send(CollectorSessionMessage::ErrClose(subgraph_id))
                                        .await
                                    {
                                        tracing::error!("Failed to send ErrClose message: {:?}", e);
                                    }
                                    return Err(());
                                }
                            }

                            Err(e) => {
                                tracing::error!("Deserialization Error: {:?}", e);
                                if let Err(e) = tx
                                    .send(CollectorSessionMessage::ErrClose(subgraph_id))
                                    .await
                                {
                                    tracing::error!("Failed to send ErrClose message: {:?}", e);
                                }
                                return Err(());
                            }
                        };
                    }
                }
            })
            .map_err(|err| eprintln!("Service Error: {:?}", err))
        })?
        .run()
        .await
}
