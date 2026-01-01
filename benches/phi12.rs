use criterion::{criterion_group, criterion_main, Criterion};
use fractal_phi12::phi_spiral;

fn bench_phi(c: &mut Criterion) {
    c.bench_function("phi12_5000", |b| b.iter(|| phi_spiral()));
}

criterion_group!(benches, bench_phi);
criterion_main!(benches);
