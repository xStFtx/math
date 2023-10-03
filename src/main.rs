mod complex;
mod fourier; // Import the fourier module

use complex::Complex;
use fourier::dft; // Import the dft function

fn main() {
    // Example usage
    let a = Complex::new(3.0, 4.0);
    let b = Complex::new(1.0, 2.0);

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("Magnitude of a: {}", a.magnitude());
    println!("Phase of b: {}", b.phase());

    let signal = vec![a, b];
    let dft_result = dft(&signal);

    for (k, dft_value) in dft_result.iter().enumerate() {
        println!("DFT[{}]: {:?}", k, dft_value);
    }
}
