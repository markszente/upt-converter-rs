use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use upt_converter::convert_to_folders;
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample-size-example");

    group.sample_size(100);
    group.measurement_time(Duration::from_secs(1));
    group.bench_function("fib 20", |b| b.iter(|| convert_to_folders()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
