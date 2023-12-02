#![feature(asm_const)]
#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;
//use crate::AppHeader;
#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;
use axhal::misc::terminate;
use axhal::arch::init_app_page_table;

use xmas_elf;
use axhal::mem::VirtAddr;
use std::vec::Vec;
use std::vm;
use core::ptr;
use memory_addr::{PAGE_SIZE_4K, align_down_4k, align_up_4k};

const PAGE_SHIFT: usize = 12;
const PLASH_START: usize = 0xffff_ffc0_2200_0000;

fn printf() {
    println!("printf function");
}

fn find_main() {
    unimplemented!();
}
fn __libc_start_main() {
    println!("__libc_main_start");
    find_main();
}


#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {

    let pflash_start = PLASH_START as *const u8;
    let num = 1;
    println!("Load payload ...\n");
    for i in 0..num {
        let app_size = 457208;
        let app_start = pflash_start;
        let code = unsafe {
            core::slice::from_raw_parts(app_start, app_size)
        };
        println!("load app {}, size is {}", i, app_size);
    }
    
    println!("Load payload to pflash_disk ok!\n");

    // switch aspace from kernel to app
    unsafe { init_app_page_table(); }

    // parse elf 
    let app_size = 457208;
    let app_start = pflash_start;
    let load_code = unsafe {
        core::slice::from_raw_parts(app_start, app_size)
    };
    println!("move app 0, size is {}", app_size);

    let elf = xmas_elf::ElfFile::new(load_code).unwrap();
    let elf_header = elf.header;
    let magic = elf_header.pt1.magic;
    let entry = elf.header.pt2.entry_point() as usize;
    println!("{:x}", entry);
    assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");

    let ph_count = elf_header.pt2.ph_count();
    for i in 0..ph_count {
        let ph = elf.program_header(i).unwrap();
        if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
            let size = ph.mem_size() as usize;
            let va_end = align_up_4k((ph.virtual_addr() + ph.mem_size()) as usize);
            let va = align_down_4k(ph.virtual_addr() as usize);
            let num_pages = (va_end - va) >> PAGE_SHIFT;
            let pa = vm::alloc_pages(num_pages, PAGE_SIZE_4K);
            println!("va: {:#x} pa: {:#x} num {}", ph.virtual_addr(), pa, num_pages);

            vm::map_region(va, pa, num_pages << PAGE_SHIFT, vm::READ | vm::WRITE | vm::EXECUTE);


            let load_code_slice = &load_code[ph.offset() as usize..(ph.offset() as usize + size) as usize];
            let run_code_slice = unsafe{ core::slice::from_raw_parts_mut(ph.virtual_addr() as *mut u8, size) };

            // Use unsafe pointer-based copy for efficiency
            unsafe {
                ptr::copy_nonoverlapping(load_code_slice.as_ptr(), run_code_slice.as_mut_ptr(), size);
            }   
        }
    }
        // let offset_printf = printf as *const() as usize;
        // let offset_libc_start_main = __libc_start_main as *const() as usize;
        // let address_printf= unsafe{
        //     core::slice::from_raw_parts_mut((0x2018) as *mut usize, 1)
        // };
        // address_printf[0] = offset_printf;
        // let address_libc_start_main= unsafe{
        //     core::slice::from_raw_parts_mut((0x2020) as *mut usize, 1)
        // };
        // address_libc_start_main[0] = offset_libc_start_main;

        println!("Execute app ...\n");

        unsafe { core::arch::asm!("
            mv      t2, {entry}
            jalr    t2",
            entry = in(reg) entry,
        )}


}

