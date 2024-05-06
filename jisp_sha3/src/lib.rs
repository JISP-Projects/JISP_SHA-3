pub mod preprocessing;
pub mod state;
pub mod sha3;

/// Various functions from the inner workings of the [SHA-3](crate::sha3) algorithm. 
/// You do not need to consider these just to use this crate's hashing functionality.
/// They are merely accessible for those interested.
pub mod internals {
    pub mod sponge;
    pub mod keccak;
}