use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter Experience (in years): ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let experience:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:f32 = input2.trim().parse().expect("Not a valid number");

    if experience >= 10.0 && age >= 40.0
    {
        println!("Your incentive is 1,560,000");
    }
    if experience >= 10.0 && age >=30.0 && age < 40.0
    {
        println!("Your incentive is 1,480,000");
    }
    if experience >= 10.0 && age < 28.0
    {
        println!("Your incentive is 1,300,000");
    }
    if experience < 10.0
    {
        println!("Your incentive is 100,000");
    }
}