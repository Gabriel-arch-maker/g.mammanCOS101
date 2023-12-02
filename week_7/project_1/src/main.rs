use std::io;
fn main() {
    println!("What do you want to calculate");
    let mut req = String::new();
    io::stdin().read_line(&mut req).expect("Not a valid answer");
    let request = req.clone();
    println!("{}",request);

    if request == "trapezium"{
        calculate_trap();
    } else if request == "rhombos" {
        calculate_rhom();
    } else if request == "paralellogram" {
        calculate_paralell();
    } else if request == "cube" {
        calculate_cube();
    } else if request == "cylinder" {
        calculate_cylin();
    } else {
        println!("We cant help you");
    }
    
}

fn calculate_trap (){
    let mut height_1 = String::new();
    let mut base_1 = String::new();
    let mut base_2 = String::new();

    println!("What is the height?");
    io::stdin().read_line(&mut height_1).expect("That is not a valid answer");
    let height_2:f32 = height_1.trim().parse().expect("Invalid answer");

    println!("What is the base?");
    io::stdin().read_line(&mut base_1).expect("That is not a valid answer");
    let base_3:f32 = base_1.trim().parse().expect("Invalid answer");

    println!("What is the base 2?");
    io::stdin().read_line(&mut base_2).expect("That is not a valid answer");
    let base_4:f32 = base_2.trim().parse().expect("Invalid answer");

    let area:f32 = 0.5 * height_2 * (base_3 + base_4);

    println!("{}", area);
}

fn calculate_rhom (){
    let mut diag_1 = String::new();
    let mut diag_2 = String::new();

    println!("What is the first daigonal?");
    io::stdin().read_line(&mut diag_1).expect("That is not a valid answer");
    let diag_3:f32 = diag_1.trim().parse().expect("Invalid answer");

    println!("What is the second diagonal
    ?");
    io::stdin().read_line(&mut diag_2).expect("That is not a valid answer");
    let diag_4:f32 = diag_2.trim().parse().expect("Invalid answer");

    let area:f32 = 0.5 * diag_3 * diag_4;

    println!("{}", area);
}

fn calculate_paralell (){
    let mut base_1 = String::new();
    let mut altitude_1 = String::new();

    println!("Enter the base of the paralellogram");
    io::stdin().read_line(&mut base_1).expect("That is not a valid answer");
    let base_2:f32 = base_1.trim().parse().expect("Invalid answer");

    println!("Enter the altitude of the paralellogram");
    io::stdin().read_line(&mut altitude_1).expect("That is not a valid answer");
    let altitude_2:f32 = altitude_1.trim().parse().expect("Invalid answer");

    let area:f32 = base_2 * altitude_2;
    println!("{}", area);
}

fn calculate_cube (){
    let mut base_1 = String::new();

    println!("Enter the base of the cube");
    io::stdin().read_line(&mut base_1).expect("That is not a valid answer");
    let base_2:f32 = base_1.trim().parse().expect("Invalid answer");

    let area:f32 = 6.0 * (base_2 * base_2);

    println!("{}", area);
}

fn calculate_cylin () {
    let mut radius = String::new();
    let mut height = String::new();

    println!("Enter the radius of the cylinder");
    io::stdin().read_line(&mut radius).expect("That is not a valid answer");
    let radius_2:f32 = radius.trim().parse().expect("Invalid answer");

    println!("Enter the height of the cylinder");
    io::stdin().read_line(&mut height).expect("That is not a valid answer");
    let height_2:f32 = height.trim().parse().expect("Invalid answer");

    let area:f32 = 3.14 * radius_2 * radius_2 * height_2;
    println!("{}",area);
}