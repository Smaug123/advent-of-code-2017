use criterion::{criterion_group, criterion_main, Criterion};
use day_25::day_25::{input, part_1};

fn criterion_benchmark(c: &mut Criterion) {
    let input = input();
    c.bench_function("day 25 part 1", |b| {
        b.iter(|| {
            part_1(&input);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
