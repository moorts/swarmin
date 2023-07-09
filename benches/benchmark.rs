use criterion::{black_box, criterion_group, criterion_main, Criterion};
use swarmin::*;

pub fn pso_rosenbrock_benchmark(c: &mut Criterion) {
    c.bench_function("solve rosenbrock", |b| {
        b.iter(|| {
            let lower_bound = Particle::new(vec![-5., -5.]);
            let upper_bound = Particle::new(vec![5., 5.]);

            let objective = Rosenbrock_2D {};

            let mut swarm = ParticleSwarm::new(
                2,
                objective,
                100,
                2.0,
                2.0,
                lower_bound,
                upper_bound,
                200,
                false,
            );
            swarm.solve();
        })
    });
}

criterion_group!(benches, pso_rosenbrock_benchmark);
criterion_main!(benches);
