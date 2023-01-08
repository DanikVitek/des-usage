#![feature(
    vec_push_within_capacity,
    array_zip,
    array_chunks,
    maybe_uninit_uninit_array,
    maybe_uninit_array_assume_init,
    core_intrinsics
)]

use cipher::{BlockSizeUser, Unsigned};
use des::Des;

pub mod crydec;
pub mod helper;
pub mod signature;

pub const CIPHER_BLOCK_SIZE: usize = <Des as BlockSizeUser>::BlockSize::USIZE;
