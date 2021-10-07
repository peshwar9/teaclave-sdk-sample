// This is a test program
// It constructs static string, array, vector and then appends all of them into a string and prints it out.

#![crate_name = "helloternoasampleenclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;


use sgx_types::*;
use std::string::String;
use std::vec::Vec;
use std::io::{self, Write};
use std::slice;

#[no_mangle]
pub extern "C" fn say_hello(text: *const u8, length: usize) -> sgx_status_t {
    let str_slice = unsafe {
        slice::from_raw_parts(text,length)
    };

    // Create a sample static string
    let hello_static_str = "Hello Ternoa. I'm inside the SGX enclave";

    // Create an array
    let hello_array : [u8; 4] = [82, 117, 115, 116];

    // Create a vector
    let hello_vec : Vec<u8> = vec![32, 115, 116, 114, 105, 110, 103, 33];

    // construct string from static string
    let mut hello_string = String::from(hello_static_str);

    // Iterate on the array

    for c in hello_array.iter() {
        hello_string.push(*c as char);
    }

    // Append Vector to string

    hello_string += String::from_utf8(hello_vec).expect("Invalid utf-8").as_str();

    // Perform Ocall to print to outside world

    println!("Concatenated string in enclave is: {}", &hello_string);

    // Return success status

    sgx_status_t::SGX_SUCCESS

}