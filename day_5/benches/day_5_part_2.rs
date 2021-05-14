use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_5::day_5::{input, part_2};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("part 2", |b| {
        let input = input();
        b.iter(|| {
            let mut input = input.to_vec();
            part_2(&mut input);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
