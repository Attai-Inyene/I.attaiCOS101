fn main() {
	let toshiba:f64 = 450_000.0;
	let mac:f64 = 1_500_000.0;
	let hp:f64 = 750_000.0;
	let dell:f64 = 2_850_000.0;
	let acer:f64 = 250_000.0;

	//to find the sum of the vales
	let a = toshiba * 2.0;
	let b = mac * 1.0;
	let c = hp * 3.0;
	let d = dell * 3.0;
	let e = acer * 1.0;
	let sum = a + b + c + d + e;

	//to find average
	let average = sum / 5.0;

	println!("the total sum is {}", sum);
	println!("the total average of the laptops is {}", average);
}