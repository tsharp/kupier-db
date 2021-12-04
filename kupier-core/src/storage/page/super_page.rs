use std::fs::File;
use std::io::{Seek, Write};
use std::slice::Iter;
use serde_derive;
use crate::error::{Result};
use crate::storage::page::descriptor::Descriptor;
use crate::storage::page::{DynPage, Page, PageType, Stream};

pub struct SuperPageData {
    /// Location of the next free list entry, 0 if none assigned.
    pub free_list_start: u64,

    /// Location of the first linked data node, 0 if none is set
    pub data_list_start: u64

    // Note that entries must be keyed
}

#[derive(Serialize, Deserialize)]
pub struct SuperPage {
    header: Descriptor,

    #[serde(skip_serializing)]
    pub data: SuperPageData
}

impl DynPage for SuperPage {
    fn get_data_size(&self) -> usize {
        (self.header.page_size as u32 - Descriptor::HEADER_SIZE) as usize
    }

    fn new(page_size: u16,
           prev_page_start: u64,
           next_page_start: u64) -> Box<dyn DynPage> {
        let mut page = SuperPage {
            header: Descriptor::new(PageType::Super,
                                    page_size,
                                    prev_page_start,
                                    next_page_start),
            data: SuperPageData {
                data_list_start: 0,
                free_list_start: 0
            }
        };

        return Box::new(page);
    }

    fn get_descriptor(&self) -> &Descriptor {
        &self.header
    }

    fn get_data_iter(&self) -> Iter<'_, Vec<u8>> {
        self.data.iter()
    }
}