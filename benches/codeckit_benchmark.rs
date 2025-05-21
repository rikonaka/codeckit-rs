use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use rand::Rng;
use rand::distributions::Alphanumeric;

// test
// use codeckit::Base64;

fn codeckit_func() {
    for length in 1..=100 {
        let rand_string: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();

        println!("Random string: {}", rand_string);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("codeckit", |b| b.iter(|| codeckit_func()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
