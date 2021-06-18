use criterion::{black_box, criterion_group, criterion_main, Criterion};
use buddhabrust::Buddha;

fn bench_small(c: &mut Criterion) {
    let (width, height, iteration) = (800, 800, 128);
    let buddha = Buddha::new(-1.3, -0.4, iteration, 800.0);
    let mut screen = vec![0; width * height];
    c.bench_function(&format!("mandelbrot {} * {} & {} iterations", width, height, iteration), |b| {
        b.iter(|| {
            buddha.compute(&mut screen, black_box(width), black_box(height));
        })
    });
}

fn bench_large(c: &mut Criterion) {
    let (width, height, iteration) = (4000, 4000, 128);
    let buddha = Buddha::new(-1.3, -0.4, iteration, 800.0);
    let mut screen = vec![0; width * height];
    c.bench_function(&format!("mandelbrot {} * {} & {} iterations", width, height, iteration), |b| {
        b.iter(|| {
            buddha.compute(&mut screen, black_box(width), black_box(height));
        })
    });
}

fn bench_normal(c: &mut Criterion) {
    let (width, height, iteration) = (2048, 1080, 128);
    let buddha = Buddha::new(-1.3, -0.4, iteration, 800.0);
    let mut screen = vec![0; width * height];
    c.bench_function(&format!("mandelbrot {} * {} & {} iterations", width, height, iteration), |b| {
        b.iter(|| {
            buddha.compute(&mut screen, black_box(width), black_box(height));
        })
    });
}

fn bench_normal_iter(c: &mut Criterion) {
    let (width, height, iteration) = (2048, 1080, 8000);
    let buddha = Buddha::new(-1.3, -0.4, iteration, 800.0);
    let mut screen = vec![0; width * height];
    c.bench_function(&format!("mandelbrot {} * {} & {} iterations", width, height, iteration), |b| {
        b.iter(|| {
            buddha.compute(&mut screen, black_box(width), black_box(height));
        })
    });
}


fn bench_small_iter(c: &mut Criterion) {
    let (width, height, iteration) = (128, 128, 50);
    let buddha = Buddha::new(-1.3, -0.4, iteration, 800.0);
    let mut screen = vec![0; width * height];
    c.bench_function(&format!("mandelbrot {} * {} & {} iterations", width, height, iteration), |b| {
        b.iter(|| {
            buddha.compute(&mut screen, black_box(width), black_box(height));
        })
    });
}

criterion_group!(benches, bench_small, bench_large, bench_normal, bench_normal_iter, bench_small_iter);
criterion_main!(benches);
