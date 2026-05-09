use std::num::NonZeroUsize;

use criterion::{Criterion, criterion_group, criterion_main};

use wwa::{BreakpointOptions, Enemy, breakpoint_map};

fn bench_breakpoint_map(c: &mut Criterion) {
    let mut enemies = Vec::new();
    for i in 0..100 {
        enemies.push(Enemy {
            name: format!("enemy_{i}"),
            hp: NonZeroUsize::new(100 + i % 10).unwrap(),
            def: i % 20,
        });
    }
    let options = BreakpointOptions::default();

    c.bench_function("breakpoint_map_100_enemies", |b| {
        b.iter(|| breakpoint_map(&enemies, &options))
    });
}

criterion_group!(benches, bench_breakpoint_map);
criterion_main!(benches);
