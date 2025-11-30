use std::fs::File;
use std::io::Write;

struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: u32,
}   

fn main() {
    let students = vec![
    Student {
        name: "Oluchi Mordi".to_string(),
        matric_number:"ACC10211111".to_string(),
        department: "Accounting".to_string(),
        level: 300,
    },
    Student {
        name: "Adams Aliyu".to_string(),
        matric_number: "ECO10110101".to_string(),
        department: "Economics".to_string(),
        level: 100,
    },
    Student {
        name: "Shania Bolade".to_string(),
        matric_number:"CSC10328828".to_string(),
        department: "Computer".to_string(),
        level: 200,
    },
    Student {
         name: "Adekunle Gold".to_string(),
         matric_number:"EE11020202".to_string(),
         department: "Electrical".to_string(),
         level: 200,
    },
    Student {
         name: "Blanca Edemoh".to_string(),
         matric_number: "MEE10202001".to_string(),
         department: "Mechanical".to_string(),
         level: 100,
        },
    ];
    println!("Students Details:");
    for student in &students {
        println!(
            "Name: {}, Matric Number: {}, Department: {}, Level: {}",
            student.name, student.matric_number, student.department, student.level
        );
    }
    
    let mut file = File::create("students.txt").expect("Could not create file");

    writeln!(file, "Stusent Name, Matric. Number, Department, Level").expect("Could not write header");

    for student in &students {
        writeln!(
            file,
            "{}, {}, {}, {}",
            student.name, student.matric_number, student.department, student.level
        )
        .expect("Could not write student record");
    }
    
    println!("Student details saved to students.txt");
}        




