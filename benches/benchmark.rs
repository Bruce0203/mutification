use std::hint::black_box;

use criterion::Criterion;
use mutification::ToMut;

fn benchmark(c: &mut Criterion) {
    c.bench_function("test", |b| {
        b.iter(|| {
            #[derive(ToMut)]
            struct Player(&'static str);

            let a = Player("HI");
            a.to_mut().0 = "hi";
            black_box(a);
        });
    });
}

criterion::criterion_group!(benches, benchmark);
criterion::criterion_main!(benches);
