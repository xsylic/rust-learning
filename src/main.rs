mod ownership;
mod borrowing;
mod lifetimes;
mod memory_awareness;

fn main() {
    println!("Jornada Rust: Lifetimes, Ownership & Borrowing");

    println!("\n=== Ownership ===");
    ownership::run();

    println!("\n=== Borrowing ===");
    borrowing::run();

    println!("\n=== Lifetimes ===");
    lifetimes::run();

    println!("\n=== Memory Awareness ===");
    memory_awareness::run();
}