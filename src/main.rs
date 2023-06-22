
use swarmin::*;
use swarmin::particle::*;

fn main() {

    let lower_bound = Particle::new(vec![-5.,-5.]);
    let upper_bound = Particle::new(vec![5.,5.]);

    let mut swarm = ParticleSwarm::new(2, rosenbrock, 100, 2.0, 2.0, lower_bound, upper_bound, 10, false);

    println!("{:?}", swarm.solve());
}
