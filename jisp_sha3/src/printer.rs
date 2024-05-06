//! Some simple functions to transform states and bytes into strings

use crate::preprocessing::flip_ordering;

/// prints bytes as hexadecimal values using big endian encoding, adds a space every 4 bytes
/// 
/// # Examples
/// ```
/// use jisp_sha3::printer::print_bytes_be;
/// let m = vec![1, 2, 3, 4, 5, 6, 7, 8];
/// let s = print_bytes_be(&m);
/// 
/// assert_eq!(s, "01020304 05060708".to_owned());
/// ```
pub fn print_bytes_be(v:&Vec<u8>) -> String {
    let mut res = String::from("");
    let mut n = 0;
    for elem in v {
        if n == 4 {
            res += " ";
            n = 0;
        }
        res += &format!("{:02x?}", elem);
        n += 1;
    }
    return res;
} 


/// prints bytes as hexadecimal values using little endian encoding, adds a space every 4 bytes
/// 
/// # Examples
/// ```
/// use jisp_sha3::printer::print_bytes_le;
/// let m = vec![1, 2, 3, 4];
/// let s = print_bytes_le(&m);
/// 
/// assert_eq!(s, "8040c020".to_owned());
/// ```
pub fn print_bytes_le(v:&Vec<u8>) -> String {
    let v = flip_ordering(v);
    print_bytes_be(&v)
}