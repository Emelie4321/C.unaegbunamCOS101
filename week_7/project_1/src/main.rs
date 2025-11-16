use std::io;
fn read_number(prompt: &str) -> f32 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("invalid string");
    let input:f32 = input.trim().parse().expect("invalid input");
    input
}

fn help_you_solve_a_problem() {
    let aot = "Area of Trapezium";
    let aor = "Area of Rhombus";
    let aop = "Area of parallelogram";
    let aoc = "Area of  Cube";
    let voc = "Volume of Cube";
    println!("{}, \n{}, \n{}, \n{}, \n{}", aot, aor, aop, aoc, voc);

    println!("Choose an equation:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("invalid string");
    let input1 = input1.trim();

    if input1 == aot {
        let a = read_number("Enter the value of a:");
        let b = read_number("Enter the value of b:");
        let h = read_number("Enter the value of h:");
        let areaoft = 0.5 * a * b * h;
        println!("Area of a Trapezium: {}", areaoft);
    }
    else if input1 == aor {
        let d1 = read_number("Enter the value of d1:");
        let d2 = read_number("Enter the value of d2:");
        let areaofr = 0.5 * d1 * d2;
        println!("Area of a Rhombus: {}", areaofr);
    }
    else if input1 == aop {
        let b = read_number("Enter the value of b:");
        let h = read_number("Enter the value of h:");
        let areaofp = b * h;
        println!("Area of a Parallelogram: {}", areaofp);
    }
    else if input1 == aoc {
        let l = read_number("Enter the value of l:");
        let areaofc = 6.0 * l.powf(2.0);
        println!("Area of a Cube: {}", areaofc);
    }
    else if input1 == voc {
        let l = read_number("Enter the value of l:");
        let volumeofc = l.powf(3.0);
        println!("Volume of a Cylinder: {}", volumeofc);
    }
    else {
        println!("Not found, try again");
    }
}

fn main() {
    loop {
        println!("Hi, this is a Rust program to help you calculate the area and volume of shapes");
        help_you_solve_a_problem();
        println!("Do you want to input another formula?(yes/no)");
        let mut again = String::new();
        io::stdin().read_line(&mut again).expect("invalid input");
        let again = again.trim().to_lowercase();
        if again != "yes" {
            break;
        }
    }
}                                
