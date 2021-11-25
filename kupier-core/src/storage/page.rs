/*
Some Reference Documentation On Database Layout:
   * MSSQL: https://docs.microsoft.com/en-us/sql/relational-databases/pages-and-extents-architecture-guide
   * MongoDB: https://docs.mongodb.com/manual/core/gridfs/
   * SqlLite: https://sqlite.org/fileformat.html
*/

use serde_derive;

pub mod PageType {
    pub const None: u8 = 0x00;
    pub const Super: u8 = 0x01;
    pub const Free: u8 = 0x02;
    pub const Leaf: u8 = 0x04;
    pub const Internal: u8 = 0x08;
}

const DESCRIPTOR_PADDING: usize = 23;
const NUM_DESCRIPTOR_SINGLE_BYTES: u32 = 1;
const NUM_DESCRIPTOR_INT_32: u32 = 1;
const NUM_DESCRIPTOR_INT_64: u32 = 3;

/// The header block of a page
#[derive(Serialize, Deserialize)]
pub struct Descriptor {
    /// Indicates what kind of page this is
    pub page_type: u8,

    /// How big in bytes this page is, header excluded.
    pub page_size: u32,

    /// How much of this page is used - also used to calculate where to put the next data entry
    pub bytes_used: u64,

    /// The file offset to the previous linked page
    pub prev_page_start: u64,

    /// The file offset to the next linked page
    pub next_page_start: u64,

    /// 15 bytes of reserved space - future use only
    reserved: [u8; DESCRIPTOR_PADDING]
}

impl Descriptor {
    pub const HEADER_SIZE: u32 = NUM_DESCRIPTOR_SINGLE_BYTES
        + (4 * NUM_DESCRIPTOR_INT_32)
        + (8 * NUM_DESCRIPTOR_INT_64)
        + DESCRIPTOR_PADDING as u32;

    pub fn new(page_type: u8,
               page_size: u32,
               prev_page_start: u64,
               next_page_start: u64) -> Descriptor {

        assert_eq!(page_size % 8, 0, "page_size must be divisible by 2");

        Descriptor {
            page_type,
            page_size,
            bytes_used: 0,
            prev_page_start,
            next_page_start,
            reserved: [0; DESCRIPTOR_PADDING]
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Page {
    header: Descriptor,

    /** This inflates the size by 7 bytes */
    // #[serde(with = "serde_bytes")]
    #[serde(skip_serializing)]
    pub data: Vec<u8>
}

impl Page {
    pub fn get_data_size(&self) -> usize {
        (self.header.page_size - Descriptor::HEADER_SIZE as u32) as usize
    }

    pub fn new(page_type: u8,
               page_size: u32,
               prev_page_start: u64,
               next_page_start: u64) -> Page {

        let mut page = Page {
            header: Descriptor::new(page_type,
                                    page_size,
                                    prev_page_start,
                                    next_page_start),
            data: Vec::new()
        };

        page.data.resize((page_size - Descriptor::HEADER_SIZE) as usize, 0);
        page.data.fill(0);

        return page;
    }
}