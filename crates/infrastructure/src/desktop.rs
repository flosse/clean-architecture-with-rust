use crate::storage::data_storage;
use std::sync::Arc;

pub fn run() {
    let db = Arc::new(data_storage(None));
    cawr_desktop::run(db);
}
