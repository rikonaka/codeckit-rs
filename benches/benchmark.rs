use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use rand::Rng;
use rand::distributions::Alphanumeric;

// test
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use codeckit::Base64;

fn rand_string(length: usize) -> String {
    let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    rand_string
}

fn codeckit_func() {
    for length in 1..1000 {
        let rand_string = rand_string(length);
        let encoded = Base64::encode(&rand_string.as_bytes());
        let decoded = Base64::decode(&encoded);
        assert_eq!(rand_string, decoded);
    }
}

fn base64_func() {
    for length in 1..1000 {
        let rand_string = rand_string(length);
        let encoded = STANDARD.encode(&rand_string);
        let decoded = STANDARD.decode(&encoded).unwrap();
        assert_eq!(rand_string, String::from_utf8_lossy(&decoded));
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("codeckit", |b| b.iter(|| codeckit_func()));
    c.bench_function("base64", |b| b.iter(|| base64_func()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
