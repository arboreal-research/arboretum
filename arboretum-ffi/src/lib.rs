use once_cell::sync::Lazy;
use serde::Deserialize;
use std::ffi::{c_char, CStr};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    sql::{
        statements::{RelateStatement, UpdateStatement},
        Id, Object, Table, Thing, Values,
    },
    Error, Surreal,
};
use tracing::error;

mod object_builder;
use object_builder::ObjectBuilder;

static RUNTIME: Lazy<tokio::runtime::Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap()
});

pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

#[derive(Debug, Deserialize)]
struct Record {
    id: Thing,
}

#[tracing::instrument]
#[no_mangle]
pub extern "C" fn arboretum_connect_surreal(addr: *const c_char) -> u8 {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).expect("set successfully");

    let result: Result<(), Error> = RUNTIME.block_on(async {
        DB.connect::<Ws>(
            unsafe { CStr::from_ptr(addr) }
                .to_string_lossy()
                .to_string(),
        )
        .await?;

        DB.use_ns("test").use_db("test").await
    });

    if result.is_err() {
        error!(?result);
        0
    } else {
        1
    }
}

#[tracing::instrument]
#[no_mangle]
pub extern "C" fn arboretum_thing_destroy(thing: *mut Thing) {
    let _ = unsafe { Box::from_raw(thing) };
}

#[tracing::instrument]
#[no_mangle]
pub extern "C" fn arboretum_node_new(
    table: *const c_char,
    fields: *mut ObjectBuilder,
) -> *mut Thing {
    let tb = unsafe { CStr::from_ptr(table) }
        .to_string_lossy()
        .to_string();

    let fields = if fields.is_null() {
        None
    } else {
        Some(*unsafe { Box::from_raw(fields) })
    };

    let node: Result<Option<Thing>, Error> = RUNTIME.block_on(async {
        let create = DB.create((tb, Id::ulid()));

        let result: Option<Record> = match fields {
            Some(fields) => create.content(Object(fields.into())).await?,
            None => create.await?,
        };

        match result {
            Some(Record { id }) => Ok(Some(id)),
            None => Ok(None),
        }
    });

    match node {
        Ok(Some(node)) => Box::into_raw(node.into()),
        Ok(None) => {
            error!("Failed to retrieve id after creating node.");
            std::ptr::null_mut()
        }
        Err(e) => {
            error!(?e);
            std::ptr::null_mut()
        }
    }
}

#[tracing::instrument]
#[no_mangle]
pub extern "C" fn arboretum_node_new_with_id(
    table: *const c_char,
    id: *const c_char,
    fields: *mut ObjectBuilder,
) -> *mut Thing {
    let tb = unsafe { CStr::from_ptr(table) }
        .to_string_lossy()
        .to_string();
    let id = Id::String(unsafe { CStr::from_ptr(id) }.to_string_lossy().to_string());

    let fields = if fields.is_null() {
        None
    } else {
        Some(*unsafe { Box::from_raw(fields) })
    };

    let node: Result<Option<Thing>, Error> = RUNTIME.block_on(async {
        let create = DB.insert((tb, id));

        let result: Option<Record> = match fields {
            Some(fields) => create.content(Object(fields.into())).await?,
            None => create.await?,
        };

        match result {
            Some(Record { id }) => Ok(Some(id)),
            None => Ok(None),
        }
    });

    match node {
        Ok(Some(node)) => Box::into_raw(node.into()),
        Ok(None) => {
            error!("Failed to retrieve id after creating node.");
            std::ptr::null_mut()
        }
        Err(e) => {
            error!(?e);
            std::ptr::null_mut()
        }
    }
}

#[tracing::instrument]
#[no_mangle]
pub extern "C" fn arboretum_create_relation(
    left: *const Thing,
    relation: *const c_char,
    right: *const Thing,
    fields: *mut ObjectBuilder,
) {
    let left = unsafe { &*left }.clone();
    let relation = Table(
        unsafe { CStr::from_ptr(relation) }
            .to_string_lossy()
            .to_string(),
    );
    let right = unsafe { &*right }.clone();

    let data = if fields.is_null() {
        None
    } else {
        let fields = *unsafe { Box::from_raw(fields) };
        Some(surrealdb::sql::Data::ContentExpression(
            Object(fields.into()).into(),
        ))
    };

    let result: Result<(), Error> = RUNTIME.block_on(async {
        DB.query(RelateStatement {
            from: left.into(),
            kind: relation.into(),
            with: right.into(),
            data,
            output: Some(surrealdb::sql::Output::None),
            ..RelateStatement::default()
        })
        .await
        .map(|_| ())
    });

    if let Err(e) = result {
        error!(?e);
    }
}

#[tracing::instrument]
#[no_mangle]
pub extern "C" fn aboretum_merge_fields(obj: *const Thing, fields: *mut ObjectBuilder) {
    let obj = unsafe { &*obj }.clone();
    let fields = *unsafe { Box::from_raw(fields) };

    if fields.fields.len() == 0 {
        return;
    }

    let result: Result<(), Error> = RUNTIME.block_on(async {
        DB.query(UpdateStatement {
            what: Values([obj.into()].into()),
            data: Some(surrealdb::sql::Data::MergeExpression(
                Object(fields.into()).into(),
            )),
            output: Some(surrealdb::sql::Output::None),
            ..UpdateStatement::default()
        })
        .await
        .map(|_| ())
    });

    if let Err(e) = result {
        error!(?e);
    }
}
