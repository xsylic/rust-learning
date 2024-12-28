mod scalar_types;
mod compound_types;
mod collections;
mod btree;
mod channels;
mod mutex_rwlock;
mod rc_arc;

fn main() {
    println!("Jornada Rust: Scalar Types, Compound Types, Collections e Concurrency");

    println!("\n=== Scalar Types ===");
    scalar_types::print_scalar_types();

    println!("\n=== Compound Types ===");
    compound_types::print_compound_types();

    println!("\n=== Collections ===");
    collections::print_collections();

    println!("\n=== BTree Example ===");
    btree::btree_example();

    println!("\n=== Channels Example ===");
    channels::channel_example();

    println!("\n=== Mutex Example ===");
    mutex_rwlock::mutex_example();

    println!("\n=== RWLock Example ===");
    mutex_rwlock::rwlock_example();

    println!("\n=== Rc Example ===");
    rc_arc::rc_example();

    println!("\n=== Arc Example ===");
    rc_arc::arc_example();
}