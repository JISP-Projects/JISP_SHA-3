//! A collection of functions used for message encoding and padding

/// flips each individual byte in a vector from little endian ordering to big endian ordering or vice versa
pub fn flip_ordering(v: &Vec<u8>) -> Vec<u8> {
    v.iter().map(|u| u.reverse_bits()).collect()
}

/// encodes a string into bytes using little endian byte encoding
pub fn le_encoding(s: &str) -> Vec<u8> {
    let bytes = be_encoding(s);
    flip_ordering(&bytes)
}

/// encodes a string into bytes using big endian byte encoding
pub fn be_encoding(s: &str) ->Vec<u8>{
    s.as_bytes().into()
}

/// Splits `u64` words into `u8` bytes. Used internally to transform words in a state back into bytes
/// # Examples
/// ```
/// use jisp_sha3::preprocessing::split_bytes;
/// let words = vec![0x0102_0304_0506_0708];
/// let bytes = split_bytes(&words);
/// 
/// assert_eq!(bytes, vec![1, 2, 3, 4, 5, 6, 7, 8]);
/// ```
pub fn split_bytes(v:&Vec<u64>) -> Vec<u8> {
    let mut res = Vec::new();
    for word in v {
        let mut word = *word;
        let mut byte_list = [0u8;8];
        for i in 0..8 {
            byte_list[7 - i] = (word & 0xff) as u8;
            word >>= 8;
        }
        for i in byte_list {
            res.push(i);
        }
    }
    return res;
}

/// Pads a string of bytes and splits it in the specified block-size. Used internally in the [SHA-3](crate::sha3) functions
/// 
/// # Panics
/// If the suffix is longer than 6 bits. Note that it is a maximum of 4 bits in the 
/// 
/// # Example
/// ```
/// use jisp_sha3::preprocessing::padding;
/// 
/// let m = vec![0;7];
/// let suffix = vec![false, true];
/// 
/// let res = padding::<1>(&m, &suffix)[0][0];
/// let expected:u64 = 0b0110_0001;
/// 
/// assert_eq!(res, expected);
/// ```
pub fn padding<const BLOCK:usize>(bytes:&Vec<u8>, suffix:&Vec<bool>) -> Vec<[u64; BLOCK]> {
    vec![0;7];
    let words = merge_bytes(bytes, suffix);
    let blocks = merge_words::<BLOCK>(&words);
    blocks
}

fn merge_words<const BLOCK:usize>(words: &Vec<u64> ) -> Vec<[u64;BLOCK]> {
    let mut res = Vec::new();

    let mut block = [0u64;BLOCK];
    let mut block_pos = 0;

    for word in words {
        if block_pos >= BLOCK {
            res.push(block);
            block = [0u64;BLOCK];
            block_pos = 0;
        }
        block[block_pos] = *word;
        block_pos += 1;
    }

    //note that due to the restriction on suffix to 6 bits the final bit of the final word will be a 0 (if it even is the final entry in this block)
    block[BLOCK - 1] += 1;
    res.push(block);
    return res;
}

fn merge_bytes(bytes: &Vec<u8>, suffix:&Vec<bool>) -> Vec<u64> {
    
    let mut res = Vec::new();

    let mut word = 0u64;
    let mut word_pos = 0;

    for byte in bytes {
        //add byte
        word = (word << 8) + *byte as u64;
        word_pos += 1;

        if word_pos >= 8 {
            //add word
            res.push(word);

            word = 0;
            word_pos = 0;
        }
    }

    //append final unfinished word.
    word = (word << 8) + suffix_to_u8(suffix) as u64;
    word_pos += 1;

    while word_pos < 8 {
        word = word << 8;
        word_pos += 1;
    }

    res.push(word);

    return res;
}

fn suffix_to_u8(suffix:&Vec<bool>) -> u8 {
    if suffix.len() > 6 {panic!("Suffix is longer than 6 bits!")}
    let mut res = 0;
    let mut count = 6; 

    for b in suffix {
        res = (res << 1) + *b as u8;
        count -= 1;
    }

    res = (res << 1) + 1;

    for _ in 0..count {
        res = res << 1;
    }

    return res << 1;
}

