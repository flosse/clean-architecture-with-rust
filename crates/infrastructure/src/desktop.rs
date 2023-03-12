use crate::storage::data_storage;
use anyhow::Result;
use std::sync::Arc;

pub fn run() -> Result<()> {
    let db = Arc::new(data_storage(None));
    cawr_desktop::run(db)
}
