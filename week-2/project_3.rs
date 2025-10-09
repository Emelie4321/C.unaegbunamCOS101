fn main() {
	let p: f64 = 510000.0;
	let r: f64 = 5.0;
	let n: u32 = 3;

	let a = p * (1.0 - r / 100.0).powi(n as i32);

	println!("The value of the TV after {} years is N{:.2}", n, a);
}	