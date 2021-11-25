/*
Some Reference Documentation On Database Layout:
   * MSSQL: https://docs.microsoft.com/en-us/sql/relational-databases/pages-and-extents-architecture-guide
   * MongoDB: https://docs.mongodb.com/manual/core/gridfs/
   * SqlLite: https://sqlite.org/fileformat.html
*/

use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use crate::storage::page::descriptor::Descriptor;
use std::slice::Iter;

pub mod data;
pub mod descriptor;
pub mod super_page;

pub mod PageType {
    pub const Super: u8 = 0x00;
    pub const Free: u8 = 0x01;
    pub const Data: u8 = 0x02;
    pub const Leaf: u8 = 0x04;
    pub const Internal: u8 = 0x08;
}

pub trait Stream: Write + Seek + Read {

}

const PAD: [u8; 1] = [0];

pub trait DynPage {
    fn get_data_size(&self) -> usize;

    fn new(page_size: u16,
           prev_page_start: u64,
           next_page_start: u64) -> Box<dyn DynPage> where Self: Sized;

    fn get_descriptor(&self) -> &Descriptor;

    fn get_data_iter(&self) -> Iter<'_, Vec<u8>>;

    fn encode(&self, mut stream: &File) {
        let descriptor = self.get_descriptor();
        let data_iter = self.get_data_iter();

        bincode::serialize_into(stream, descriptor);

        let mut size: usize = 0;

        for value in data_iter {
            stream.write_all(&value);
            size += value.len();
        }

        let pad = self.get_data_size() - size;
        if pad < 0 {
            panic!("Data Exceeded Page Size! Expected <= {} -> Actual {}", self.get_data_size(), size);
        }

        let old_pos = stream.stream_position().unwrap();

        // This ensures the file is actually extended
        if pad > 0 {
            stream.seek(SeekFrom::Current((pad - 1) as i64));
            stream.write(&PAD);
        }

        // Ensure that this page is flushed to disk ...
        let new_pos = stream.stream_position().unwrap();
        stream.flush();
    }
}

pub trait Page<T> {
    fn get_data_size(&self) -> usize;

    fn new(page_size: u16,
           prev_page_start: u64,
           next_page_start: u64) -> T;
}