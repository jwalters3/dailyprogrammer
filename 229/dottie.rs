use std::f32;
use std::io;

fn f(x: f32) -> f32 {
    x.cos()
}

fn main() {
    let mut x = String::new();

    println!("Input seed");

    io::stdin().read_line(&mut x)
        .ok()
        .expect("invalid!");

    let mut x: f32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => { 1.1 },
    };

    while (f(x) - x).abs() > f32::EPSILON {
        x = f(x)
    }

    println!("Dottie: {}", x);
}
