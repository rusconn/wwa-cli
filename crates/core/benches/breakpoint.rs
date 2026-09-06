use std::num::NonZeroUsize;

use criterion::{Criterion, criterion_group, criterion_main};

use wwa::{BreakpointOptions, EnemiesBreakpointExt, Enemy};

fn bench_enemies_breakpoints(c: &mut Criterion) {
    let mut enemies = Vec::new();
    for i in 0..100 {
        enemies.push(Enemy {
            name: format!("enemy_{i}"),
            hp: NonZeroUsize::new(100 + i % 10).unwrap(),
            atk: i % 20,
            def: i % 20,
        });
    }
    let options = BreakpointOptions::default();

    c.bench_function("enemies_breakpoints_100_enemies", |b| {
        b.iter(|| enemies.breakpoints(&options))
    });
}

criterion_group!(benches, bench_enemies_breakpoints);
criterion_main!(benches);
