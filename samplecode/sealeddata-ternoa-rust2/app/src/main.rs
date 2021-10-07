extern crate sgx_types;
extern crate sgx_urts;

use sgx_types::*;
use sgx_urts::SgxEnclave;

static ENCLAVE_FILE: &'static str  = "enclave.signed.so";

extern {
    fn say_hello(eid: sgx_enclave_id_t, retval: *mut sgx_status_t, text: *const u8, length: usize) -> sgx_status_t;
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

    let input_string = String::from("This is the normal string passed into Ternoa enclave\n");
    let mut retval = sgx_status_t::SGX_SUCCESS;

    // Call the enclave function

    let result = unsafe {
        say_hello(enclave.geteid(), &mut retval, input_string.as_ptr() as * const u8, input_string.len())
    };

    match result {
        sgx_status_t::SGX_SUCCESS => println!("Successfully called enclave program from untrusted app"),
        _ => println!("Error in calling enclave function from untrusted app: {}", result.as_str())
    }

    enclave.destroy();

}