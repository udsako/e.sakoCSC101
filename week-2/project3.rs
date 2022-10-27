fn main() {
	let p:f64 = 210_000.00;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	// Depreciation
	let a = 1.0 - (r / 100.0);
	let a = f64::powf(a, n);
	let b = p * a;
	println!("Amount is {}", b);
	let dep = b - p;
	println!("Depreciation is {}", dep);

}