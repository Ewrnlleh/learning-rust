use std::io;

// Customer structure
struct Customer {
    name: String,
    surname: String,
    balance: f64,
}

// Product structure
struct Product {
    name: String,
    price: f64,
    stock: u32,
}

// Market functions
impl Customer {
    // Create a new customer
    fn new(name: String, surname: String, balance: f64) -> Customer {
        Customer {
            name,
            surname,
            balance,
        }
    }

    // Function for purchasing a product
    fn buy_product(&mut self, product: &mut Product, quantity: u32) -> bool {
        let total_cost = product.price * quantity as f64;
        
        // Check if customer has enough balance and product has enough stock
        if self.balance >= total_cost && product.stock >= quantity {
            // Deduct money from customer balance
            self.balance -= total_cost;
            // Reduce product stock
            product.stock -= quantity;
            
            println!(
                "{} {} purchased {} units of {} for ${:.2}. Remaining balance: ${:.2}",
                self.name, self.surname, quantity, product.name, total_cost, self.balance
            );
            return true;
        } else if self.balance < total_cost {
            println!(
                "{} {} doesn't have enough balance. Required: ${:.2}, Available: ${:.2}",
                self.name, self.surname, total_cost, self.balance
            );
            return false;
        } else {
            println!(
                "Not enough stock for {}. Requested: {}, Available: {}",
                product.name, quantity, product.stock
            );
            return false;
        }
    }

    // Display customer info
    fn display_info(&self) {
        println!(
            "Customer: {} {}, Balance: ${:.2}",
            self.name, self.surname, self.balance
        );
    }
}

impl Product {
    // Create a new product
    fn new(name: String, price: f64, stock: u32) -> Product {
        Product {
            name,
            price,
            stock,
        }
    }

    // Display product info
    fn display_info(&self) {
        println!(
            "Product: {}, Price: ${:.2}, Stock: {} units",
            self.name, self.price, self.stock
        );
    }
}

// Market system functions
fn display_menu() {
    println!("\n=== MARKET SYSTEM ===");
    println!("1. Display all customers");
    println!("2. Display all products");
    println!("3. Customer 1 buy product");
    println!("4. Customer 2 buy product");
    println!("5. Add new product");
    println!("6. Exit");
    println!("Choose an option (1-6): ");
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

// Main program
fn main() {
    // Create sample customers
    let mut customer1 = Customer::new(
        String::from("Alice"),
        String::from("Johnson"),
        150.0
    );

    let mut customer2 = Customer::new(
        String::from("Bob"),
        String::from("Smith"),
        200.0
    );

    // Create sample products
    let mut products = vec![
        Product::new(String::from("Apple"), 2.5, 10),
        Product::new(String::from("Bread"), 3.0, 15),
        Product::new(String::from("Milk"), 4.5, 8),
    ];

    println!("Welcome to the Market System!");
    
    // Initial purchases as per template
    println!("\n=== Initial Purchases ===");
    
    println!("Customer 1 is buying a product...");
    if customer1.buy_product(&mut products[0], 3) {
        println!("Customer 1 successfully purchased the product.");
    } else {
        println!("Customer 1 couldn't purchase the product.");
    }

    println!("\nCustomer 2 is buying a product...");
    if customer2.buy_product(&mut products[1], 8) {
        println!("Customer 2 successfully purchased the product.");
    } else {
        println!("Customer 2 couldn't purchase the product.");
    }

    // Interactive menu loop
    loop {
        display_menu();
        let choice = get_user_input();

        match choice.as_str() {
            "1" => {
                println!("\n=== CUSTOMERS ===");
                customer1.display_info();
                customer2.display_info();
            }
            "2" => {
                println!("\n=== PRODUCTS ===");
                for (i, product) in products.iter().enumerate() {
                    print!("{}. ", i + 1);
                    product.display_info();
                }
            }
            "3" => {
                println!("\n=== Customer 1 Purchase ===");
                customer1.display_info();
                println!("\nAvailable products:");
                for (i, product) in products.iter().enumerate() {
                    print!("{}. ", i + 1);
                    product.display_info();
                }
                
                print!("Select product (1-{}): ", products.len());
                let product_choice = get_user_input();
                if let Ok(index) = product_choice.parse::<usize>() {
                    if index > 0 && index <= products.len() {
                        print!("Enter quantity: ");
                        let quantity_input = get_user_input();
                        if let Ok(quantity) = quantity_input.parse::<u32>() {
                            customer1.buy_product(&mut products[index - 1], quantity);
                        } else {
                            println!("Invalid quantity!");
                        }
                    } else {
                        println!("Invalid product selection!");
                    }
                } else {
                    println!("Invalid input!");
                }
            }
            "4" => {
                println!("\n=== Customer 2 Purchase ===");
                customer2.display_info();
                println!("\nAvailable products:");
                for (i, product) in products.iter().enumerate() {
                    print!("{}. ", i + 1);
                    product.display_info();
                }
                
                print!("Select product (1-{}): ", products.len());
                let product_choice = get_user_input();
                if let Ok(index) = product_choice.parse::<usize>() {
                    if index > 0 && index <= products.len() {
                        print!("Enter quantity: ");
                        let quantity_input = get_user_input();
                        if let Ok(quantity) = quantity_input.parse::<u32>() {
                            customer2.buy_product(&mut products[index - 1], quantity);
                        } else {
                            println!("Invalid quantity!");
                        }
                    } else {
                        println!("Invalid product selection!");
                    }
                } else {
                    println!("Invalid input!");
                }
            }
            "5" => {
                println!("\n=== Add New Product ===");
                print!("Enter product name: ");
                let name = get_user_input();
                
                print!("Enter product price: ");
                let price_input = get_user_input();
                if let Ok(price) = price_input.parse::<f64>() {
                    print!("Enter stock quantity: ");
                    let stock_input = get_user_input();
                    if let Ok(stock) = stock_input.parse::<u32>() {
                        products.push(Product::new(name, price, stock));
                        println!("Product added successfully!");
                    } else {
                        println!("Invalid stock quantity!");
                    }
                } else {
                    println!("Invalid price!");
                }
            }
            "6" => {
                println!("Thank you for using the Market System!");
                break;
            }
            _ => {
                println!("Invalid option! Please choose 1-6.");
            }
        }
    }
}
