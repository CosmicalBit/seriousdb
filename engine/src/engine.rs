use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;
use std::{io::Error, path::PathBuf};

const MAGIC: &[u8] = b"seriousdb";

struct FileDB {
    path: PathBuf,
    file: File,
}

pub trait Engine {
    fn get(&self, key: &[u8]) -> io::Result<Option<Vec<u8>>>;
    fn put(&mut self, key: &[u8], value: &[u8]) -> io::Result<()>;
    fn delete(&mut self, key: &[u8]) -> io::Result<()>;
}

impl FileDB {
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();

        let file = OpenOptions::new().read(true).write(true).create(true).open(&path)?;

        Ok(Self { path, file })
    }
}
