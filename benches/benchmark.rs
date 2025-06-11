use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::rngs::OsRng;
use curve25519_dalek::Scalar;
use yao_gc_group5::{circuit::Circuit, utils::scalar_to_bits};

fn bench_different_sizes(c: &mut Criterion) {
    let mut rng = OsRng;
    let sizes = [10, 32, 64, 128, 256];

    for &size in sizes.iter() {
        let a = Scalar::random(&mut rng);
        let b = Scalar::random(&mut rng);
        let a_bits = scalar_to_bits(&a);
        let b_bits = scalar_to_bits(&b);
        let mut input_bits = Vec::with_capacity(size * 2);
        input_bits.extend_from_slice(&a_bits[..size]);
        input_bits.extend_from_slice(&b_bits[..size]);

        let circuit = Circuit::compare_n_bit_numbers(input_bits.clone(), size);

        c.bench_function(&format!("circuit construction {}bit", size), |b| {
            b.iter(|| Circuit::compare_n_bit_numbers(black_box(input_bits.clone()), size))
        });

        c.bench_function(&format!("circuit evaluation {}bit", size), |b| {
            b.iter(|| black_box(circuit.eval()))
        });
    }
}

criterion_group!(
    benches,
    bench_different_sizes
);
criterion_main!(benches);