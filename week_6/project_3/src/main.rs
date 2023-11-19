use std::io;

fn main() {
    let mut input1 = String::new();
    println!("Input the upper limit: ");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let n:u32 = input1.trim().parse().expect("Invalid input");

    println!("**** Multiplication Table of {} ****",n);
    for i in 1..12 {
        for j in 1..n {
            println!("{} * {} = {}", i, j, i*j);
            println!("");
        }
    }

    if n == 1 {
        println!("Please input a number greater than 1");
    }
}