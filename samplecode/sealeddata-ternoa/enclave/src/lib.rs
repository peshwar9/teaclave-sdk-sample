#![crate_name = "sealdatasssenclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
extern crate sgx_tseal;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate sgx_rand;


#[macro_use]
extern crate serde_derive;
extern crate serde_cbor;


use sgx_types::{sgx_status_t, sgx_sealed_data_t};
use sgx_types::marker::ContiguousMemory;
use sgx_tseal::SgxSealedData;
use sgx_rand::{Rng, StdRng};
use std::vec::Vec;

#[derive(Serialize, Deserialize,Clone, Default, Debug)]
struct TernoaNFTSecretVariable {
    key: u32,
    rand: [u8;16],
    vec: Vec<u8>,
}


#[derive(Copy, Clone,Default,Debug)]
struct TernoaNFTSecretFixed {
    key: u32,
    rand: [u8;16],
}

unsafe impl ContiguousMemory for TernoaNFTSecretFixed{}

// Function to seal data inside enclave
#[no_mangle]
pub extern "C" fn create_sealeddata_for_fixed(sealed_log: * mut u8, sealed_log_size: u32) -> sgx_status_t {

    // Instantiate custom data structure
    let mut data = TernoaNFTSecretFixed::default();
    data.key = 0x1234;

    // Populate custom data structure with random bytes: Note this logic will be replaced in actual implementation
    let mut rand = match StdRng::new() {
        Ok(rng) => rng,
        Err(_) => { return sgx_status_t::SGX_ERROR_UNEXPECTED; },
    };
    rand.fill_bytes(&mut data.rand);

    // Encrypt data and hash it with AES, returning SGXSealedData type
    let aad: [u8;0] = [0_u8;0];
    let result = SgxSealedData::<TernoaNFTSecretFixed>::seal_data(&aad, &data);
    let sealed_data =  match result {
        Ok(x) => x,
        Err(ret) => {return ret},
    };
    // Convert SgxSealedData to sgx_Sealed_data_t buffer 
    let opt = to_sealed_log_for_fixed(&sealed_data, sealed_log, sealed_log_size);
    if opt.is_none() {
        return sgx_status_t::SGX_ERROR_INVALID_PARAMETER
    }
    println!("Data stored is:{:?}",data);
    sgx_status_t::SGX_SUCCESS

}
// Convert SgxSealedData to pointer of sgx_sealed_data_t buffer and return it
fn to_sealed_log_for_fixed<T: Copy + ContiguousMemory>(sealed_data: &SgxSealedData<T>, sealed_log: * mut u8, sealed_log_size: u32) -> Option<* mut sgx_sealed_data_t> {
    unsafe {
        sealed_data.to_raw_sealed_data_t(sealed_log as * mut sgx_sealed_data_t, sealed_log_size)
    }
}

// Convert a pointer of sgx_sealed_data_t buffer to SgxSealedData.
fn from_sealed_log_for_fixed<'a, T: Copy + ContiguousMemory>(sealed_log: * mut u8, sealed_log_size: u32) -> Option<SgxSealedData<'a, T>> {
    unsafe {
        SgxSealedData::<T>::from_raw_sealed_data_t(sealed_log as * mut sgx_sealed_data_t, sealed_log_size)
    }
}

// Function to unseal data within enclave
#[no_mangle]
pub extern "C" fn verify_sealeddata_for_fixed(sealed_log: * mut u8, sealed_log_size: u32) -> sgx_status_t {
    // Convert a pointer of sgx_sealed_data_t buffer to SgxSealedData.
    let opt = from_sealed_log_for_fixed::<TernoaNFTSecretFixed>(sealed_log, sealed_log_size);
    let sealed_data = match opt {
        Some(t) => t,
        None => {return sgx_status_t::SGX_ERROR_INVALID_PARAMETER;},
    };

    let result = sealed_data.unseal_data();
    let unsealed_data = match result {
        Ok(r) => r,
        Err(e) => {return e;},
    };
    let data = unsealed_data.get_decrypt_txt();
    println!("{:?}",data);
    sgx_status_t::SGX_SUCCESS

}