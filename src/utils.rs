use curve25519_dalek::Scalar;

pub fn scalar_to_bits(scalar: &Scalar) -> Vec<bool> {
    let bytes = scalar.to_bytes(); // Returns [u8; 32]
    let mut bits = Vec::with_capacity(256);

    for byte in bytes.iter().rev() {
        for i in (0..8).rev() {
            bits.push((byte >> i) & 1 == 1);
        }
    }
    return bits;
}
