
use swarmin::*;
use swarmin::particle::*;

use std::fs::File;

fn main() -> std::io::Result<()> {

    let lower_bound = Particle::new(vec![-20.,-20.]);
    let upper_bound = Particle::new(vec![20.,20.]);

    let mut swarm = ParticleSwarm::new(2, rosenbrock, 100, 2.0, 2.0, lower_bound, upper_bound, 50, true);

    let result = swarm.solve();

    let mut buffer = File::create("pso_result.json")?;

    serde_json::to_writer(buffer, &result);

    Ok(())
}
