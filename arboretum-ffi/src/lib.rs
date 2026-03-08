use once_cell::sync::Lazy;
use reify_rs::io::TableBuilders;
use std::cell::RefCell;
use std::ffi::c_char;

// Global runtime for running async code from sync FFI functions
static GLOBAL_RUNTIME: Lazy<tokio::runtime::Runtime> =
    Lazy::new(|| tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime"));

thread_local!(static TABLE_BUILDERS: RefCell<Option<TableBuilders>> = RefCell::new(None));

#[no_mangle]
pub extern "C" fn arboretum_connect(url: *const c_char) -> bool {
    let url = match unsafe { std::ffi::CStr::from_ptr(url).to_str() } {
        Ok(s) => s.to_owned(),
        Err(_) => return false,
    };

    // Create table builders with PostgreSQL connection
    TABLE_BUILDERS.with(|tb| {
        *tb.borrow_mut() = Some(TableBuilders::new(&url));
    });
    
    true
}

#[no_mangle]
pub extern "C" fn arboretum_subgraph_id() -> u64 {
    0
}

#[no_mangle]
pub extern "C" fn arboretum_finalize() -> bool {
    // Flush all pending data to PostgreSQL
    let result = TABLE_BUILDERS.with(|tb| {
        if let Some(mut tb) = tb.borrow_mut().take() {
            GLOBAL_RUNTIME.block_on(tb.flush())
        } else {
            Ok(())
        }
    });

    result.is_ok()
}

#[no_mangle]
pub extern "C" fn arboretum_set_db_url(url: *const c_char) -> bool {
    let url = match unsafe { std::ffi::CStr::from_ptr(url).to_str() } {
        Ok(s) => s.to_owned(),
        Err(_) => return false,
    };

    TABLE_BUILDERS.with(|tb| {
        *tb.borrow_mut() = Some(TableBuilders::new(&url));
    });
    
    true
}
