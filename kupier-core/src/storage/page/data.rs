use std::fs::File;
use std::io::{Seek, Write};
use std::slice::Iter;
use serde_derive;
use crate::error::{Result};
use crate::storage::page::descriptor::Descriptor;
use crate::storage::page::{DynPage, Page, PageType, Stream};

#[derive(Serialize, Deserialize)]
pub struct DataPage {
    header: Descriptor,

    /** This inflates the size by 7 bytes */
    // #[serde(with = "serde_bytes")]
    #[serde(skip_serializing)]
    pub data: Vec<Vec<u8>>
}

impl DynPage for DataPage {
    fn get_data_size(&self) -> usize {
        (self.header.page_size as u32 - Descriptor::HEADER_SIZE) as usize
    }

    fn new(page_size: u16,
           prev_page_start: u64,
           next_page_start: u64) -> Box<dyn DynPage> {
        let mut page = DataPage {
            header: Descriptor::new(PageType::Data,
                                    page_size,
                                    prev_page_start,
                                    next_page_start),
            data: Vec::new()
        };

        // Fills a vector with empty data ...
        // page.data.resize((page_size as u32 - Descriptor::HEADER_SIZE) as usize, 0);
        // page.data.fill(0);

        return Box::new(page);
    }

    fn get_descriptor(&self) -> &Descriptor {
        return &self.header;
    }

    fn get_data_iter(&self) -> Iter<'_, Vec<u8>> {
        return self.data.iter()
    }
}