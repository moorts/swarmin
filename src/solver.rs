use ndarray::Array1;

pub trait ObjectiveFunction {
    const DIM: usize;
    
    fn cost(&self, p: &Array1<f64>) -> f64;
}

/*
 * N-dimensional Rosenbrock 
 */
pub struct Rosenbrock<const N: usize> {}

impl<const N: usize> ObjectiveFunction for Rosenbrock<N> {
    const DIM: usize = N;

    fn cost(&self, p: &Array1<f64>) -> f64 {
        assert_eq!(Self::DIM, p.dim());
        (0..Self::DIM-1).map(|i| 100.*(p[i + 1] - p[i]*p[i]).powi(2) + (1. - p[i]).powi(2)).sum()
    }
}

pub type Rosenbrock_2D = Rosenbrock::<2>;
pub type Rosenbrock_3D = Rosenbrock::<3>;
pub type Rosenbrock_4D = Rosenbrock::<4>;
pub type Rosenbrock_5D = Rosenbrock::<5>;

/*
struct Solver<O> {
    swarm: ParticleSwarm,

    max_iterations: usize,

    optimum_value: Option<f64>,
    optimum_tolerance: Option<f64>
}

trait<O> Solver
where
    O: ObjectiveFunction
{
    //fn new(objective: O, 
}

*/
