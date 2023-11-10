#![no_std]
extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use core::mem;
use core::result::Result;
use core::result::Result::Ok;
use fdt::{Fdt, FdtError};

use axlog::info;
// 类型定义
pub struct DtbInfo {
    pub memory_addr: usize,
    pub memory_size: usize,
    pub mmio_regions: Vec<(usize, usize)>,
}

// 参考函数原型
pub fn parse_dtb(dtb_pa: usize) -> Result<DtbInfo, FdtError> {
    unsafe {
        let fdt = Fdt::from_ptr(dtb_pa as *const u8)?;

        let memory_addr = fdt.memory().regions().next().unwrap().starting_address as usize;
        let memory_size = fdt.memory().regions().next().unwrap().size.expect("REASON");

        let mmio_regions: Vec<(usize, usize)> = vec![];
        // for node in fdt.all_nodes() {

        let mut mmio_regions = vec![];

        for node in fdt.all_nodes() {
            // 提取节点名称
            // 使用 '@' 分割节点名称
            let parts: Vec<&str> = node.name.split('@').collect();

            // 如果前一半是 "virtio_mmio"，则提取地址和长度
            if let Some(name) = parts.get(0) {
                if *name == "virtio_mmio" {
                    // 使用 raw_reg 方法获取地址和长度
                    if let Some(iter) = node.raw_reg() {
                        // 迭代 RawReg 实例并添加到 mmio_regions 数组
                        mmio_regions.extend(iter.map(|raw_reg| {
                            (
                                usize::from_le_bytes(
                                    raw_reg
                                        .address
                                        .try_into()
                                        .unwrap_or_else(|_| [0; core::mem::size_of::<usize>()]),
                                ),
                                usize::from_le_bytes(
                                    raw_reg
                                        .size
                                        .try_into()
                                        .unwrap_or_else(|_| [0; core::mem::size_of::<usize>()]),
                                ),
                            )
                        }));
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
}
