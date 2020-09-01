#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


extern crate log;

#[macro_use]
extern crate lazy_static;

pub mod edlibrs;

//#[doc(hidden)]
pub mod bindings;


lazy_static! {
    #[allow(dead_code)]
    static ref LOG: u64 = {
        let res = init_log();
        res
    };
}
// install a logger facility
fn init_log() -> u64 {
    env_logger::try_init().unwrap();
    println!("\n ************** initializing logger *****************\n");    
    return 1;
}


#[cfg(test)]
mod tests {

    #[test]
    // initialize once log system for tests.
    fn init_log() {
        env_logger::try_init().unwrap();
    }
}  // end of tests
