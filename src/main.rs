use yao_gc_group5::circuit::Circuit;
use curve25519_dalek::Scalar;
use num_bigint::BigUint;
use rand::rngs::OsRng;
use yao_gc_group5::utils;

fn main() {
    for _ in 0..100 {
        let a = Scalar::random(&mut OsRng);
        let b = Scalar::random(&mut OsRng);

        // Convert to bit representation
        let a_bits = utils::scalar_to_bits(&a);
        let b_bits = utils::scalar_to_bits(&b);

        // Combine A and B bits into one input vector
        let mut input_bits = Vec::with_capacity(512);
        input_bits.extend_from_slice(&a_bits);
        input_bits.extend_from_slice(&b_bits);

        // Build and evaluate the comparison circuit
        let circuit = Circuit::compare_n_bit_numbers(input_bits, 256);
        let circuit_result = circuit.eval();

        // Compare expected result using BigUint
        let a_int = BigUint::from_bytes_le(&a.to_bytes());
        let b_int = BigUint::from_bytes_le(&b.to_bytes());
        let expected = a_int > b_int;
        println!(
            "A = {:?}, B = {:?}, A > B = {}, and circuit says {}",
            a_int, b_int, expected, circuit_result
        );
        assert_eq!(
            circuit_result, expected,
            "Mismatch: A = {:?}, B = {:?}, A > B = {}, but circuit says {}",
            a_int, b_int, expected, circuit_result
        );
    }
}
