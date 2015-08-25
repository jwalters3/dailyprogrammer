use std::f32;
use std::io;

fn fixed_point<F>(f: F, mut x: f32) -> f32 
    where F : Fn(f32) -> f32 {

    let mut count = 0;
    loop {
        let old = x;
        x = f(x);
        count += 1;

        if (x - old).abs() <= f32::EPSILON || count > 10000 { break; }
    }
    x
}

fn main() {
    let mut x = String::new();

    println!("Input seed");

    io::stdin().read_line(&mut x)
        .ok()
        .expect("invalid!");

    let x: f32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => { 1.1 },
    };

    // Dottie Number
    let dottie = fixed_point(|y| y.cos(), x);
   
    println!("Dottie: {}", dottie);


    // Challenge 1
    let ch1 = fixed_point(|y| y - y.tan(), x);

    println!("challenge 1: {}", ch1);

    // Challenge 2
    let ch2 = fixed_point(|y| 1.0 + (1.0 / y), x);

    println!("challenge 2: {}", ch2);

    // Challenge 3
    let ch3 = fixed_point(|y| 4.0 * y * (1.0 - y), x);

    println!("challenge 3: {}", ch3);
}
