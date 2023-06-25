use std::ops::*;
use std::assert;
use std::cmp::*;

use serde::Serialize;

#[derive(Debug, Clone,Serialize)]
pub struct Particle {
    position: Vec<f64>,
    dim: usize
}


impl Particle {
    pub fn new(position: Vec<f64>) -> Particle {
        let dim = position.len();
        Particle {
            position: position,
            dim: dim
        }
    }

    pub fn zeros(dim: usize) -> Particle {
        Particle {
            position: vec![0.; dim],
            dim: dim
        }
    }

    fn combine_pointwise<F>(&self, other: &Self, mut op: F) -> Self
        where F: FnMut((f64, f64)) -> f64 {
        let mut position = Vec::with_capacity(self.dim);
        for i in 0..self.dim {
            position.push(op((self.position[i], other.position[i])));
        }
        Particle {
            position: position,
            dim: self.dim
        }
    }

    fn combine_pointwise_assign<F>(&mut self, other: &Self, mut op: F)
        where F: FnMut((f64, f64)) -> f64 {
        for i in 0..self.dim {
            self.position[i] = op((self.position[i], other.position[i]));
        }
    }

    pub fn restrict(&self, min: &Self, max: &Self) -> Self {
        let mut position = Vec::with_capacity(self.dim);
        for i in 0..self.dim {
            position.push(self[i].clamp(min[i], max[i]));
        }
        Self::new(position)
    }

}

impl Index<usize> for Particle {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.position[index]
    }
}

/*
impl IntoIterator for Particle {
    type Item = f64;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.position.into_iter()
    }
}

impl IntoIterator for &Particle {
    type Item = f64;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.position.into_iter()
    }
}
*/

 
impl Mul<f64> for Particle {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            position: self.position.iter().map(|x| x * other).collect(),
            dim: self.dim
        }
    }
}

impl Mul<f64> for &Particle {
    type Output = Particle;

    fn mul(self, other: f64) -> Particle {
        Particle {
            position: self.position.iter().map(|x| x * other).collect(),
            dim: self.dim
        }
    }
}

impl Sub for Particle {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.combine_pointwise(&other, |(a, b)| a - b)
    }
}

impl Add for Particle {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.combine_pointwise(&other, |(a, b)| a + b)
    }
}

impl Add<&Particle> for Particle {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        self.combine_pointwise(other, |(a, b)| a + b)
    }
}

impl Add<Particle> for &Particle {
    type Output = Particle;

    fn add(self, other: Particle) -> Particle {
        self.combine_pointwise(&other, |(a, b)| a + b)
    }
}

impl Add for &Particle {
    type Output = Particle;

    fn add(self, other: &Particle) -> Particle {
        self.combine_pointwise(other, |(a, b)| a + b)
    }
}

impl AddAssign for Particle {
    fn add_assign(&mut self, rhs: Particle) {
        for i in 0..self.dim {
            self.position[i] += rhs[i];
        }
    }
}

impl Sub for &Particle {
    type Output = Particle;

    fn sub(self, other: &Particle) -> Particle {
        self.combine_pointwise(other, |(a, b)| a - b)
    }
}

impl Sub<&Particle> for Particle {
    type Output = Self;

    fn sub(self, other: &Self) -> Self {
        self.combine_pointwise(other, |(a, b)| a - b)
    }
}

impl SubAssign for Particle {
    fn sub_assign(&mut self, rhs: Particle) {
        for i in 0..self.dim {
            self.position[i] -= rhs[i];
        }
    }
}

impl Ord for Particle {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut result = Ordering::Equal;
        for i in 0..self.dim {
            result = result.then(self[i].total_cmp(&other[i]));
        }
        result
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        let mut position = Vec::with_capacity(self.dim);
        for i in 0..self.dim {
            position.push(self[i].clamp(min[i], max[i]));
        }
        Self::new(position)
    }
}


impl PartialOrd for Particle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Particle {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Particle {}
