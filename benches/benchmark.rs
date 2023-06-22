use criterion::{black_box, criterion_group, criterion_main, Criterion};
use swarmin::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve rosenbrock", |b| b.iter(|| {
        let lower_bound = Particle::new(vec![-5.,-5.]);
        let upper_bound = Particle::new(vec![5.,5.]);

        let mut swarm = ParticleSwarm::new(2, rosenbrock, 100, 2.0, 2.0, lower_bound, upper_bound, 200, false);
        swarm.solve();
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
