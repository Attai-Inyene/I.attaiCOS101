fn main() {
	let p:f64 = 520_000_000.0;
	let r:f64 = 0.10;
	let _y:f64 = 5.0;

	//for finding the compound amount
	let x = 1.0 + r;
	let t = x * x * x * x * x;
	let a = p * t;


	//for finding compound interest
	let c_i = a - p;

    println!("the rate divide through y 100 was {}", r);
	println!("the principal is {}", p);
	println!("Amount after 5 yrs is {}", a);
	println!("compound interest is {}", c_i);
}