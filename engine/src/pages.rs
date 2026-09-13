use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom, Write},
};

pub const PAGE_SIZE: usize = 4096;

#[derive(Clone, Copy, Debug)]
pub struct PageId(pub u64);

pub type Page = [u8; PAGE_SIZE];

pub struct FilePager {
    file: File,
}

impl FilePager {
    pub fn new(file: File) -> Self {
        Self { file }
    }

    /// Reads a page from a [`PageId`].
    pub fn read_page(&mut self, page_id: PageId) -> io::Result<Page> {
        let offset = page_offset(page_id)?;

        self.file.seek(SeekFrom::Start(offset))?;

        let mut page = [0u8; PAGE_SIZE];
        self.file.read_exact(&mut page)?;

        Ok(page)
    }

    /// Writes a [`Page`] to a provided [`PageId`].
    pub fn write_page(&mut self, page_id: PageId, page: &Page) -> io::Result<()> {
        let offset = page_offset(page_id)?;

        self.file.seek(SeekFrom::Start(offset))?;
        self.file.write_all(page)?;

        Ok(())
    }
}

fn page_offset(page_id: PageId) -> io::Result<u64> {
    page_id
        .0
        .checked_mul(PAGE_SIZE as u64)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "page offset overflow"))
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    fn temporary_file() -> (File, std::path::PathBuf) {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after the Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("seriousdb-pager-test-{}-{unique}", std::process::id()));
        let file = File::options()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&path)
            .expect("temporary pager file should be created");

        (file, path)
    }

    #[test]
    fn writes_and_reads_a_page() {
        let (file, path) = temporary_file();
        let mut pager = FilePager::new(file);
        let page = [0x5a; PAGE_SIZE];

        pager.write_page(PageId(2), &page).unwrap();
        assert_eq!(pager.read_page(PageId(2)).unwrap(), page);

        drop(pager);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn page_offset_rejects_overflow() {
        let error = page_offset(PageId(u64::MAX)).unwrap_err();

        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
    }
}
