#![allow(non_upper_case_globals)] //perhaps this three lines isn't necessary
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
include!(concat!("bindings.rs"));


use std::ffi::c_int;

//use crate::bindings::{root::add};

mod bindings;

fn main() {
    unsafe{

        let result: c_int= root::add(3, 5);
        println!("result: {}",result);

    }
}
