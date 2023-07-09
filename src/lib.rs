pub mod objective_functions;
pub mod prelude;
pub mod pso;
//pub mod solver;

use ndarray::{Array, Array1};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

use rand::prelude::*;
use std::cmp::*;

use serde::Serialize;

pub type Particle = Array1<f64>;
pub type Velocity = Array1<f64>;
