use std::{borrow::Cow, intrinsics::unlikely, mem::MaybeUninit};

pub fn exact_blocks<'data, const N: usize>(values: &'data [u8]) -> Vec<[Cow<'data, u8>; N]> {
    let mut chunks_exact = values.array_chunks::<N>();

    let mut data_blocks: Vec<[Cow<'data, u8>; N]> = Vec::with_capacity(
        chunks_exact.len()
            + (!chunks_exact.remainder().is_empty())
                .then_some(1)
                .unwrap_or_default(),
    );

    for chunk in chunks_exact.by_ref() {
        let mut chunk_of_refs: [MaybeUninit<Cow<'data, u8>>; N] = MaybeUninit::uninit_array();
        for (i, el) in chunk.iter().enumerate() {
            chunk_of_refs[i].write(Cow::Borrowed(el));
        }

        // SAFETY: array has been initialized
        let chunk_of_refs = unsafe { MaybeUninit::array_assume_init(chunk_of_refs) };

        // SAFETY: vec has enough capacity left
        unsafe {
            data_blocks
                .push_within_capacity(chunk_of_refs)
                .unwrap_unchecked()
        };
    }

    if unlikely(chunks_exact.remainder().is_empty()) {
        return data_blocks;
    }

    let mut remainder: [Cow<'data, u8>; N] = [0; N].map(Cow::Owned);
    for (i, el) in chunks_exact.remainder().iter().enumerate() {
        remainder[i] = Cow::Borrowed(el);
    }

    // SAFETY: vec has enough capacity left
    unsafe {
        data_blocks
            .push_within_capacity(remainder)
            .unwrap_unchecked()
    };

    data_blocks
}

pub fn trim_suffix(mut values: Vec<u8>, suffix: &u8) -> Vec<u8> {
    let mut len = values.len();
    while len > 0 && &values[len - 1] == suffix {
        len -= 1;
    }
    values.truncate(len);
    values.shrink_to_fit();
    values
}

#[cfg(test)]
mod tests {
    use super::trim_suffix;

    #[test]
    fn trim_zeros_on_nonempty_arr_works() {
        let data = vec![1, 2, 3, 4, 5, 0, 0, 0, 0];
        assert_eq!(vec![1, 2, 3, 4, 5], trim_suffix(data, &0))
    }

    #[test]
    fn trim_zeros_on_empty_arr_returns_empty_slice() {
        let data = vec![];
        let empty_arr: Vec<u8> = vec![];
        assert_eq!(empty_arr, trim_suffix(data, &0))
    }
}
