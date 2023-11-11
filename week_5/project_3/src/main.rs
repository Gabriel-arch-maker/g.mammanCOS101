use std::io;
fn main() {

    let price_p:f32 = 3_200.0;
    let price_f:f32 = 3_000.0;
    let price_a:f32 = 2_500.0;
    let price_e:f32 = 2_000.0;
    let price_w:f32 = 2_500.0;

    println!("Let me have your order");

    println!("Menu                                             Price");
    println!("Poundo Yam/Edinkaiko Soup                      -₦3,200");
    println!("Fried Rice and Chicken                         -₦3,000");
    println!("Amala and Ewedu Soup                           -₦2,500");
    println!("Eba and Egusi Soup                             -₦2,000");
    println!("White Rice and Stew                            -₦2,500");

    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("That is not a valid answer");
    let order = a.trim();

    println!("How manyn do you want?");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("That is not a valid answer");
    let quantity:f32 = b.trim().parse().expect("That is not a valid answer");

    if order == "poundo yam" {
        println!("You ordered {} pack(s) of {}.", quantity, order);
        let mut total_price:f32 = price_p * quantity;
        if total_price > 10_000.0 {
            let new_price:f32 = total_price - (total_price * 0.05);
            total_price = new_price;
        } else {
            total_price = price_p * quantity;
        }
        println!("The total cost of your order is ₦{}", total_price);
    } else if order == "fried rice" {
        println!("You ordered {} pack(s) of {}.", quantity, order);
        let mut total_price:f32 = price_p * quantity;
        if total_price > 10_000.0 {
            let new_price:f32 = total_price - (total_price * 0.05);
            total_price = new_price;
        } else {
            total_price = price_p * quantity;
        }
        println!("The total cost of your order is ₦{}", total_price);
    } else if order == "amala" {
        println!("You ordered {} pack(s) of {}.", quantity, order);
        let mut total_price:f32 = price_p * quantity;
        if total_price > 10_000.0 {
            let new_price:f32 = total_price - (total_price * 0.05);
            total_price = new_price;
        } else {
            total_price = price_p * quantity;
        }
        println!("The total cost of your order is ₦{}", total_price);
    } else if order == "eba" {
        println!("You ordered {} pack(s) of {}.", quantity, order);
        let mut total_price:f32 = price_p * quantity;
        if total_price > 10_000.0 {
            let new_price:f32 = total_price - (total_price * 0.05);
            total_price = new_price;
        } else {
            total_price = price_p * quantity;
        }
        println!("The total cost of your order is ₦{}", total_price);
    } else if order == "white rice" {
        println!("You ordered {} pack(s) of {}.", quantity, order);
        let mut total_price:f32 = price_p * quantity;
        if total_price > 10_000.0 {
            let new_price:f32 = total_price - (total_price * 0.05);
            total_price = new_price;
        } else {
            total_price = price_p * quantity;
        }
        println!("The total cost of your order is ₦{}", total_price);
    } else {
        println!("You haven't made a complete order yet");
    }
}
