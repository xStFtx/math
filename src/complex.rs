// complex.rs

use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Complex {
        Complex { real, imag }
    }

    pub fn magnitude(&self) -> f64 {
        (self.real.powi(2) + self.imag.powi(2)).sqrt()
    }

    pub fn phase(&self) -> f64 {
        self.imag.atan2(self.real)
    }
}

// Implement the multiplication trait (Mul) for Complex
impl Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        let real = self.real * other.real - self.imag * other.imag;
        let imag = self.real * other.imag + self.imag * other.real;
        Complex::new(real, imag)
    }
}

// Implement the addition trait (Add) for Complex (for completeness)
impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex::new(self.real + other.real, self.imag + other.imag)
    }
}
