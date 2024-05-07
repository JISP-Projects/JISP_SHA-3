//! # About
//! This crate contains my pure-rust implementations of SHA-3 and its 6 variants, including the extendable output functions [SHAKE128](sha3::shake128) and [SHAKE256](sha3::shake256)
//!
//! # Security
//! This implementation is just my personal project and has not been officially verified or audited.
//! It should therefore not be used in any real-world applications, it is only meant for small personal projects such as mine. 
//! 
//! # Usage
//! To perform one of the hashing algorithm variations on your data you first need to parse it into a vector of `u8` bytes. 
//! You can then simply call one of the functions in [sha3] on your data.
//! Note that the standard for SHA-3 is to use little endian encoding. You can swap your data to a different encoding scheme using the functions in [preprocessing]
//! 
//! # Example
//! ```
//! use jisp_sha3::preprocessing::le_encoding;
//! use jisp_sha3::sha3::sha3_224;
//! use jisp_sha3::printer::print_bytes_le;
//! 
//! let hex = le_encoding("abc");
//! let hash = sha3_224(&hex);
//! let res = print_bytes_le(&hash);
//! 
//! let expected = "e642824c 3f8cf24a d09234ee 7d3c766f c9a3a516 8d0c94ad 73b46fdf".to_owned();
//! assert_eq!(res, expected);
//! ```


pub mod preprocessing;
pub mod sha3;
pub mod printer;

/// Various functions from the inner workings of the [SHA-3](crate::sha3) algorithm. 
/// You do not need to consider these just to use this crate's hashing functionality.
/// They are merely accessible for those interested.
pub mod internals {
    pub mod sponge;
    pub mod keccak;
    pub mod state;
}