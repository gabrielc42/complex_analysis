use num::Complex;
use num::traits::pow;

fn main() {
    // let theta = 
    let complex_integer = num::complex::Complex::new(10, 20);
    let complex_float = num::complex::Complex::new(10.1, 20.1);
    
    let c_a = num::complex::Complex::new(1, 2);

    let c_b = num::complex::Complex::new(1, -1);
    let c_c = num::complex::Complex::new(3, 2);
    let c_d = num::complex::Complex::new(2, 1);
    
    let c_a_exponent = c_a.scale(3);
    let c_a_exponent2 = c_a.powi(3); // exponent2, .powi returns correct solution, .scale in Trait needs work
    println!("CA num = {}, .scale CA Trait exponent = {}, .powi i32 exponent = {}\n", c_a, c_a_exponent, c_a_exponent2);

    let c_b_exponent = c_b.powi(3);
    let c_c_exponent = c_c.powi(3);
    let c_d_exponent = c_d.powi(2);

    let c_calculate = (c_a_exponent - c_b_exponent) / (c_c_exponent - c_d_exponent);
    
    println!("Complex integer: {}", complex_integer);
    println!("Complex float: {}", complex_float);
    println!("Complex point 1: {}", c_calculate);
}

// https://autumnai.github.io/cuticula/num/complex/struct.Complex.html

// https://www.wolframalpha.com/input?i=%28%281%2B2i%29%5E3+-+%281-i%29%5E3%29+%2F+%28%283%2B2i%29%5E3+-+%282%2Bi%29%5E2%29
// notice the result and the polar coordinates, consider a simple coding equation practice with those patterns