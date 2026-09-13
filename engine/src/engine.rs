use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    path::PathBuf,
};

use pyo3::{PyResult, pyclass, pymethods};

const MAGIC: &[u8] = b"seriousdb";

#[pyclass(name = "FileDb")]
pub(crate) struct FileDB {
    path: PathBuf,
    file: File,
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
        let mut file = OpenOptions::new().read(true).write(true).create(true).truncate(false).open(&path)?;

        file.write_all(&MAGIC)?;

        Ok(Self { path, file })
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
    fn delete(&mut self, key: &[u8]) -> io::Result<()> {
        todo!();
    }
    fn get(&self, key: &[u8]) -> io::Result<Option<Vec<u8>>> {
        todo!();
    }
    fn put(&mut self, key: &[u8], value: &[u8]) -> io::Result<()> {
        todo!();
    }
}
