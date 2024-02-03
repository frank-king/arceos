#![no_std]
#![feature(let_chains)]

extern crate alloc;

use alloc::vec::Vec;
use byteorder::{ByteOrder, BE};
use fdt_rs::base::DevTree;
use fdt_rs::error::Result;
use fdt_rs::prelude::*;

static VIRTIO_MMIO_NODE: &str = "virtio_mmio";
static REG_PROP: &str = "reg";

pub struct DtbInfo {
    pub memory_addr: usize,
    pub memory_size: usize,
    pub mmio_regions: Vec<(usize, usize)>,
}

pub fn parse_dtb(dtb_pa: usize) -> Result<DtbInfo> {
    let dtb = unsafe { DevTree::from_raw_pointer(dtb_pa as *const u8) }?;
    let memory_addr = dtb_pa;
    let memory_size = dtb.totalsize();
    let mut mmio_regions = Vec::new();
    let mut iter = dtb.nodes();
    while let Some(node) = iter.next()? {
        if node.name()?.starts_with(VIRTIO_MMIO_NODE) {
            let mut props = node.props();
            while let Some(prop) = props.next()? {
                if prop.name()? == REG_PROP {
                    let addr = BE::read_u32(&prop.raw()[4..]);
                    let size = BE::read_u32(&prop.raw()[12..]);
                    mmio_regions.push((addr as usize, size as usize));
                }
            }
        }
    }
    Ok(DtbInfo {
        memory_addr,
        memory_size,
        mmio_regions,
    })
}
