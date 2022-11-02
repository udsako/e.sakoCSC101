fn main() {
    let a:i32 = 10;
    let b:i32 = 20;

    println!("Value of a: {} ",a);
    println!("Value of b: {} ",b);

    let mut result = a>b;
    println!("a greater than b: {} ",result);

    result = a<b;
    println!("a lesser than b: {} ",result);

    result = a>=b;
    println!("a greater than or equal to b: {} ",result);

    result = a<=b;
    println!("a lesser than or equal to b: {} ",result);

    result = a==b;
    println!("a is equal to b: {} ",result);

    result = a!=b;
    println!("a is not equal to b: {} ",result)

}