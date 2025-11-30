use std::fs::File;
use std::io::Write;

fn main() {
    let efcc = vec![
    ("Aigbogun Alamba Daudu", "Internal Affairs", "South West"),
    ("Murtala Afeez Bendu", "Justice", "North East"),
    ("Okorocha Calistus Ogbonna", "Defense", "South South"),
    ("Adewale Jimoh Akanbi", "Power & Steel", "South West"),
    ("Osazuwa Faith Etieye", "Petroleum", "South East")
    ];
    let mut file = File::create("convicted_criminalsxcommissioners.csv").unwrap();
    file.write_all("\nName, Ministry, Geopolitical Zone\n".as_bytes()).unwrap();
    for (name, ministry, zone) in efcc {
        let line = format!("\n{}, {}, {}\n", name, ministry, zone);
        file.write_all(line.as_bytes());
    }
    println!("Saved all in single dataset");    
}
