#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub enum CvMat {}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
