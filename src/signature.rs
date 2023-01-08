use std::borrow::Cow;

use cipher::{Block, BlockEncrypt, Key, KeyInit};
use des::Des;

use crate::{helper::iter::exact_blocks, CIPHER_BLOCK_SIZE};

pub fn sign(data: &[u8], key: &Key<Des>) -> [u8; CIPHER_BLOCK_SIZE] {
    let des = Des::new(key);

    let blocks = exact_blocks::<CIPHER_BLOCK_SIZE>(data);

    let mut convolution = convolution(blocks, &des);
    des.encrypt_block((&mut convolution).into());

    convolution
}

fn convolution(blocks: Vec<[Cow<u8>; CIPHER_BLOCK_SIZE]>, des: &Des) -> [u8; CIPHER_BLOCK_SIZE] {
    if blocks.is_empty() {
        return [1; CIPHER_BLOCK_SIZE];
    }
    blocks.into_iter().fold([1; CIPHER_BLOCK_SIZE], |v, s| {
        let mut encrypted = Block::<Des>::default();
        des.encrypt_block_b2b(
            (&s.zip(v).map(|(byte, last_v)| *byte ^ last_v)).into(),
            &mut encrypted,
        );
        encrypted.into()
    })
}
