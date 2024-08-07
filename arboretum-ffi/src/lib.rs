use once_cell::sync::Lazy;

use simple_triplestore::{
    mem::MemHashIndex,
    prelude::*,
    rdf::{Entity, RdfTripleStore, RdfTripleStoreError},
    MemTripleStore, Triple, UlidIdGenerator,
};
use std::{
    ffi::{c_char, CStr},
    sync::{Arc, RwLock},
};
use ulid::Ulid;

// static RUNTIME: Lazy<tokio::runtime::Runtime> = Lazy::new(|| {
//     tokio::runtime::Builder::new_multi_thread()
//         .worker_threads(2)
//         .enable_all()
//         .build()
//         .unwrap()
// });

type NodeProps = ();
type EdgeProps = ();

type MemRdfNameIndex = MemHashIndex<String, Ulid>;
type MemRdfTripleStoreImpl = MemTripleStore<Ulid, NodeProps, EdgeProps>;

type MemRdfTripleStore =
    RdfTripleStore<NodeProps, EdgeProps, MemRdfNameIndex, MemRdfTripleStoreImpl>;

static GRAPH: Lazy<Arc<RwLock<MemRdfTripleStore>>> = Lazy::new(|| {
    Arc::new(RwLock::new(RdfTripleStore::new(
        MemHashIndex::new(),
        MemTripleStore::new(UlidIdGenerator::new()),
    )))
});

#[tracing::instrument]
#[no_mangle]
pub extern "C" fn arboretum_connect(addr: *const c_char) -> u8 {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).expect("set successfully");
    1
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
pub extern "C" fn arboretum_create_named_node(name: *const c_char) -> *mut Entity {
    fn inner(name: String) -> Result<*mut Entity, <MemRdfTripleStore as TripleStoreError>::Error> {
        let entity;
        {
            let mut lock = GRAPH.write().unwrap();

            let maybe_id = lock
                .get_name_index()
                .left_to_right(&name)
                .map_err(|e| RdfTripleStoreError::NameIndexStorageError(e))?;

            match maybe_id {
                Some(id) => {
                    entity = Entity::Ulid(id);
                }
                None => {
                    let id = Ulid::new();

                    entity = Entity::Ulid(id);

                    lock.get_name_index_mut()
                        .set(name, id)
                        .map_err(|e| RdfTripleStoreError::NameIndexStorageError(e))?;

                    lock.insert_node(entity.clone(), ())?;
                }
            }
        }
        Ok(Box::into_raw(Box::new(entity)))
    }

    let name = unsafe { CStr::from_ptr(name) }
        .to_string_lossy()
        .to_string();

    match inner(name) {
        Err(e) => {
            tracing::error!(?e);
            std::ptr::null_mut()
        }
        Ok(ptr) => ptr,
    }
}

#[tracing::instrument]
#[no_mangle]
pub extern "C" fn arboretum_create_named_node_with_id(
    name: *const c_char,
    id_hi: u64,
    id_lo: u64,
) -> *mut Entity {
    fn inner(
        name: String,
        id: u128,
    ) -> Result<*mut Entity, <MemRdfTripleStore as TripleStoreError>::Error> {
        let id = Ulid(id);
        let entity = Entity::Ulid(id);

        {
            let mut lock = GRAPH.write().unwrap();

            lock.get_name_index_mut()
                .set(name, id)
                .map_err(|e| RdfTripleStoreError::NameIndexStorageError(e))?;

            lock.insert_node(entity.clone(), ())?;
        }
        Ok(Box::into_raw(Box::new(entity)))
    }

    let name = unsafe { CStr::from_ptr(name) }
        .to_string_lossy()
        .to_string();

    let id = ((id_hi as u128) << 64) + (id_lo as u128);

    match inner(name, id) {
        Err(e) => {
            tracing::error!(?e);
            std::ptr::null_mut()
        }
        Ok(ptr) => ptr,
    }
}

#[tracing::instrument]
#[no_mangle]
pub extern "C" fn arboretum_create_nameless_node() -> *mut Entity {
    fn inner() -> Result<*mut Entity, <MemRdfTripleStore as TripleStoreError>::Error> {
        let entity = Entity::Ulid(Ulid::new());

        {
            let mut lock = GRAPH.write().unwrap();
            lock.insert_node(entity.clone(), ())?;
        }

        Ok(Box::into_raw(Box::new(entity)))
    }

    match inner() {
        Err(e) => {
            tracing::error!(?e);
            std::ptr::null_mut()
        }
        Ok(ptr) => ptr,
    }
}

#[tracing::instrument]
#[no_mangle]
pub extern "C" fn arboretum_get_named_node(name: *const c_char) -> *mut Entity {
    let name = unsafe { CStr::from_ptr(name) }
        .to_string_lossy()
        .to_string();
    Box::into_raw(Box::new(Entity::String(name)))
}

#[tracing::instrument]
#[no_mangle]
pub extern "C" fn arboretum_get_ulid_node(id_hi: u64, id_lo: u64) -> *mut Entity {
    let id = ((id_hi as u128) << 64) + (id_lo as u128);
    Box::into_raw(Box::new(Entity::Ulid(Ulid(id))))
}

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
    sub_ptr: *const Entity,
    pred_ptr: *const Entity,
    obj_ptr: *const Entity,
) {
    if sub_ptr.is_null() || pred_ptr.is_null() || obj_ptr.is_null() {
        return;
    }

    fn inner(
        sub: Entity,
        pred: Entity,
        obj: Entity,
    ) -> Result<(), <MemRdfTripleStore as TripleStoreError>::Error> {
        let mut lock = GRAPH.write().unwrap();
        lock.insert_edge(Triple { sub, pred, obj }, ())?;
        Ok(())
    }

    let sub;
    let pred;
    let obj;
    unsafe {
        sub = (&*sub_ptr).clone();
        pred = (&*pred_ptr).clone();
        obj = (&*obj_ptr).clone();
    }

    if let Err(e) = inner(sub, pred, obj) {
        tracing::error!(?e);
    }
}

/**
  _____      _                        _
 |_   _|    | |                      | |
   | | _ __ | |_ ___ _ __ _ __   __ _| |
   | || '_ \| __/ _ \ '__| '_ \ / _` | |
  _| || | | | ||  __/ |  | | | | (_| | |
  \___/_| |_|\__\___|_|  |_| |_|\__,_|_|
*/

#[tracing::instrument]
#[no_mangle]
pub extern "C" fn arboretum_dump() {
    fn inner() -> Result<(), <MemRdfTripleStore as TripleStoreError>::Error> {
        let lock = GRAPH.read().unwrap();

        let edges = lock
            .iter_edges(simple_triplestore::EdgeOrder::SPO)
            .collect::<Result<Vec<_>, _>>()?;

        for (Triple { sub, pred, obj }, _) in edges {
            tracing::info!(?sub, ?pred, ?obj);
        }

        Ok(())
    }

    if let Err(e) = inner() {
        tracing::error!(?e);
    }
}
