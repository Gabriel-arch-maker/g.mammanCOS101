use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    let mut researchers_count:i32 = 0;

    while researchers_count < 500 {
        println!("*****This program can only be executed for 500 researchers*****");
        println!("Current number of researchers this program has been executed for: {}", researchers_count);
        researchers_count += 1;

        println!("Please, input your name as it appears in your passport.");
        io::stdin().read_line(&mut input1).expect("Invalid string");
        let name = input1.trim();

        println!("How many books have you published? ");
        io::stdin().read_line(&mut input2).expect("Invalid string");
        let number_of_books:i32 = input2.trim().parse().expect("Error converting string to int");

        if number_of_books >= 3 && number_of_books   <= 5 {
            println!("Your name is {} and your salary is salary is N500,000", name);
        } else if number_of_books > 5 && number_of_books < 10 {
            println!("Your name is {} and your salary is N800,000", name);
        } else if number_of_books >= 10 {
            println!("Your name is {} and your salary is N1,000,000", name);
        } else {
            println!("Your name is {} and your salary is N100,000", name);
        }

        println!("Any other researchers?");
        io::stdin().read_line(&mut input3).expect("Invalid string");
        let continue_loop = input3.trim();

        if continue_loop.to_lowercase() == "yes" {
            println!("Alright. Next researcher!");
            continue;
        } else {
            println!("Thank you for using my program.");
            break;
        }
    }
}