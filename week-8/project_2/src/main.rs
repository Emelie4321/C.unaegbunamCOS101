//Rust program to find the person with the highest years of programming experience

use std::io;


fn main() {
    //Vectors to store names and years of experience
    let mut full_name:Vec<String> = Vec::new();
    let mut years_of_experience:Vec<u32> = Vec::new();

    println!("Enter the number of candidates:");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read input");
    let n: usize = num.trim().parse().expect("Invalid number");

    //Loop to collect candidate data
    for _ in 0..n {
        //Input full name
        let mut input1 = String::new();
        println!("Please enter your full name:");
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let name = input1.trim().to_string();
        full_name.push(name);

        //Input years of experience
        let mut input2 = String::new();
        println!("Please input the number of years of programming experience:");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let exp:u32 = input2.trim().parse().expect("Invalid input");
        years_of_experience.push(exp);
    }
    
    //Find candidate with the highest experience
    let mut highest_index = 0;
    for i in 1..years_of_experience.len() {
       if years_of_experience[i] > years_of_experience[highest_index] {
          highest_index = i;
        }
    }

    //Output of all candidates
    println!("\n Candidate Summary");
    for i in 0..full_name.len() {
      println!("{} has {} years of experience", full_name[i], years_of_experience[i]);
    }
    
    //Output the best candidate
    println!(
        "\nThe candidate with the highest programming experience is: {} ({} years)",
        full_name[highest_index],
        years_of_experience[highest_index]
    );
}                 

