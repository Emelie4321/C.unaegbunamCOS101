fn main() {
	let sales: [f64; 5] = [450000.00, 1500000.00, 750000.00, 2850000.00, 2500000.00];
	let mut sum = 0.0;
	let count = sales.len() as f64;
	for amount in sales.iter() {
		sum += amount;
	}

    let average = sum / count;

    println!("the total sum of sales is: ${}", sum);
    println!("the average sales amount is: ${}", average);
}



	


