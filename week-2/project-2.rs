fn main(){
	let toshiba:f64 = 450_000.0;
	let toshiba_quantity:f64 = 2.0;
	let toshiba_total:f64 = toshiba * toshiba_quantity;

	let mac:f64 = 1_500_000.0;
	let mac_quantity:f64 = 1.0;
	let mac_total:f64 = mac * mac_quantity;

	let hp:f64 = 750_000.0;
	let hp_quantity:f64 = 3.0;
	let hp_total:f64 = hp * hp_quantity; 

	let dell:f64 = 2_850_000.0;
	let dell_quantity:f64 = 3.0;
	let dell_total:f64 = dell * dell_quantity; 

	let acer:f64 = 250_000.0;
	let acer_quantity:f64 = 1.0;
	let acer_total:f64 = acer * acer_quantity;

	let total_quantity:f64 = toshiba_quantity + mac_quantity + hp_quantity + dell_quantity + acer_quantity;
	let total_price:f64 = toshiba_total + mac_total + hp_total + dell_total + acer_total;

	let average_price:f64 = total_price / total_quantity;

	println!("There is a total sale of ₦{} items with an average price of ₦{}", total_price, average_price);


}