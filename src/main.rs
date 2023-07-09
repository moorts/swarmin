use ndarray::Array;

use swarmin::prelude::*;
//use swarmin::solver::*;

use std::fs::File;

fn main() -> std::io::Result<()> {
    let lower_bound = -5.;
    let upper_bound = 5.;

    let objective = Rosenbrock_2D {};

    let mut swarm = ParticleSwarm::new(
        2,
        objective,
        100,
        1.49445,
        1.49445,
        0.729,
        -10.0,
        10.0,
        lower_bound,
        upper_bound,
        50,
        false,
    );

    let result = swarm.solve();

    println!("{:?}", &result.solution);


    //let mut buffer = File::create("pso_result.json")?;

    //serde_json::to_writer(buffer, &result);

    Ok(())
}
