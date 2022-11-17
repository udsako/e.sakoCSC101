use std::io;

fn main() {

    let p:i32 = 3200;
    let f:i32 = 3000;
    let a:i32 = 2500;
    let e:i32 = 2000;
    let w:i32 = 2500;

    println!("
        Menu                                        Price
        p = Poundo Yam/Edikainko Soup              -N3200
        f = Fried Rice and Chicken                 -N3000
        a = Amala and Ewedu Soup                   -N2500
        e = Eba and Egusi Soup                     -N2000
        w = White Rice and Stew                    -N2500");


    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();

    println!("How many portions of Poundo Yam and Edikaiko Soup would you like?");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let t:i32 = input1.trim().parse().expect("Not a valid response");

    println!("How many portions of Fried Rice and Chicken would you like?");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let x:i32 = input2.trim().parse().expect("Not a valid response");

    println!("How many portions of Amala and Ewedu Soup would you like?");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let u:i32 = input3.trim().parse().expect("Not a valid response");

    println!("How many portions of Eba and Egusi Soup would you like?");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let v:i32 = input4.trim().parse().expect("Not a valid response");

    println!("How many portions of White Rice and Stew would you like?");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let z:i32 = input5.trim().parse().expect("Not a valid response");

    let total:i32 = (p*t) + (f*x) + (a*u) + (e*v) + (w*z);

    if total > 10000
    {
        let discount = total * 95/100;
        println!("Total is {}", discount);
    }
    else {
        println!("Total is {}", total);
    }
}