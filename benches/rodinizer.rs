use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{black_box, criterion_group, criterion_main};
use rand::Rng;
use rodinizer2::{read_names, rodinize};

fn rodinize_all<S: AsRef<str>>(strings: &[S], rng: &mut impl Rng) {
    for s in strings {
        black_box(rodinize(black_box(s.as_ref()), rng));
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let path = "./data/yob1999.txt";
    let names = read_names(path).expect("Failed to read file");
    c.bench_with_input(
        BenchmarkId::new("rodinizer", path),
        &names,
        |b, names| {
            let mut rng = rand::thread_rng();
            b.iter(|| rodinize_all(names, &mut rng));
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
