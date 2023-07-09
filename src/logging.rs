#[derive(Debug, Serialize)]
pub struct SolverState {
    particles: Vec<Particle>,
    velocities: Vec<Velocity>,
    particle_bests: Vec<(Particle, f64)>,
    global_best_x: Particle,
    global_best_value: f64,
}

struct Logger {
    history: Vec<SolverState>
}

/*
trait Observer {
    fn notify_state(&mut self, 
}
*/
