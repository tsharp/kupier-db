use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::{Arc, RwLock};
use bson::Document;
use crate::storage::page::{Descriptor, Page, PageType};
use crate::error::{Error, Result};

pub mod EngineConfig {
    /// Default Page Size of 64KB
    pub const DEFAULT_PAGE_SIZE: u32 = 65536; // 8KB Page Size, 128 per MB
}

pub struct StorageEngine {
    super_page: Arc<RwLock<Box<Page>>>,
    pages: Arc<RwLock<HashMap<usize, Page>>>,
    db_file: File
}

impl StorageEngine {
    pub fn new(path: &str) -> StorageEngine {
        let super_page_arc = Arc::new(
            RwLock::new(
                Box::new(
                    Page::new(PageType::Super,
                                 EngineConfig::DEFAULT_PAGE_SIZE,
                                 0,
                                 0)
                )));

        let storage = StorageEngine {
            super_page: super_page_arc,
            pages: Arc::new(RwLock::new(HashMap::new())),
            db_file: File::create(path).unwrap()
        };

        {
            let page_lock = storage.super_page.write();
            let mut page_box = page_lock.unwrap();
            let page = page_box.as_mut();
        }

        return storage;
    }

    pub fn save(&mut self) {
        let locked_page_data = self.super_page.read().unwrap();
        let boxed_page_data = &*locked_page_data;
        let page_data = boxed_page_data.as_ref();
        bincode::serialize_into(&mut self.db_file, &page_data);
        self.db_file.write_all(&page_data.data);
        self.db_file.flush();
    }

    pub fn read_page_header(mut db_file: &File) -> Result<Descriptor> {

        let mut buf: [u8; Descriptor::HEADER_SIZE as usize] =
            [0; Descriptor::HEADER_SIZE  as usize];

        db_file.read_exact(&mut buf)?;
        db_file.seek(SeekFrom::Current(-(Descriptor::HEADER_SIZE as i64)));
        let page_header: Option<Descriptor> = bincode::deserialize(&buf[..]).unwrap();

        Ok(page_header.unwrap())
    }

    pub fn load(path: &str) -> Result<StorageEngine> {
        let mut db_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .unwrap();

        let super_header =
            StorageEngine::read_page_header(&db_file).unwrap();

        if super_header.page_type == PageType::Super {
            println!("SUPER_HEADER!");
        }

        if super_header.bytes_used == 0 {
            println!("EMPTY SUPER PAGE!");
        }

        // Read and deserialize the super page ..
        let mut buf: [u8; EngineConfig::DEFAULT_PAGE_SIZE as usize] = [0; EngineConfig::DEFAULT_PAGE_SIZE as usize];
        db_file.read_exact(&mut buf)?;
        let super_page: Option<Page> = bincode::deserialize(&buf[..]).unwrap();
        let super_page_arc = Arc::new(RwLock::new(Box::new(super_page.unwrap())));

        Ok(StorageEngine  {
            super_page: super_page_arc,
            pages: Arc::new(RwLock::new(HashMap::new())),
            db_file
        })
    }

    /// Returns where in the file the given
    fn calculate_location(&self, ) {

    }

    fn allocate_pages(&self, number: u32) {
        /*
        let prev_page_start = 0;
        let next_page_start:
        for idx in 0..number {
            self.pages.write().unwrap().insert(2, Page::new(PageType::Super,
                                                         Config::DEFAULT_PAGE_SIZE,
                                                         0,
                                                         0,
                                                         0));
        }
         */
    }

    pub fn insert(&self, doc: Document) {
        let locked_page_data = self.super_page.read().unwrap();
        let boxed_page_data = &*locked_page_data;
        let page_data = boxed_page_data.as_ref();

        let mut byte_array : Vec<u8> = vec![];
        doc.to_writer(&mut byte_array);
        println!("{}", byte_array.len() % page_data.get_data_size());

        // .data.append(&mut byte_array);
    }
}