// src/lib.rs

extern "C" {
    fn mdns_query_host(host: *const libc::c_char);
}

use std::ffi::CString;


pub fn mdns_init() {
    println!("mdns_init called");
}

pub fn mdns_deinit() {
    println!("mdns_deinit called");
}

pub fn mdns_query_host_rust(name: &str) {
    let c_name = CString::new(name).expect("Failed to create CString");
    unsafe {
        mdns_query_host(c_name.as_ptr());
    }
}