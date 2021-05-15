use criterion::{criterion_group, criterion_main, Criterion};
use day_3::day_3::{part_1, part_2};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 3 part 1", |b| {
        b.iter(|| {
            part_1(500000);
        })
    });
    c.bench_function("day 3 part 2", |b| {
        b.iter(|| {
            part_2(500000);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
