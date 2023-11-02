use std::io;
fn main() {
    
    let mut time = String::new();
    let mut dist = String::new();

    println!("Please enter the time in hours");
    io::stdin().read_line(&mut time).expect("Please enter a number with a decimal point");
    let a:f32 = time.trim().parse().expect("Not a valid number");

    println!("please enter the distance in miles");
    io::stdin().read_line(&mut dist).expect("Please enter a number with a decimal point");
    let b:f32 = dist.trim().parse().expect("Not a valid number");
    let c:f32 = b*1.609;

    let speed:f32  = c / a;

    println!("Your speed will be in kilometers per hour");
    println!("Your speed is {}kmph", speed);
}
