use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter your first value: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter your second value: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the third value: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let d:f32 = (b * b) - (4.0 * (a * c));

    if d > 0.0 {
        let root1 = -b + (d.sqrt()) / (2.0 * a);
        let root2 = -b - (d.sqrt()) / (2.0 * a);
        println!("The roots are {} {}, there are two distinct roots", root1, root2);
    }
    else if d == 0.0 {
        let root1 = -b + (d.sqrt()) / (2.0 * a);
        let root2 = -b - (d.sqrt()) / (2.0 * a);
        println!("The roots are {} {}, there is one real root", root1, root2);
    }
    else if d < 0.0 {
        let root1 = -b + (d.sqrt()) / (2.0 * a);
        let root2 = -b - (d.sqrt()) / (2.0 * a);
        println!("The roots are {} {}, there are no distinct roots", root1, root2);
    }

}
