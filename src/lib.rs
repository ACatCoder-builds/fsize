#![no_std]
#![allow(non_camel_case_types)]

//! The pointer-sized floating-point type.
//! 
//! The size of this primitive is how many bytes it takes to reference any location in memory. For example, on a 32 bit target, this is 4 bytes and on a 64 bit target, this is 8 bytes.
//! As f8 does not exist and #[cfg] cannot check for 128 bit systems, only 8, 16 and 32 bits are supported.

#[cfg(target_pointer_width = "16")]
pub type fsize = f16;

#[cfg(target_pointer_width = "32")]
pub type fsize = f32;

#[cfg(target_pointer_width = "64")]
pub type fsize = f64;

#[cfg(not(any(target_pointer_width = "16", target_pointer_width = "32", target_pointer_width="64")))]
compile_error!("Only 16, 32 and 64 bit systems support fsize.");
