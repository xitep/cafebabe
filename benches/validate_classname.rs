use std::{borrow::Cow, convert::TryFrom, hint::black_box};

use cafebabe::descriptors::ClassName;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_validate_classname(c: &mut Criterion) {
    let names = include_str!("classnames.txt")
        .lines()
        .map(|line| line.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    c.bench_function("classname-try-from", |b| {
        b.iter(|| {
            for name in &names {
                if black_box(ClassName::try_from(Cow::Borrowed(*name))).is_err() {
                    panic!("not a valid class name");
                }
            }
        });
    });
}

criterion_group!(benches, bench_validate_classname,);
criterion_main!(benches);
