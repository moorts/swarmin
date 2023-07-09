use crate::Particle;

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
 * N-dimensional Rastrigin 
 */
pub struct Rastrigin<const N: usize> {}

impl<const N: usize> ObjectiveFunction for Rastrigin<N> {
    const DIM: usize = N;

    fn cost(&self, p: &Array1<f64>) -> f64 {
        assert_eq!(Self::DIM, p.dim());
        10. * (Self::DIM as f64)
            + p.map(|x| (x.powi(2) - 10. * (std::f64::consts::TAU * x).cos()))
                .sum()
    }
}

pub type Rastrigin_2D = Rastrigin::<2>;
pub type Rastrigin_3D = Rastrigin::<3>;
pub type Rastrigin_4D = Rastrigin::<4>;
pub type Rastrigin_5D = Rastrigin::<5>;

/*
 * Himmelblau function, only defined over R^2
 */
struct Himmelblau {}

impl ObjectiveFunction for Himmelblau {
    const DIM: usize = 2;

    fn cost(&self, p: &Array1<f64>) -> f64 {
        assert!(p.dim() == 2);
        (p[0].powi(2) + p[1] - 11.).powi(2) + (p[0] + p[1].powi(2) - 7.).powi(2)
    }
}
