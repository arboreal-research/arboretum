use std::net::ToSocketAddrs;

use actix_server::Server;
use actix_service::{fn_service, ServiceFactoryExt as _};
use arboretum_ffi::tcp_client::{ClangPluginClientMessage, FfiValue};
use arboretum_core::{GraphBuffer, Value};
use tokio::{io::AsyncReadExt, net::TcpStream, sync::mpsc::Sender};

fn to_value(ffi_value: FfiValue) -> Value {
    match ffi_value {
        FfiValue::Unsigned(u) => Value::Unsigned(u),
        FfiValue::Signed(s) => Value::Signed(s),
        FfiValue::Double(d) => Value::Double(d),
        FfiValue::String(s) => Value::String(s),
    }
}

pub async fn clang_collector<U>(tx: Sender<GraphBuffer>, bind_addr: U) -> Result<(), std::io::Error>
where
    U: ToSocketAddrs,
{
    Server::build()
        .bind("clang-collector", bind_addr, move || {
            let tx = tx.clone();
            fn_service(move |stream: TcpStream| {
                tracing::info!("Accepted connection from {}", stream.peer_addr().unwrap());

                let mut stream = tokio::io::BufReader::new(stream);

                let tx = tx.clone();
                async move {
                    let mut size_buf = [0u8; 8];

                    let mut buf = GraphBuffer::new();

                    loop {
                        // Read the size byte.
                        let size = match stream.read_exact(&mut size_buf).await {
                            Ok(_) => u64::from_le_bytes(size_buf),

                            Err(e) => {
                                tracing::error!("Stream Error reading size: {:?}", e);
                                return Err::<(), ()>(());
                            }
                        };

                        // If we get a size 0 object, we're done.
                        if size == 0 {
                            tracing::info!("Finished reading GraphBuffer, sending it downstream.");
                            if let Err(e) = tx.send(buf).await {
                                tracing::error!("Send Error: {:?}", e);
                                return Err(());
                            }
                            return Ok(());
                        }

                        // Read the data.
                        let mut data_buf = vec![0u8; size as usize];

                        if let Err(e) = stream.read_exact(&mut data_buf).await {
                            tracing::error!("Stream Error reading data: {:?}", e);
                            return Err(());
                        }

                        match bincode::deserialize(&data_buf) {
                            Ok(ClangPluginClientMessage::NewNodeWithProps(id, props)) => {
                                buf.add_node_with_props(id, to_value(props))
                            }

                            Ok(ClangPluginClientMessage::NewNamedNode(name, id)) => {
                                buf.add_named_node(id, name);
                            }

                            Ok(ClangPluginClientMessage::NewNamedNodeWithProps(
                                name,
                                id,
                                props,
                            )) => {
                                buf.add_named_node_with_props(id, name, to_value(props));
                            }

                            Ok(ClangPluginClientMessage::NewEdge(s, p, o)) => {
                                buf.add_edge((s, p, o));
                            }

                            Ok(ClangPluginClientMessage::NewEdgeWithProps(s, p, o, props)) => {
                                buf.add_edge_with_props((s, p, o), to_value(props));
                            }

                            Err(e) => {
                                tracing::error!("Deserialization Error: {:?}", e);
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
