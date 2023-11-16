#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;
//use crate::AppHeader;
#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

use std::vec::Vec;
use core::ptr;
const PLASH_START: usize = 0x22000000;

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
    
    println!("Load payload ok!");

}

#[inline]
fn bytes_to_usize(bytes: &[u8]) -> usize {
    usize::from_be_bytes(bytes.try_into().unwrap())
}

