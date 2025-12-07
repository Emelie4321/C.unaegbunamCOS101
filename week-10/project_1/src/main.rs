struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn calculate_cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let hp = Laptop { 
        brand: String::from("HP"), 
        price: 650_000, 
    };
    let ibm = Laptop { 
        brand: String::from("IBM"), 
        price: 755_000, 
    };
    let toshiba = Laptop { 
        brand: String::from("Toshiba"), 
        price: 550_000,
    };
    let dell = Laptop { 
        brand: String::from("Dell"), 
        price: 850_000, 
    };

    let quantity_purchased = 3;

     let total_cost = hp.calculate_cost(quantity_purchased) +
                      ibm.calculate_cost(quantity_purchased) +
                      toshiba.calculate_cost(quantity_purchased) +
                      dell.calculate_cost(quantity_purchased);
    println!("The total cost is: {}", total_cost);
}                        
        
