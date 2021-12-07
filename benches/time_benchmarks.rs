use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fast_hampath::fastpath::HampathBuilder;
use typed_arena::Arena;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve n=100", |b| b.iter(|| solve_random_hampath(black_box(100))));
}

fn solve_random_hampath(n: usize) {
    let a = Arena::new();
    HampathBuilder::new_random(n, &a).solution_pair();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);