use std::io;

fn main() {
    println!("\n Enter the value for a...");
    let mut input_1 = String::new();
    io::stdin().read_line(&mut input_1).expect("Not a valid number");
    let a:f32 = input_1.trim().parse().expect("Not a valid number");

    println!("\n Enter the value for b...");
    let mut input_2 = String::new();
    io::stdin().read_line(&mut input_2).expect("Not a valid number");
    let b:f32 = input_2.trim().parse().expect("Not a valid number");

    println!("\n Enter the value for c...");
    let mut input_3 = String::new();
    io::stdin().read_line(&mut input_3).expect("Not a valid number");
    let c:f32 = input_3.trim().parse().expect("Not a valid number");

    let d = b.powf(2.0) - 4.0*a*c;

    if d > 0.0 {
        println!("There are two distinct roots for the quadratic equation");
    } 
    else if d == 0.0 {
        println!("There is one real root for the quadratic equation");
    } else{
        println!("There are no real roots for the quadratic equation");
    }
}
