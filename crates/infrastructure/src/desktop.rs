use std::sync::Arc;

use anyhow::Result;

use crate::storage::data_storage;

pub fn run() -> Result<()> {
    let db = Arc::new(data_storage(None));
    cawr_desktop_egui::run(db)
}
