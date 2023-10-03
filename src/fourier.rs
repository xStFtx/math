use crate::complex::Complex;

pub fn dft(signal: &[Complex]) -> Vec<Complex> {
    let n = signal.len();
    let mut result = vec![Complex::new(0.0, 0.0); n];

    for k in 0..n {
        let mut sum = Complex::new(0.0, 0.0);

        for t in 0..n {
            let angle = -2.0 * std::f64::consts::PI * (k as f64) * (t as f64) / (n as f64);
            let complex_exp = Complex::new(angle.cos(), angle.sin());
            sum = sum + (signal[t] * complex_exp);
        }

        result[k] = sum;
    }

    result
}
