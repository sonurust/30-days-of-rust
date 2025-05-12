fn main() {
    println!("Hello, Rust World!");
    
    // Print some information about Rust
    println!("Welcome to my 30 Days of Rust journey!");
    println!("Day 01: Getting started with the basics");
    
    // Using variables
    let language = "Rust";
    let year_created = 2010;
    
    println!("{} was first released in {}.", language, year_created);
    println!("It's designed for safety, concurrency, and performance.");
    
    // Using a simple loop
    println!("\nCounting to 5:");
    for i in 1..=5 {
        println!("  {}", i);
    }
    
    println!("\nExcited to learn more tomorrow!");
}