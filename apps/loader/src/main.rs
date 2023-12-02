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

use xmas_elf;
use axhal::mem::VirtAddr;
use std::vec::Vec;
use core::ptr;
const PLASH_START: usize = 0x22000000;

fn printf() {
    println!("printf function");
}

fn __libc_start_main() {
    println!("__libc_main_start");
    printf();
}


// struct AppHeader {
//     apps_num: usize,
//     app_size: Vec<usize>,
//     app_start: Vec<*const u8>,
// }

// impl AppHeader {
//     fn read_from_pflash (pflash_start: *const u8) -> Self {
//         let mut offset = 0;
//         let usize_value:&[u8] = unsafe{ core::slice::from_raw_parts(pflash_start.offset(offset as isize), 8) };
//         let apps_num = bytes_to_usize(usize_value);
//         //println!("app num {:?}\n", apps_num);
//         let mut app_size = Vec::new();
//         let mut app_start = Vec::new();
        
//         for _ in 0..apps_num {
//             offset += 8;
//             let value = bytes_to_usize(unsafe{ core::slice::from_raw_parts(pflash_start.offset(offset as isize), 8) });
//             app_size.push(value);
//         }
//         let mut app_start_offset = offset + 8;

//         for i in 0..apps_num {
//             //println!("app {} start at {}\n", i, app_start_offset );
//             app_start.push((PLASH_START + app_start_offset )as *const u8);
//             app_start_offset += app_size[i];
//         }

//         AppHeader { 
//             apps_num,
//             app_size, 
//             app_start,
//         }
//     }
    
// }



#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {

    let pflash_start = PLASH_START as *const u8;
    //let app_info = AppHeader::read_from_pflash(pflash_start);
    let num = 1;
    println!("Load payload ...\n");
    for i in 0..num {
        let app_size = 15528;
        let app_start = pflash_start;
        let code = unsafe {
            core::slice::from_raw_parts(app_start, app_size)
        };
        println!("load app {}, size is {}", i, app_size);
    }
    
    println!("Load payload to pflash_disk ok!\n");

    // switch aspace from kernel to app
    unsafe { init_app_page_table(); }
    unsafe { switch_app_aspace(); }

    // app running aspace
    // SBI(0x8000_0000) -> APP <- Kernel(0x8020_0000)
    // 0xffff_ffc0_0000_0000
    const RUN_START:usize= 0x4010_0000;
    //const RUN_START:usize = 0x0;
    for i in 0..num {
        let app_size = 15528;
        let app_start = pflash_start;
        let load_code = unsafe {
            core::slice::from_raw_parts(app_start, app_size)
        };
        println!("move app {}, size is {}", i, app_size);
        //println!("content: {:?}", load_code);
        let run_code = unsafe {
            core::slice::from_raw_parts_mut(RUN_START as *mut u8, app_size)
        };

    let elf = xmas_elf::ElfFile::new(load_code).unwrap();
    let elf_header = elf.header;
    let magic = elf_header.pt1.magic;
    let entry = elf.header.pt2.entry_point() as usize;
    println!("{:x}", entry);
    assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");
    let ph_count = elf_header.pt2.ph_count();
    let mut offset = 0;
    for i in 0..ph_count {
        let ph = elf.program_header(i).unwrap();
        if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
            let start_va: VirtAddr = (ph.virtual_addr() as usize).into();
            let size = ph.mem_size() as usize;
            let end_va: VirtAddr = ((ph.virtual_addr() + ph.mem_size()) as usize).into();
            //error!("start_va {:x} end_va {:x}", start_va.0, end_va.0);
            println!("start_va {:x} end_va {:x}", start_va, end_va);
            // Ensure that the size does not exceed the capacity of run_code
            assert!(offset + size <= run_code.len());
            //计算 data 应该放在run_code的哪个 offset 的位置
            println!("offset {}  size {}", offset, size);
            // 将数据从 load_code copy 到 run_code 的 offset 的 位置上
            let load_code_slice = &load_code[ph.offset() as usize..(ph.offset() as usize + size) as usize];
            let run_code_slice = &mut run_code[offset..(offset + size)];  

            // Use unsafe pointer-based copy for efficiency
            unsafe {
                ptr::copy_nonoverlapping(load_code_slice.as_ptr(), run_code_slice.as_mut_ptr(), size);
            }   

            offset += size;       

            // 输出当前加载的段的信息
            println!(
                "Loaded segment {}: start_va={:x}, size={}, offset={}, next_offset={}",
                i,
                ph.virtual_addr(),
                size,
                offset - size,
                offset
            );
            // 输出加载的数据内容
            //println!("Data content: {:?}", run_code_slice);
        }
    }
        let offset_printf = printf as *const() as usize;
        let offset_libc_start_main = __libc_start_main as *const() as usize;
        let address_printf= unsafe{
            core::slice::from_raw_parts_mut((RUN_START + 0x2018) as *mut usize, 1)
        };
        address_printf[0] = offset_printf;
        let address_libc_start_main= unsafe{
            core::slice::from_raw_parts_mut((RUN_START + 0x2020) as *mut usize, 1)
        };
        address_libc_start_main[0] = offset_libc_start_main;

        println!("Execute app ...\n");

        //li      t3, 0
        //mv      a0, t3
        // execute app
        // li      t4, 0
        // mv      a2, t4
        // li      t3, 0
        // mv      a1, t3
        unsafe { core::arch::asm!("
            li      t2, {run_start}
            add     t2, t2, {entry}
            jalr    t2",
            run_start = const RUN_START,
            entry = in(reg) entry,
        )}
        // 清除 run_code 中的内容，将所有字节设为 0
        let clear_value = 0;
        unsafe {
            ptr::write_bytes(run_code.as_mut_ptr(), clear_value, run_code.len());
        }
    }

}

//
// App aspace
//

#[link_section = ".data.app_page_table"]
static mut APP_PT_SV39: [u64; 512] = [0; 512];

unsafe fn init_app_page_table() {
    // 0x8000_0000..0xc000_0000, VRWX_GAD, 1G block
    APP_PT_SV39[2] = (0x80000 << 10) | 0xef;
    // 0xffff_ffc0_8000_0000..0xffff_ffc0_c000_0000, VRWX_GAD, 1G block
    APP_PT_SV39[0x102] = (0x80000 << 10) | 0xef;

    // 0x0000_0000..0x4000_0000, VRWX_GAD, 1G block
    APP_PT_SV39[0] = (0x00000 << 10) | 0xef;

    // For App aspace!
    // 0x4000_0000..0x8000_0000, VRWX_GAD, 1G block
    APP_PT_SV39[1] = (0x80000 << 10) | 0xef;
}

unsafe fn switch_app_aspace() {
    use riscv::register::satp;
    let page_table_root = APP_PT_SV39.as_ptr() as usize - axconfig::PHYS_VIRT_OFFSET;
    satp::set(satp::Mode::Sv39, 0, page_table_root >> 12);
    riscv::asm::sfence_vma_all();
}

#[inline]
fn bytes_to_usize(bytes: &[u8]) -> usize {
    usize::from_be_bytes(bytes.try_into().unwrap())
}

