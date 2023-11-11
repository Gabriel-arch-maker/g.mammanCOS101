use std::io;
fn main() {

    let first:i32 = 1_560_000;
    let second:i32 = 1_480_000;
    let third:i32 = 1_300_000;
    let last:i32 = 100_000;

    println!("Please enter your age.");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("This is not a valid value");
    let a:i32 = age.trim().parse().expect("This is not a valid value");

    println!("Are you experienced or not?");
    let mut experienced = String::new();
    io::stdin().read_line(&mut experienced).expect("This is not a valid answer");
    let b:&str = experienced.trim().expect("This is not a valid answer");

    if b == "yes" {
        if a >= 40 {
            println!("Your salary is {}", first);
        } else if a >= 28 && a < 40 {
            println!("Your salary is {}", second);
        } else if a < 28 {
            println!("Your salary is {}", third)
        }
    } else{
        println!("Your salary is {}", last)
    }
}
