use once_cell::sync::OnceCell;
use std::path::PathBuf;

pub static OUIS_DB_PATH: OnceCell<PathBuf> = OnceCell::new();