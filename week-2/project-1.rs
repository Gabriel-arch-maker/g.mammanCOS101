fn main() {
	let loan:f64= 520_000_000.0;
	let years:f64= 5.0;
	let rate:f64= 10.0;
	let x:f64 = (1.0 + (rate / 100.0));
	let y:f64 = x.powf(years);
	let amount = loan * y;

	let compound_interest = amount - loan;

	println!("The compound interest is ₦{}", compound_interest);
}