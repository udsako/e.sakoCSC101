fn main() {
	let p:f64 = 520_000_000.00;
	let r:f64 = 10.00;
	let n:f64 = 5.00;

	// compound interest
	let a = 1.0 + (r / 100.0);
	let a = f64::powf(a, n);
	let b = p * a;
	println!("Amount is {}", b);
	let ci = b - p;
	println!("Compound Interest is {}", ci);
}