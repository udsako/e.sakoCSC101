fn main() {
	let t:f64 = 450_000.00;
	let nt:f64 = t * 2.0;
	let m:f64 = 1_500_000.00;
	let nm:f64 = m * 1.0;
	let h:f64 = 750_000.00;
	let nh:f64 = h * 3.0;
	let d:f64 = 2_850_000.00;
	let nd:f64 = d * 3.0;
	let a:f64 = 250_000.00;
	let na:f64 = a * 1.0;
	let s:f64 = nt + nm + nh + nd + na;
	let avg:f64 = s / 10.0; 
	println!("Sum is {}", s); 
	println!("Avg is {}", avg);
	
}