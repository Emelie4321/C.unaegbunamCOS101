//Rust program using vectors to validate staff level

fn main() {
    println!("Welcome staff!");

    // level vector
    let level = vec!["APS 1-2","APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    // Office Administration vector
    let office_ad = vec!["Intern", "Administrator","Senior Administrator", "Office Manager", "Director", "CEO"];
     

    // Academic vector
    let academic = vec!["", "Research Assistant","PHD candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];


    // lawyer vector
    let lawyer = vec!["Paralegal", "Junior Associate","Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];

    // Teacher vector
    let teacher = vec!["Placement", "Classroom Teacher","Snr Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    println!("Level Validation: ");
    
    // Loop to iterate elements in vector
    for i in 0..level.len(){
        println!("Dear staff, {} is in level {} .", office_ad[i], level[i]);
        println!("Dear staff, {} is in level {} .", academic[i], level[i]);
        println!("Dear staff, {} is in level {} .", lawyer[i], level[i]);
        println!("Dear staff, {} is in level {} .", teacher[i], level[i]);
    }
}

