use once_cell::sync::OnceCell;
use std::path::PathBuf;

pub static DB_PATH: OnceCell<PathBuf> = OnceCell::new();