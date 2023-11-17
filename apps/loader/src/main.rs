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

use std::vec::Vec;
use core::ptr;
const PLASH_START: usize = 0x22000000;

const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_TERMINATE: usize = 3;

static mut ABI_TABLE: [usize; 16] = [0; 16];

fn register_abi(num: usize, handle: usize) {
    unsafe { ABI_TABLE[num] = handle; }
}

fn abi_hello() {
    println!("[ABI:Hello] Hello, Apps!");
    println!("");
}

fn abi_putchar(c: char) {
    println!("[ABI:Print] {c}");
}

fn abi_terminate() {
    println!("[ABI:Terminate]");
    terminate();

}


struct AppHeader {
    apps_num: usize,
    app_size: Vec<usize>,
    app_start: Vec<*const u8>,
}

impl AppHeader {
    fn read_from_pflash (pflash_start: *const u8) -> Self {
        let mut offset = 0;
        let usize_value:&[u8] = unsafe{ core::slice::from_raw_parts(pflash_start.offset(offset as isize), 8) };
        let apps_num = bytes_to_usize(usize_value);
        //println!("app num {:?}\n", apps_num);
        let mut app_size = Vec::new();
        let mut app_start = Vec::new();
        
        for _ in 0..apps_num {
            offset += 8;
            let value = bytes_to_usize(unsafe{ core::slice::from_raw_parts(pflash_start.offset(offset as isize), 8) });
            app_size.push(value);
        }
        let mut app_start_offset = offset + 8;

        for i in 0..apps_num {
            //println!("app {} start at {}\n", i, app_start_offset );
            app_start.push((PLASH_START + app_start_offset )as *const u8);
            app_start_offset += app_size[i];
        }

        AppHeader { 
            apps_num,
            app_size, 
            app_start,
        }
    }
    
}



#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    register_abi(SYS_HELLO, abi_hello as usize);
    register_abi(SYS_PUTCHAR, abi_putchar as usize);
    register_abi(SYS_TERMINATE, abi_terminate as usize);

    println!("Execute app ...");
    let arg0: u8 = b'A';


    let pflash_start = PLASH_START as *const u8;
    let app_info = AppHeader::read_from_pflash(pflash_start);
    let num = app_info.apps_num;
    println!("Load payload ...\n");
    for i in 0..num {
        let app_size = app_info.app_size[i];
        let app_start = app_info.app_start[i];
        let code = unsafe {
            core::slice::from_raw_parts(app_start, app_size)
        };
        println!("load app {}, size is {}", i, app_size);
        println!("content: {:?}\n", code);
    }
    
    println!("Load payload to pflash_disk ok!\n");
    
    // app running aspace
    // SBI(0x8000_0000) -> APP <- Kernel(0x8020_0000)
    // 0xffff_ffc0_0000_0000
    const RUN_START:usize= 0xffff_ffc0_8010_0000;

    for i in 0..num {
        let app_size = app_info.app_size[i];
        let app_start = app_info.app_start[i];
        let load_code = unsafe {
            core::slice::from_raw_parts(app_start, app_size)
        };
        println!("move app {}, size is {}", i, app_size);
        println!("content: {:?}", load_code);
        let run_code = unsafe {
            core::slice::from_raw_parts_mut(RUN_START as *mut u8, app_size)
        };
        run_code.copy_from_slice(load_code);
        println!("run code {:?}; address [{:?}]", run_code, run_code.as_ptr());
        println!("Execute app ...\n");

        // execute app
        unsafe { core::arch::asm!("
            la      a0, {abi_table}
            li      t2, {run_start}
            jalr    t2",
            run_start = const RUN_START,
            abi_table = sym ABI_TABLE,
            clobber_abi("C")
        )}
        // 清除 run_code 中的内容，将所有字节设为 0
        let clear_value = 0;
        unsafe {
            ptr::write_bytes(run_code.as_mut_ptr(), clear_value, run_code.len());
        }
    }

}

#[inline]
fn bytes_to_usize(bytes: &[u8]) -> usize {
    usize::from_be_bytes(bytes.try_into().unwrap())
}

