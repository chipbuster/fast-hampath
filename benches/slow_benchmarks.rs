use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fast_hampath::slowpath::HampathBuilder;
use fast_hampath::tngraph::TournamentGraph;
use typed_arena::Arena;

pub fn benchmark_100(c: &mut Criterion) {
    c.bench_function("solve n=100", |b| {
        b.iter(|| solve_random_hampath(black_box(100)))
    });
}

pub fn benchmark_200(c: &mut Criterion) {
    c.bench_function("solve n=200", |b| {
        b.iter(|| solve_random_hampath(black_box(200)))
    });
}

pub fn benchmark_50(c: &mut Criterion) {
    c.bench_function("solve n=50", |b| {
        b.iter(|| solve_random_hampath(black_box(50)))
    });
}

pub fn benchmark_10(c: &mut Criterion) {
    c.bench_function("solve n=10", |b| {
        b.iter(|| solve_random_hampath(black_box(50)))
    });
}

fn solve_random_hampath(n: usize) {
    let a = Arena::new();
    let g = TournamentGraph::new_random(n, &a);
    HampathBuilder::new(&g).solve();
}

fn sleep_short_time(c: &mut Criterion) {
    c.bench_function("sleep 500μs", |b| {
        b.iter(|| std::thread::sleep(black_box(std::time::Duration::from_micros(500))))
    });
}

fn sleep_long_time(c: &mut Criterion) {
    c.bench_function("sleep 25ms", |b| {
        b.iter(|| std::thread::sleep(black_box(std::time::Duration::from_millis(25))))
    });
}

criterion_group! {
    name = sleep_benches;
    config = Criterion::default();
    targets = sleep_short_time, sleep_long_time
}

criterion_group!(
    solve_benches,
    benchmark_10,
    benchmark_50,
    benchmark_100,
    benchmark_200
);

criterion_main!(solve_benches, sleep_benches);
