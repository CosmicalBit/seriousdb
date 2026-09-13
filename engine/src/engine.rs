use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    path::PathBuf,
};

use pyo3::{PyResult, pyclass, pymethods};

use crate::pages::PAGE_SIZE;

const MAGIC: &[u8; 9] = b"seriousdb";
const FORMAT_VERSION: u32 = 0;
const HEADER_SIZE: usize = MAGIC.len() + 2 * size_of::<u32>() + 2 * size_of::<u64>();

struct Header {
    magic: [u8; 9],
    format_version: u32,
    page_size: u32,
    page_count: u64,
    root_page: u64,
}

impl Header {
    fn new() -> Self {
        Self {
            magic: *MAGIC,
            format_version: FORMAT_VERSION,
            page_size: PAGE_SIZE as u32,
            page_count: 2,
            root_page: 1,
        }
    }
    fn serialize(&self) -> [u8; HEADER_SIZE] {
        let mut buf = [0u8; HEADER_SIZE];
        let mut offset = 0;

        buf[offset..offset + self.magic.len()].copy_from_slice(&self.magic);
        offset += self.magic.len();

        buf[offset..offset + 4].copy_from_slice(&self.format_version.to_be_bytes());
        offset += 4;

        buf[offset..offset + 4].copy_from_slice(&self.page_size.to_be_bytes());
        offset += 4;

        buf[offset..offset + 8].copy_from_slice(&self.page_count.to_be_bytes());
        offset += 8;

        buf[offset..offset + 8].copy_from_slice(&self.root_page.to_be_bytes());

        buf
    }
}

#[pyclass(name = "FileDb")]
pub(crate) struct FileDB {
    _path: PathBuf,
    _file: File,
}

pub trait Engine {
    fn get(&self, key: &[u8]) -> io::Result<Option<Vec<u8>>>;
    fn put(&mut self, key: &[u8], value: &[u8]) -> io::Result<()>;
    fn delete(&mut self, key: &[u8]) -> io::Result<()>;
}

#[pymethods]
impl FileDB {
    #[staticmethod]
    pub fn create(path: PathBuf) -> io::Result<Self> {
        let mut file = OpenOptions::new().read(true).write(true).create_new(true).open(&path)?;

        let header = Header::new();
        let header_serialized = header.serialize();

        let mut header_page = [0u8; PAGE_SIZE];
        header_page[..HEADER_SIZE].copy_from_slice(&header_serialized);

        file.write_all(&header_page)?;
        file.write_all(&[0u8; PAGE_SIZE])?;

        Ok(Self { _path: path, _file: file })
    }
}

#[pymethods]
impl FileDB {
    #[pyo3(name = "get")]
    fn py_get(&self, key: &[u8]) -> PyResult<Option<Vec<u8>>> {
        Ok(Engine::get(self, key)?)
    }

    #[pyo3(name = "put")]
    fn py_put(&mut self, key: &[u8], value: &[u8]) -> PyResult<()> {
        Ok(Engine::put(self, key, value)?)
    }

    #[pyo3(name = "delete")]
    fn py_delete(&mut self, key: &[u8]) -> PyResult<()> {
        Ok(Engine::delete(self, key)?)
    }
}

impl Engine for FileDB {
    fn delete(&mut self, _key: &[u8]) -> io::Result<()> {
        todo!();
    }
    fn get(&self, _key: &[u8]) -> io::Result<Option<Vec<u8>>> {
        todo!();
    }
    fn put(&mut self, _key: &[u8], _value: &[u8]) -> io::Result<()> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_serializes_the_file_format() {
        let serialized = Header::new().serialize();

        assert_eq!(&serialized[..MAGIC.len()], MAGIC);
        assert_eq!(&serialized[9..13], &FORMAT_VERSION.to_be_bytes());
        assert_eq!(&serialized[13..17], &(PAGE_SIZE as u32).to_be_bytes());
        assert_eq!(&serialized[17..25], &2_u64.to_be_bytes());
        assert_eq!(&serialized[25..33], &1_u64.to_be_bytes());
    }
}
