#[macro_use]
extern crate criterion;
extern crate logicGate;

use criterion::Criterion;
use logicGate::{and, or, xor};

fn and_benchmark(c: &mut Criterion) {
    c.bench_function("AND BenchMark", |b| b.iter(|| and(10, 10)));
}

fn or_benchmark(c: &mut Criterion) {
    c.bench_function("OR BenchMark", |b| b.iter(|| or(10, 10)));
}

fn xor_benchmark(c: &mut Criterion) {
    c.bench_function("X-OR BenchMark", |b| b.iter(|| or(10, 10)));
}

criterion_group!(logic_gate, and_benchmark, or_benchmark, xor_benchmark);
criterion_main!(logic_gate);
