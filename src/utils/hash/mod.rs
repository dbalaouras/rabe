use blake2_rfc::blake2b::blake2b;
use rabe_bn::{G1, G2, Fr};

/// hash a String to an element of G1 using blake2b and generator g
pub fn blake2b_hash_g1(g: G1, data: &String) -> G1 {
    let hash = blake2b(64,&[] , data.as_bytes());
    return g * Fr::interpret(&pop(hash.as_ref()));
}

/// hash a String to an element of G2 using blake2b and generator g
pub fn blake2b_hash_g2(g: G2, data: &String) -> G2 {
    let hash = blake2b(64, &[], data.as_bytes());
    return g * Fr::interpret(&pop(hash.as_ref()));
}

/// hash a String to Fr using blake2b
pub fn blake2b_hash_fr(data: &String) -> Fr {
    let hash = blake2b(64, &[], data.as_bytes());
    return Fr::interpret(&pop(hash.as_ref()));
}

fn pop(barry: &[u8]) -> [u8; 64] {
    let mut array = [0u8; 64];
    for (&x, p) in barry.iter().zip(array.iter_mut()) {
        *p = x;
    }
    array
}