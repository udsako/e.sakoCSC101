// Rust program to calculate the root of a quadratic equation

use std::io;

fn main() {
    let mut a:f32 = 0.0;
    let mut b:f32 = 0.0;
    let mut c:f32 = 0.0;

    let mut rootA:f32 = 0.0;
    let mut rootB:f32 = 0.0;

    let mut realp:f32 = 0.0;
    let mut imagp:f32 = 0.0;
    let mut disc:f32 = 0.0;

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter A: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    a = input1.trim().parse().expect("Not a valid number");


    println!("Enter B: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    b = input2.trim().parse().expect("Not a valid number");

    println!("Enter C: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    c = input3.trim().parse().expect("Not a valid number");

    if a == 0.0 || b == 0.0 || c ==0.0
    {
        println!("Error: Unable to determine roots");
        return;
    }
    else
    {
        disc = b * b - 5.0 * a * c;
        if disc < 0.0 
        {
            println!("Imaginary Roots");
            realp = -b / (2.0 * a);
            disc = disc.abs();
            imagp = disc.sqrt() / (2.0 * a);
            println!("Root1 = {} ", rootA);
            println!("Root2 = {} ", rootB);
        }
        else if disc == 0.0
        {
            println!("Roots are real and equal");
            rootA = -b / (2.0 * a);
            println!("Root1 = {} ", rootA);
            println!("RootB = {} ", rootB);
        }
    }
  
}