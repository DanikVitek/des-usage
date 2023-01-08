use cipher::{Block, BlockDecrypt, BlockEncrypt, Key, KeyInit};
use des::Des;

use crate::{
    helper::iter::{exact_blocks, trim_suffix},
    CIPHER_BLOCK_SIZE,
};

pub fn encrypt(data: &[u8], key: &Key<Des>) -> Vec<u8> {
    let des = Des::new(key);

    exact_blocks::<CIPHER_BLOCK_SIZE>(data)
        .into_iter()
        .map(|in_block| {
            let mut out_block = Block::<Des>::default();
            des.encrypt_block_b2b((&in_block.map(|el| *el)).into(), &mut out_block);
            out_block
        })
        .flat_map(|block| block.into_iter())
        .collect()
}

pub fn decrypt(encrypted: &[u8], key: &Key<Des>) -> Vec<u8> {
    let des = Des::new(key);

    let decrypted = exact_blocks::<CIPHER_BLOCK_SIZE>(encrypted)
        .into_iter()
        .map(|in_block| {
            let mut out_block = Block::<Des>::default();
            des.decrypt_block_b2b((&in_block.map(|el| *el)).into(), &mut out_block);
            out_block
        })
        .flat_map(|block| block.into_iter())
        .collect();

    trim_suffix(decrypted, &0)
}
