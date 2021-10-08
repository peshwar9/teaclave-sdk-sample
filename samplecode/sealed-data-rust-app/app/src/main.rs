extern crate sgx_types;
extern crate sgx_urts;
extern crate sgx_tseal;

use sgx_urts::SgxEnclave;
use sgx_types::*;
//use sgx_types::{sgx_enclave_id_t, SgxResult, sgx_launch_token_t, sgx_status_t, sgx_sealed_data_t, sgx_misc_attribute_t, sgx_attributes_t};
use sgx_tseal::SgxSealedData;

static ENCLAVE_FILE: &'static str  = "enclave.signed.so";

extern {
    fn create_sealeddata_for_fixed(eid: sgx_enclave_id_t, retval: *mut sgx_status_t, sealed_log: * mut u8, sealed_log_size: u32) -> sgx_status_t;
    fn verify_sealeddata_for_fixed(eid: sgx_enclave_id_t, retval: *mut sgx_status_t, sealed_log: * mut u8, sealed_log_size: u32) -> sgx_status_t; 

}

// Function to initialize enclave

fn init_enclave() -> SgxResult<SgxEnclave> {

    // setup parameters for creating enclave

    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    let debug = 1;
    let mut misc_attr = sgx_misc_attribute_t { secs_attr: sgx_attributes_t {flags: 0, xfrm: 0}, misc_select: 0 };
    SgxEnclave::create(ENCLAVE_FILE, debug, &mut launch_token, &mut launch_token_updated, &mut misc_attr )
}



// Main  function

fn main() {
    let enclave = match init_enclave() {
        Ok(r) => { println!("Ternoa Enclave initialization successful {}", r.geteid()); r },
        Err(x) => {println!("Error occurred while initializing Ternoa enclave: {}",x.as_str()); return;},
    };

    // Create string to pass to enclave
 //   uint32_t sealed_log_size = 1024;
 //   uint8_t sealed_log[1024] = {0};

    let sealed_log = String::from("This is the normal string passed into Ternoa enclave\n");
    let sealed_log: [u8;1024] = [0_u8;1024];
    let sealed_log_size = 1024_u32;

    let mut retval = sgx_status_t::SGX_SUCCESS;

    // Call the enclave function

    let result = unsafe {
        create_sealeddata_for_fixed(enclave.geteid(), &mut retval, sealed_log.as_ptr() as *mut u8, sealed_log_size)
    };

    match result {
        sgx_status_t::SGX_SUCCESS => println!("Successfully called enclave program from untrusted app"),
        _ => println!("Error in calling enclave function from untrusted app: {}", result.as_str())
    }

    enclave.destroy();

}