use std::io;

fn main() {
    loop {
    println!("Annual incentive");

    println!("Type in the name of the entity");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("failed to read input");
    let name = name.trim();

    println!("Type in the age of the entity");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("failed to read input");
    let age:f32 = age.trim().parse().expect("invalid input");

    println!("Are they experienced? (yes/no)");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("failed to read input");
    let experience = experience.trim().to_lowercase();

    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Experience: {}", experience);

    if experience == "no"  {
        println!("Annual Incentive: 100,000");
    } else if experience == "yes" {
        if age >= 40.0 {
            println!("Annual Incentive: 1,560,000");
        } else if age >= 30.0 && age < 40.0 {
            println!("Annual Incentive: 1,480,000");
        } else if age < 28.0 {
            println!("Annual Incentive: 1,300,000");
        }
    } 
    else {
        println!("Invalid input. Please type yes/no");
        continue;
    }
    println!("Do you want to enter another record? (yes/no)");
    let mut again = String::new();
    io::stdin().read_line(&mut again).expect("Please, enter a valid input");
    let again = again.trim().to_lowercase();
    if again != "yes"{
        break;
    }
}
}

