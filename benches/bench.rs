use criterion::{criterion_group, criterion_main, Criterion};
use std::rc::Rc;
use itertools::Itertools;
use the_pun_machine::{core::prelude::*, };

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(100).measurement_time(instant::Duration::new(5, 0));

    targets= bench_solver,


);
criterion_main!(benches);

fn bench_solver(c: &mut Criterion) {
    //let context = Rc::new(WordContext::from_data(get_phrase_expressions()));

    let mut group = c.benchmark_group("solver");
    group.sample_size(10);
    // group.bench_function("Find a husband for Emma", |bench| {
    //     bench.iter(|| solve(context.clone()))
    // });
    group.finish()
}

