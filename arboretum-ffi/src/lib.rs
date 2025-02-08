use once_cell::sync::Lazy;
use reify_rs::ffi::FfiMessage;
use serde::Serialize;
use std::{
    ffi::{c_char, CStr},
    io,
    sync::Mutex,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufStream},
    net::TcpStream,
    runtime::Runtime,
    sync::mpsc::Sender,
    task::JoinHandle,
};

static mut CLIENT: Mutex<Option<CollectorClient>> = Mutex::new(None);

static GLOBAL_RUNTIME: Lazy<Runtime> =
    Lazy::new(|| Runtime::new().expect("Failed to create Tokio runtime"));

async fn write_record<T: Serialize>(stream: &mut BufStream<TcpStream>, r: T) -> io::Result<()> {
    let serialized_data = bincode::serialize(&r).expect("Failed to serialize object");

    // Get the length of the serialized data
    let length = serialized_data.len() as u64;

    // Write the length to the stream
    stream.write_all(&length.to_le_bytes()).await?;

    // Write the serialized data to the stream
    stream.write_all(&serialized_data).await?;
    Ok(())
}

#[derive(Debug)]
pub struct CollectorClient {
    finalize_sender: Sender<FfiMessage>,
    join_handle: JoinHandle<()>,
    subgraph_id: u64,
}

impl CollectorClient {
    pub fn new(base_url: String) -> Self {
        let (send, mut recv) = tokio::sync::mpsc::channel::<FfiMessage>(1024);
        let finalize_sender = send.clone();
        unsafe {
            reify_rs::ffi::RECORD_SINK = Some(Box::new(move |record: FfiMessage| {
                send.blocking_send(record).expect("send failed")
            }))
        };

        let (mut stream, subgraph_id) = GLOBAL_RUNTIME.block_on(async {
            let mut stream = BufStream::new(TcpStream::connect(base_url).await.expect("Connect"));

            let subgraph_id = stream.read_u64().await.expect("read subgraph id");

            (stream, subgraph_id)
        });

        let join_handle = GLOBAL_RUNTIME.spawn(async move {
            while let Some(msg) = recv.recv().await {
                match msg {
                    FfiMessage::Finalize => recv.close(),
                    FfiMessage::Record(record) => write_record(&mut stream, record)
                        .await
                        .expect("write message"),
                }
            }
            stream
                .write_all(&0u64.to_le_bytes())
                .await
                .expect("write finalization");
        });

        Self {
            finalize_sender,
            join_handle,
            subgraph_id,
        }
    }

    pub fn finalize(&self) {}
}

#[no_mangle]
pub extern "C" fn arboretum_connect(url: *const c_char) -> bool {
    let url = match unsafe { CStr::from_ptr(url) }.to_str() {
        Ok(s) => s.to_owned(),
        Err(_) => return false,
    };

    unsafe { CLIENT.lock() }
        .unwrap()
        .replace(CollectorClient::new(url));

    true
}

#[no_mangle]
pub extern "C" fn arboretum_subgraph_id() -> u64 {
    unsafe { CLIENT.lock() }
        .expect("lock client")
        .as_ref()
        .unwrap()
        .subgraph_id
}

#[no_mangle]
pub extern "C" fn arboretum_finalize() -> bool {
    let client = unsafe { CLIENT.lock() }.unwrap().take();

    match client {
        Some(client) => {
            GLOBAL_RUNTIME
                .block_on(async {
                    client
                        .finalize_sender
                        .send(FfiMessage::Finalize)
                        .await
                        .expect("send the finalize message");
                    client.join_handle.await
                })
                .expect("join on connection handler thread");
            true
        }
        None => false,
    }
}
