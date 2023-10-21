fn main() {
	let cost:f64= 210_000.0;
	let years:f64= 3.0;
	let rate:f64= 5.0;
	let x:f64 = (1.0 - (rate / 100.0));
	let y:f64 = x.powf(years);
	let amount = cost * y;

	//let value = cost - amount;

	println!("The new value of the television is ₦{}", amount);
	
}