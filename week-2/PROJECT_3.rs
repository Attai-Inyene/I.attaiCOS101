fn main() {
	let principal:f64 = 510_000.0;
	let rate:f64 = 5.0;
	let _time:f64 = 3.0;

	//to open the bracket first
	let r = rate / 100.0;
	let x = 1.0 - r;
	let y = x * x * x;


	//to find the amount
	let a = principal * y;

	println!("the value of the tv set after it has depreciated in value is {}", a);
}