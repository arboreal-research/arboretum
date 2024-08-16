use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    ffi::{c_char, CStr},
    sync::{atomic::AtomicU64, Arc, Mutex},
};
use tokio::{
    sync::mpsc::{channel, Sender},
    task::JoinHandle,
};

pub mod tcp_client;
use tcp_client::{tcp_client_worker, ClangPluginClientMessage, FfiValue, TcpClientWorkItem};

struct ConnectionState {
    tx: Sender<TcpClientWorkItem>,
    join_handle: JoinHandle<()>,
    name_assoc: Arc<Mutex<HashMap<String, u64>>>,
    next_id: Arc<AtomicU64>,
}

struct Id(u64);

impl ConnectionState {
    pub fn close(self) {
        tracing::info!("Sending shutdown signal to TCP worker.");

        let tx = self.tx.clone();
        RUNTIME.block_on(async move {
            tx.send(TcpClientWorkItem::Shutdown)
                .await
                .expect("Transport to network worker");

            tracing::info!("Waiting on TCP worker to finish up.");
            self.join_handle.await.expect("joining worker thread");
        });
    }

    pub fn new_node(&self) -> u64 {
        self.next_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }

    pub fn new_node_with_props(&self, props: FfiValue) -> u64 {
        let id = self
            .next_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        let tx = self.tx.clone();
        RUNTIME
            .block_on(async move {
                tx.send(TcpClientWorkItem::Message(
                    ClangPluginClientMessage::NewNodeWithProps(id, props),
                ))
                .await
            })
            .expect("Transport to network worker");
        id
    }

    pub fn new_named_node(&self, s: String) -> u64 {
        let mut lock = self.name_assoc.lock().expect("Locking the name_assoc");
        match lock.entry(s) {
            std::collections::hash_map::Entry::Occupied(o) => *o.get(),
            std::collections::hash_map::Entry::Vacant(v) => {
                let id = self
                    .next_id
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

                let name = v.key().clone();
                let tx = self.tx.clone();
                RUNTIME
                    .block_on(async move {
                        tx.send(TcpClientWorkItem::Message(
                            ClangPluginClientMessage::NewNamedNode(name, id),
                        ))
                        .await
                    })
                    .expect("Transport to network worker");
                v.insert(id);
                id
            }
        }
    }

    pub fn new_named_node_with_props(&self, s: String, props: FfiValue) -> u64 {
        let mut lock = self.name_assoc.lock().expect("Locking the name_assoc");
        match lock.entry(s) {
            std::collections::hash_map::Entry::Occupied(o) => *o.get(),
            std::collections::hash_map::Entry::Vacant(v) => {
                let id = self
                    .next_id
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

                let name = v.key().clone();
                let tx = self.tx.clone();
                RUNTIME
                    .block_on(async move {
                        tx.send(TcpClientWorkItem::Message(
                            ClangPluginClientMessage::NewNamedNodeWithProps(name, id, props),
                        ))
                        .await
                    })
                    .expect("Transport to network worker");
                v.insert(id);
                id
            }
        }
    }

    pub fn new_named_node_with_id(&self, s: String, id: u64) -> u64 {
        let mut lock = self.name_assoc.lock().expect("Locking the name_assoc");
        match lock.entry(s) {
            std::collections::hash_map::Entry::Occupied(o) => *o.get(),
            std::collections::hash_map::Entry::Vacant(v) => {
                let name = v.key().clone();
                let tx = self.tx.clone();
                RUNTIME
                    .block_on(async move {
                        tx.send(TcpClientWorkItem::Message(
                            ClangPluginClientMessage::NewNamedNode(name, id),
                        ))
                        .await
                    })
                    .expect("Transport to network worker");
                v.insert(id);
                id
            }
        }
    }

    pub fn new_named_node_with_id_props(&self, s: String, id: u64, props: FfiValue) -> u64 {
        let mut lock = self.name_assoc.lock().expect("Locking the name_assoc");
        match lock.entry(s) {
            std::collections::hash_map::Entry::Occupied(o) => *o.get(),
            std::collections::hash_map::Entry::Vacant(v) => {
                let name = v.key().clone();
                let tx = self.tx.clone();
                RUNTIME
                    .block_on(async move {
                        tx.send(TcpClientWorkItem::Message(
                            ClangPluginClientMessage::NewNamedNodeWithProps(name, id, props),
                        ))
                        .await
                    })
                    .expect("Transport to network worker");
                v.insert(id);
                id
            }
        }
    }

    pub fn new_edge(&self, sub: u64, pred: u64, obj: u64) {
        let tx = self.tx.clone();
        RUNTIME
            .block_on(async move {
                tx.send(TcpClientWorkItem::Message(
                    ClangPluginClientMessage::NewEdge(sub, pred, obj),
                ))
                .await
            })
            .expect("Transport to network worker");
    }

    pub fn new_edge_with_props(&self, sub: u64, pred: u64, obj: u64, props: FfiValue) {
        let tx = self.tx.clone();
        RUNTIME
            .block_on(async move {
                tx.send(TcpClientWorkItem::Message(
                    ClangPluginClientMessage::NewEdgeWithProps(sub, pred, obj, props),
                ))
                .await
            })
            .expect("Transport to network worker");
    }
}

static mut CONNECTION: Option<ConnectionState> = None;
static RUNTIME: Lazy<tokio::runtime::Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap()
});

#[tracing::instrument]
#[no_mangle]
pub extern "C" fn arboretum_connect(addr: *const c_char) -> u8 {
    let addr = unsafe { CStr::from_ptr(addr) }
        .to_string_lossy()
        .to_string();

    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).expect("set successfully");

    // Setup a channel for communicating with the network thread.
    let (tx, rx) = channel(100000);
    let join_handle = RUNTIME.spawn(async move { tcp_client_worker(addr, rx).await });

    // Set the channel and the join handle into the global variable for access by the other FFI methods.
    unsafe {
        CONNECTION = Some(ConnectionState {
            tx,
            join_handle,
            name_assoc: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(AtomicU64::new(0)),
        });
    }

    1
}

#[tracing::instrument]
#[no_mangle]
pub extern "C" fn arboretum_stop() {
    unsafe {
        CONNECTION.take().map(|c| c.close());
    }
}

#[tracing::instrument]
#[no_mangle]
pub extern "C" fn arboretum_create_value_unsigned(v: u64) -> *mut FfiValue {
    Box::into_raw(Box::new(FfiValue::Unsigned(v)))
}

#[tracing::instrument]
#[no_mangle]
pub extern "C" fn arboretum_create_value_signed(v: i64) -> *mut FfiValue {
    Box::into_raw(Box::new(FfiValue::Signed(v)))
}

#[tracing::instrument]
#[no_mangle]
pub extern "C" fn arboretum_create_value_double(d: f64) -> *mut FfiValue {
    Box::into_raw(Box::new(FfiValue::Double(d)))
}

#[tracing::instrument]
#[no_mangle]
pub extern "C" fn arboretum_create_value_string(s: *const c_char) -> *mut FfiValue {
    let s = unsafe { CStr::from_ptr(s) }.to_string_lossy().to_string();
    Box::into_raw(Box::new(FfiValue::String(s)))
}

/*
  _   _           _
 | \ | |         | |
 |  \| | ___   __| | ___
 | . ` |/ _ \ / _` |/ _ \
 | |\  | (_) | (_| |  __/
 \_| \_/\___/ \__,_|\___|
*/

#[tracing::instrument]
#[no_mangle]
pub unsafe extern "C" fn arboretum_destroy_id(id: *mut Id) {
    let _ = Box::from_raw(id);
}

#[tracing::instrument]
#[no_mangle]
pub extern "C" fn arboretum_create_named_node(name: *const c_char) -> *mut Id {
    let name = unsafe { CStr::from_ptr(name) }
        .to_string_lossy()
        .to_string();
    unsafe {
        Box::into_raw(Box::new(Id(CONNECTION
            .as_ref()
            .unwrap()
            .new_named_node(name))))
    }
}

#[tracing::instrument]
#[no_mangle]
pub extern "C" fn arboretum_create_named_node_with_props(
    name: *const c_char,
    props: *mut FfiValue,
) -> *mut Id {
    let name = unsafe { CStr::from_ptr(name) }
        .to_string_lossy()
        .to_string();

    let props = unsafe { Box::from_raw(props) };

    unsafe {
        Box::into_raw(Box::new(Id(CONNECTION
            .as_ref()
            .unwrap()
            .new_named_node_with_props(name, *props))))
    }
}

#[tracing::instrument]
#[no_mangle]
pub extern "C" fn arboretum_create_named_node_with_id(name: *const c_char, id: u64) -> *mut Id {
    let name = unsafe { CStr::from_ptr(name) }
        .to_string_lossy()
        .to_string();

    unsafe {
        Box::into_raw(Box::new(Id(CONNECTION
            .as_ref()
            .unwrap()
            .new_named_node_with_id(name, id))))
    }
}

#[tracing::instrument]
#[no_mangle]
pub extern "C" fn arboretum_create_named_node_with_id_props(
    name: *const c_char,
    id: u64,
    props: *mut FfiValue,
) -> *mut Id {
    let name = unsafe { CStr::from_ptr(name) }
        .to_string_lossy()
        .to_string();

    let props = unsafe { Box::from_raw(props) };

    unsafe {
        Box::into_raw(Box::new(Id(CONNECTION
            .as_ref()
            .unwrap()
            .new_named_node_with_id_props(name, id, *props))))
    }
}

#[tracing::instrument]
#[no_mangle]
pub unsafe extern "C" fn arboretum_create_nameless_node() -> *mut Id {
    Box::into_raw(Box::new(Id(CONNECTION.as_ref().unwrap().new_node())))
}

#[tracing::instrument]
#[no_mangle]
pub unsafe extern "C" fn arboretum_create_nameless_node_with_props(
    props: *mut FfiValue,
) -> *mut Id {
    let props = unsafe { Box::from_raw(props) };
    Box::into_raw(Box::new(Id(CONNECTION
        .as_ref()
        .unwrap()
        .new_node_with_props(*props))))
}

#[tracing::instrument]
#[no_mangle]
pub extern "C" fn arboretum_create_nameless_node_with_id(id: u64) -> *mut Id {
    Box::into_raw(Box::new(Id(id)))
}

#[tracing::instrument]
#[no_mangle]
pub unsafe extern "C" fn arboretum_create_nameless_node_with_id_props(
    id: u64,
    props: *mut FfiValue,
) -> *mut Id {
    let props = unsafe { Box::from_raw(props) };
    Box::into_raw(Box::new(Id(CONNECTION
        .as_ref()
        .unwrap()
        .new_node_with_props(*props))))
}

// #[tracing::instrument]
// #[no_mangle]
// pub extern "C" fn arboretum_get_named_node(name: *const c_char) -> *mut Ulid {
//     let name = unsafe { CStr::from_ptr(name) }
//         .to_string_lossy()
//         .to_string();
//     Box::into_raw(Box::new(Entity::String(name)))
// }

/*

  _____    _
 |  ___|  | |
 | |__  __| | __ _  ___
 |  __|/ _` |/ _` |/ _ \
 | |__| (_| | (_| |  __/
 \____/\__,_|\__, |\___|
              __/ |
             |___/
*/

#[tracing::instrument]
#[no_mangle]
pub extern "C" fn arboretum_create_edge(
    sub_ptr: *const Id,
    pred_ptr: *const Id,
    obj_ptr: *const Id,
) {
    if sub_ptr.is_null() || pred_ptr.is_null() || obj_ptr.is_null() {
        return;
    }

    let sub;
    let pred;
    let obj;
    unsafe {
        sub = &*sub_ptr;
        pred = &*pred_ptr;
        obj = &*obj_ptr;
    }

    unsafe { CONNECTION.as_ref().unwrap().new_edge(sub.0, pred.0, obj.0) }
}

#[tracing::instrument]
#[no_mangle]
pub extern "C" fn arboretum_create_edge_with_props(
    sub_ptr: *const Id,
    pred_ptr: *const Id,
    obj_ptr: *const Id,
    props: *mut FfiValue,
) {
    if sub_ptr.is_null() || pred_ptr.is_null() || obj_ptr.is_null() {
        return;
    }

    let sub;
    let pred;
    let obj;
    unsafe {
        sub = &*sub_ptr;
        pred = &*pred_ptr;
        obj = &*obj_ptr;
    }

    let props = unsafe { Box::from_raw(props) };

    unsafe {
        CONNECTION
            .as_ref()
            .unwrap()
            .new_edge_with_props(sub.0, pred.0, obj.0, *props)
    }
}
