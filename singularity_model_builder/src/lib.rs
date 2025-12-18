use std::path::PathBuf;

mod build_model;
mod download;

pub mod errors;

pub use mistralrs::Model;

#[derive(Debug)]
pub struct Builder {
    cache_dir: PathBuf,
}

impl Builder {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self { cache_dir }
    }
}
