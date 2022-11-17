// Rust program to read the height of a person 
// and then print if person is tail, dwarf,
// or average height person

use std::io;

fn main() 
{
    let mut input1 = String::new();

    println!("Enter Your Height (in centimetres):");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let height:f32 = input1.trim().parse().expect("Not a valid number");

    if height >= 150.0 && height <= 195.0
    {
        println!("You are of average height person");
    }
    else if height > 170.0 && height <= 195.0
    {
        println!("You are tall");
    }
    else 
    {
        println!("You are dwarf");
    }
}