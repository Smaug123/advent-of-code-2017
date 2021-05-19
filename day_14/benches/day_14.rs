use criterion::{criterion_group, criterion_main, Criterion};
use day_14::day_14::{input, part_1, part_1_longhand, part_2};

fn criterion_benchmark(c: &mut Criterion) {
    let input = input();
    c.bench_function("day 14 part 1", |b| {
        b.iter(|| {
            part_1(&input);
        })
    });
    c.bench_function("day 14 part 1 longhand", |b| {
        b.iter(|| {
            part_1_longhand(&input);
        })
    });
    c.bench_function("day 14 part 2", |b| {
        b.iter(|| {
            part_2(&input);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
