use std::io;

fn main() {

    println!("********** Please note that this discount conditions only applies to the first 100 patients **********");
    let mut name_1 = String::new();
    let mut date_of_birth_1 = String::new();
    let mut email_address_1 = String::new();
    let mut phone_number_1 = String::new();
    let mut number_of_siblings_1 = String::new();
    let mut number_of_children_1 = String::new();
    let mut medical_diagnosis_1 = String::new();
    let mut village_1 = String::new();
    let currentYear:i32 = 2023;  

    println!("Please your name:");
    io::stdin().read_line(&mut name_1).expect("Not a valid answer");
    let name_2 = name_1.trim();
    
    println!("Please your date of birth:");
    io::stdin().read_line(&mut date_of_birth_1).expect("Not a valid answer");
    let date_of_birth_2:i32 = date_of_birth_1.trim().parse().expect("Not a valid answer");
    let age = currentYear - date_of_birth_2;
    println!("{}",age);

    println!("Please your email address:");
    io::stdin().read_line(&mut email_address_1).expect("Not a valid answer");
    let email_address_2 = email_address_1.trim();

    println!("Please your phone number:");
    io::stdin().read_line(&mut phone_number_1).expect("Not a valid answer");
    let phone_number_2 = phone_number_1.trim();

    println!("Please your number of siblings:");
    io::stdin().read_line(&mut number_of_siblings_1).expect("Not a valid answer");
    let number_of_siblings_2:i32 = number_of_siblings_1.trim().parse().expect("Not a valid answer");

    println!("Please your number of children:");
    io::stdin().read_line(&mut number_of_children_1).expect("Not a valid answer");
    let number_of_children_2:i32 = number_of_children_1.trim().parse().expect("Not a valid answer");

    println!("Please your medical diagnosis:");
    io::stdin().read_line(&mut medical_diagnosis_1).expect("Not a valid answer");
    let medical_diagnosis_2 = medical_diagnosis_1.trim();    

    println!("Please your village:");
    io::stdin().read_line(&mut village_1).expect("Not a valid answer");
    let village_2 = village_1.trim();

    if medical_diagnosis_2 == "alzheimer" && age >50 && number_of_children_2 > 4 && village_2 == "akpabom"{
        println!("Your discount is going to be 20%");
        let discount:f32 = 0.2;
        let amount:f32 = 1_200_000.0;
        let newAmount:f32 = amount - (amount * discount);

        println!("Your new amount is ₦{}", newAmount);
    } else if medical_diagnosis_2 == "arrythmia" && age >30 && number_of_siblings_2 > 4 && village_2 == "ngbauji" {
        println!("Your discount is going to be 5%");
        let discount:f32 = 0.05;
        let amount:f32 = 550_000.0;
        let newAmount:f32 = amount - (amount * discount);

        println!("Your new amount is ₦{}", newAmount);
    } else if medical_diagnosis_2 == "kidney disease" && age >40 && number_of_siblings_2 > 3 && number_of_children_2 > 3 && village_2 == "atabrikang" {
        println!("Your discount is going to be 15%");
        let discount:f32 = 0.15;
        let amount:f32 = 1_500_000.0;
        let newAmount:f32 = amount - (amount * discount);

        println!("Your new amount is ₦{}", newAmount);
    } else if medical_diagnosis_2 == "diabetes" && age >28 && age <45 && number_of_children_2 > 3 && number_of_children_2 < 4 && village_2 == "okorobilom" {
        println!("Your discount is going to be 10%");
        let discount:f32 = 0.10;
        let amount:f32 = 800_000.0;
        let newAmount:f32 = amount - (amount * discount);

        println!("Your new amount is ₦{}", newAmount);
    } else if medical_diagnosis_2 == "arthritis" && age > 58 && number_of_siblings_2 > 5 && number_of_children_2 > 5 && village_2 == "emeremen" {
        println!("Your discount is going to be 10%");
        let discount:f32 = 0.10;
        let amount:f32 = 450_000.0;
        let newAmount:f32 = amount - (amount * discount);

        println!("Your new amount is ₦{}", newAmount);
    } else {
        println!("I'm sorry, you are not eligible for any discount plan 💀💀")
    }
}
