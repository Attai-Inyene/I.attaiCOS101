use std::io;

fn main() {
    // --- 1. Define Menu and Prices (using CONST for fixed values) ---
    const PRICE_P: i32 = 3200; // Poundo Yam/Edinkaiko Soup
    const PRICE_F: i32 = 3000; // Fried Rice & Chicken
    const PRICE_A: i32 = 2500; // Amala & Ewedu Soup
    const PRICE_E: i32 = 2000; // Eba & Egusi Soup
    const PRICE_W: i32 = 2500; // White Rice & Stew

    // --- 2. Display the Menu ---
    println!("\n==============================================");
    println!("          🥘 Welcome to Attais Kitchen 🥘       ");
    println!("==============================================");
    println!("KEY | FOOD ITEM                   | PRICE (N)");
    println!("----------------------------------------------");
    println!("  P | Poundo Yam/Edinkaiko Soup   | N{},00", PRICE_P);
    println!("  F | Fried Rice & Chicken        | N{},00", PRICE_F);
    println!("  A | Amala & Ewedu Soup          | N{},00", PRICE_A);
    println!("  E | Eba & Egusi Soup            | N{},00", PRICE_E);
    println!("  W | White Rice & Stew           | N{},00", PRICE_W);
    println!("----------------------------------------------\n");

    // --- 3. Get User Input (Food Code) ---
    println!("Enter food code (P, F, A, E, W):");
    let mut food_code = String::new();
    io::stdin().read_line(&mut food_code).expect("Failed to read line");
    let food_code = food_code.trim().to_uppercase();

    // --- 4. Get User Input (Quantity) ---
    println!("Enter the quantity for your order:");
    let mut quantity_input = String::new();
    io::stdin().read_line(&mut quantity_input).expect("Failed to read line");
    let quantity: i32 = quantity_input.trim().parse().unwrap_or(0);

    // --- 5. Determine Price and Food Name (Conditional Logic) ---
    let mut item_price: i32 = 0;
    let mut food_name: &str = "Invalid Item";
    
    // Using `match` is often cleaner for single-variable comparisons, 
    // but the `if/else if` structure you used is perfectly fine too.
    if food_code == "P" {
        item_price = PRICE_P;
        food_name = "Poundo Yam/Edinkaiko Soup";
    } else if food_code == "F" {
        item_price = PRICE_F;
        food_name = "Fried Rice & Chicken";
    } else if food_code == "A" {
        item_price = PRICE_A;
        food_name = "Amala & Ewedu Soup";
    } else if food_code == "E" {
        item_price = PRICE_E;
        food_name = "Eba & Egusi Soup";
    } else if food_code == "W" {
        item_price = PRICE_W;
        food_name = "White Rice & Stew";
    } else {
        println!("\n🚨 Invalid food key entered: '{}'. Please try again.", food_code);
        return;
    }
    
    if quantity <= 0 {
        println!("\n🚨 Quantity must be greater than zero.");
        return;
    }

    // --- 6. Calculate Totals and Apply Discount ---
    let sub_total: i32 = item_price * quantity;
    let mut total_charges: f64 = sub_total as f64; // Use clearer variable name instead of 'bob'
    const DISCOUNT_RATE: f64 = 0.05; // 5%
    let mut discount_amount: f64 = 0.0;
    
    if total_charges > 10000.0 {
        discount_amount = total_charges * DISCOUNT_RATE;
        total_charges -= discount_amount; // Equivalent to: total_charges = total_charges - discount_amount
    }

    // --- 7. Display Order Summary (Unified Output Block) ---
    println!("\n---- Order Recap -----");
    println!("Food Ordered:          {}", food_name);
    println!("Price per item:        N{},00", item_price);
    println!("Quantity Bought:       {}", quantity);
    println!("Sub-Total:             N{:.2}", sub_total as f64);
    
    if discount_amount > 0.0 {
        println!("Discount Applied (5%): N{:.2}", discount_amount);
    } else {
        println!("Discount Applied:      N0.00 (Total under N10,000)");
    }

    println!("---------------------------------");
    println!("💰 **TOTAL CHARGES:  N{:.2}**", total_charges);
    println!("---------------------------------");
}