
use swarmin::*;

use std::fs::File;

fn main() -> std::io::Result<()> {

    let lower_bound = -100.;
    let upper_bound = 100.;

    let mut swarm = ParticleSwarm::new(2, rosenbrock, 100, 1.49445, 1.49445, 0.729, -10.0, 10., lower_bound, upper_bound, 50, true);

    let result = swarm.solve();

    println!("{:?}", &result.solution);

    let mut buffer = File::create("pso_result.json")?;

    serde_json::to_writer(buffer, &result);

    Ok(())
}
