use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mandelbrust::Mandel;

fn bench_small(c: &mut Criterion) {
    let mandel = Mandel::new(-1.3, -0.4, 16, 800.0);
    let mut screen = vec![0; 800 * 800];
    c.bench_function("mandelbrot 800 * 800 & 128 iterations", |b| {
        b.iter(|| {
            mandel.compute(&mut screen, black_box(800), black_box(800));
        })
    });
}

fn bench_large(c: &mut Criterion) {
    let mandel = Mandel::new(-1.3, -0.4, 16, 800.0);
    let mut screen = vec![0; 4000 * 4000];
    c.bench_function("mandelbrot 4000 * 4000 & 128 iterations", |b| {
        b.iter(|| {
            mandel.compute(&mut screen, black_box(4000), black_box(4000));
        })
    });
}

fn bench_iter(c: &mut Criterion) {
    let mandel = Mandel::new(-1.3, -0.4, 1024, 800.0);
    let mut screen = vec![0; 50 * 50];
    c.bench_function("mandelbrot 50 * 50 & 8096 iterations", |b| {
        b.iter(|| {
            mandel.compute(&mut screen, black_box(50), black_box(50));
        })
    });
}

criterion_group!(benches, bench_small, bench_large, bench_iter);
criterion_main!(benches);
