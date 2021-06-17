use criterion::{black_box, criterion_group, criterion_main, Criterion};
use buddhabrust::Buddha;

fn bench_small(c: &mut Criterion) {
    let buddha = Buddha::new(-1.3, -0.4, 128, 800.0);
    let mut screen = vec![0; 800 * 800];
    c.bench_function("mandelbrot 800 * 800 & 128 iterations", |b| {
        b.iter(|| {
            buddha.compute(&mut screen, black_box(800), black_box(800));
        })
    });
}

fn bench_large(c: &mut Criterion) {
    let buddha = Buddha::new(-1.3, -0.4, 128, 800.0);
    let mut screen = vec![0; 4000 * 4000];
    c.bench_function("mandelbrot 4000 * 4000 & 128 iterations", |b| {
        b.iter(|| {
            buddha.compute(&mut screen, black_box(4000), black_box(4000));
        })
    });
}

fn bench_normal(c: &mut Criterion) {
    let buddha = Buddha::new(-1.3, -0.4, 128, 800.0);
    let mut screen = vec![0; 2048 * 1080];
    c.bench_function("mandelbrot 2048 * 1080 & 128 iterations", |b| {
        b.iter(|| {
            buddha.compute(&mut screen, black_box(128), black_box(4000));
        })
    });
}

fn bench_normal_iter(c: &mut Criterion) {
    let buddha = Buddha::new(-1.3, -0.4, 8000, 800.0);
    let mut screen = vec![0; 2048 * 1080];
    c.bench_function("mandelbrot 2048 * 1080 & 8000 iterations", |b| {
        b.iter(|| {
            buddha.compute(&mut screen, black_box(4000), black_box(4000));
        })
    });
}


fn bench_iter(c: &mut Criterion) {
    let buddha = Buddha::new(-1.3, -0.4, 8000, 800.0);
    let mut screen = vec![0; 50 * 50];
    c.bench_function("mandelbrot 50 * 50 & 8000 iterations", |b| {
        b.iter(|| {
            buddha.compute(&mut screen, black_box(50), black_box(50));
        })
    });
}

criterion_group!(benches, bench_small, bench_large, bench_iter);
criterion_main!(benches);
