#![no_std]
#![allow(non_camel_case_types)]

//! The pointer-sized floating-point type.
//! 
//! The size of this primitive is how many bytes it takes to reference any location in memory. For example, on a 32 bit target, this is 4 bytes and on a 64 bit target, this is 8 bytes.
//! As f8 does not exist 

#[cfg(target_pointer_width = "16")]
pub type fsize = f16;

#[cfg(target_pointer_width = "32")]
pub type fsize = f32;

#[cfg(target_pointer_width = "64")]
pub type fsize = f64;

#[cfg(target_pointer_width = "128")]
pub type fsize = f128;

#[cfg(target_pointer_width = "8")]
compile_error!("8-bit systems are not supported.");
