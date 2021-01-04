extern crate criterion;
extern crate markdown;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn bench_easy(c: &mut Criterion) {
    let text = format!("tests/fixtures/files/{}.text", "easy");
    let md = Path::new(&text);

    let mut tokens = String::new();
    File::open(md).unwrap().read_to_string(&mut tokens).unwrap();

    c.bench_function("bench_easy", |b| {
        b.iter(|| {
            for i in 1..=100 {
                black_box(markdown::to_html(&tokens));
            }
        });
    });
}

criterion_group!(benches, bench_easy);
criterion_main!(benches);
