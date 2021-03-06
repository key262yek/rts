// Benchmarks for reflective boundary condition

use rts::system_mod::{SystemCore, cont_circ::{ContCircSystem, check_bc_exact}};
use rts::position::Position;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_exact(c : &mut Criterion){
    // Collecting 100 samples in estimated 5.0034 s (3.7M iterations)
    // 1.3653 us 1.3946 us 1.4311 us
    let sys : ContCircSystem = ContCircSystem::new(1.0, 2);
    let mut pos : Position<f64> = Position::new(vec![0.99, 0.0]);
    let t: f64 = 3.141592f64 / 6f64;
    let mut dp : Position<f64> = Position::new(vec![0.1 * t.cos(), 0.1 * t.sin()]);

    c.bench_function("bc_exact", |b|  b.iter(|| check_bc_exact(black_box(sys),black_box(&mut pos), black_box(&mut dp))));
}


fn bench_ratio(c : &mut Criterion){
    // Collecting 100 samples in estimated 5.0021 s (7.6M iterations)
    // 656.58 ns 658.09 ns 659.75 ns
    // 2배 정도 느리다.
    // 전반적으로 check_bc 함수는 전체 rts 구동시간의 22% 정도를 차지한다.
    // 2배가 느려지는 것은 꽤 큰 performance 차이를 보일 것.
    let sys : ContCircSystem = ContCircSystem::new(1.0, 2);
    let mut pos : Position<f64> = Position::new(vec![0.99, 0.0]);
    let t: f64 = 3.141592f64 / 6f64;
    let mut dp : Position<f64> = Position::new(vec![0.1 * t.cos(), 0.1 * t.sin()]);

    c.bench_function("bc_ratio", |b| b.iter(|| sys.check_bc(black_box(&mut pos), black_box(&mut dp))));
}

criterion_group!(benches, bench_exact, bench_ratio);
criterion_main!(benches);
