use std::str::FromStr;

use criterion::{criterion_group, criterion_main, Criterion};
use the_pun_machine::core::prelude::{Category, DictionaryWord, PunFactory};

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(100).measurement_time(instant::Duration::new(5, 0));

    targets= bench_solver,


);
criterion_main!(benches);

fn bench_solver(c: &mut Criterion) {
    let theme_word = DictionaryWord::from_str("food").unwrap();

    let mut group = c.benchmark_group("solver");
    group.sample_size(10);
    group.bench_function("Count puns", |bench| {
        bench.iter(|| count_solutions(theme_word.clone(), None))
    });
    group.finish()
}

fn count_solutions(theme_word: DictionaryWord, category: Option<Category>) -> usize {
    theme_word.find_all_puns(&category).len()
}
