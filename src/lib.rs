use ndarray::{Array, Array1};
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;

use rand::prelude::*;
use std::cmp::*;

use serde::Serialize;

type Particle = Array1<f64>;
type Velocity = Array1<f64>;

#[derive(Debug,Serialize)]
pub struct SolverResult {
    pub solution: (Particle, f64),
    swarm_size: usize,
    cognitive_factor: f64,
    social_factor: f64,
    lower_bound: f64,
    upper_bound: f64,
    max_iterations: usize,
    history: Vec<SolverState>
}

#[derive(Debug,Serialize)]
pub struct SolverState {
    particles: Vec<Particle>,
    velocities: Vec<Velocity>,
    particle_bests: Vec<(Particle, f64)>,
    global_best_x: Particle,
    global_best_value: f64,
}

pub struct ParticleSwarm {
    dim: usize,
    objective: fn(&Particle) -> f64,


    swarm_size: usize,
    particles: Vec<Particle>,
    velocities: Vec<Velocity>,
    particle_bests: Vec<(Particle, f64)>,
    global_best_x: Particle,
    global_best_value: f64,

    cognitive_factor: f64,
    social_factor: f64,
    inertia: f64,

    v_min: f64,
    v_max: f64,

    lower_bound: f64,
    upper_bound: f64,

    max_iterations: usize,
    keep_history: bool
}


impl ParticleSwarm {

    pub fn new(dim: usize, objective: fn(&Particle) -> f64, swarm_size: usize, cognitive_factor: f64, social_factor: f64, inertia: f64, v_min: f64, v_max: f64, lower_bound: f64, upper_bound: f64, max_iterations: usize, keep_history: bool) -> Self {
        // Initialize Particles and Velocitie
        let mut particles = Vec::with_capacity(swarm_size);
        let mut velocities = vec![Array::zeros(dim); swarm_size];
        let mut particle_bests = Vec::with_capacity(swarm_size);
        let mut global_best_x = Array::zeros(dim);
        let mut global_best_value = f64::INFINITY;

        let mut rng = rand::thread_rng();
        
        for i in 0..swarm_size {
            // Generate random particle position within bounds
            //let position = (0..dim).map(|i| rng.gen_range(lower_bound[i]..upper_bound[i])).collect();
            let particle = Array::random(dim, Uniform::new(lower_bound, upper_bound));

            let objective_value = objective(&particle);

            particles.push(particle.clone());
            particle_bests.push((particle.clone(), objective_value));


            if objective_value < global_best_value {
                global_best_x = particle;
                global_best_value = objective_value;
            }
        }

        Self {
            dim: dim,
            objective: objective,
            swarm_size: swarm_size,
            particles: particles,
            velocities: velocities,
            particle_bests: particle_bests,
            global_best_x: global_best_x,
            global_best_value: global_best_value,
            cognitive_factor: cognitive_factor,
            social_factor: social_factor,
            inertia: inertia,
            v_min: v_min,
            v_max: v_max,
            lower_bound: lower_bound,
            upper_bound: upper_bound,
            max_iterations: max_iterations,
            keep_history: keep_history
        }
        
    }

    fn restrict_particle(p: Particle, p_min: f64, p_max: f64) -> Particle {
        p.map(|x| x.clamp(p_min, p_max))
    }

    fn step(&mut self) {
        //let mut new_global_best_x = Particle::zeros(self.dim);
        //let mut new_global_best_value = f64::INFINITY;

        for i in 0..self.swarm_size {
            let particle = &self.particles[i];
            let velocity = &self.velocities[i];
            let (p_best_x, p_best_value) = &self.particle_bests[i];


            // Update velocity and position
            let cognitive_update = (p_best_x - particle) * self.cognitive_factor * rand::random::<f64>();
            let social_update = (&self.global_best_x - particle) * self.social_factor * rand::random::<f64>();
            let updated_velocity = velocity * self.inertia + cognitive_update + social_update;
            let updated_velocity = Self::restrict_particle(updated_velocity, self.v_min, self.v_max);

            let updated_particle = particle + &updated_velocity;
            let updated_particle = Self::restrict_particle(updated_particle, self.lower_bound, self.upper_bound);

            self.particles[i] = updated_particle.clone();
            self.velocities[i] = updated_velocity;

            // Evaluate objective at new particle and update best values
            let objective_value = (self.objective)(&updated_particle);

            if objective_value < *p_best_value {
                self.particle_bests[i] = (updated_particle.clone(), objective_value);
            }
            if objective_value < self.global_best_value {
                self.global_best_x = updated_particle;
                self.global_best_value = objective_value;
            }
        }
    }

    pub fn solve(&mut self) -> SolverResult {
        let mut history = Vec::new();
        for i in 0..self.max_iterations {
            self.step();
            if self.keep_history {
                let state = SolverState {
                    particles: self.particles.clone(),
                    velocities: self.velocities.clone(),
                    particle_bests: self.particle_bests.clone(),
                    global_best_x: self.global_best_x.clone(),
                    global_best_value: self.global_best_value
                };
                history.push(state);
            }
        }

        SolverResult {
            solution: (self.global_best_x.clone(), self.global_best_value),
            swarm_size: self.swarm_size,
            cognitive_factor: self.cognitive_factor,
            social_factor: self.social_factor,
            lower_bound: self.lower_bound.clone(),
            upper_bound: self.upper_bound.clone(),
            max_iterations: self.max_iterations,
            history: history
        }
    }
}

pub fn rosenbrock(p: &Particle) -> f64 {
    // n-dimensional rosenbrock
    // a = 1, b = 100
    (0..p.dim()-1).map(|i| 100.*(p[i + 1] - p[i]*p[i]).powi(2) + (1. - p[i]).powi(2)).sum()
}
