use cawr_db::json_file::JsonFile;
use directories::UserDirs;
use std::path::{Path, PathBuf};

pub fn data_storage(data_dir: Option<PathBuf>) -> JsonFile {
    let data_dir = data_storage_directory(data_dir);
    log::info!("Use data directory: {data_dir:?}");
    JsonFile::try_new(data_dir).expect("JSON file store")
}

const DEFAULT_STORAGE_DIR_NAME: &str = "clean-architecture-with-rust-data";

// Get storage directory with the following priority:
// 1. Custom (passed by the CLI)
// 2. HOME/DOCUMENTS/clean-architecture-with-rust-data
// 3. HOME/clean-architecture-with-rust-data
// 4. Relative to the executable: ./clean-architecture-with-rust-data
pub fn data_storage_directory(data_dir: Option<PathBuf>) -> PathBuf {
    if let Some(data_dir) = data_dir {
        data_dir
    } else {
        let base_path = if let Some(users_dir) = UserDirs::new() {
            users_dir
                .document_dir()
                .unwrap_or_else(|| users_dir.home_dir())
                .to_path_buf()
        } else {
            Path::new(".").to_path_buf()
        };
        base_path.join(DEFAULT_STORAGE_DIR_NAME)
    }
}
