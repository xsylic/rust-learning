pub fn run() {
    println!("Memory Awareness em Rust");

    let x = 42; // Stack
    let s = String::from("hello"); // Heap
    static GREETING: &str = "Olá, mundo!"; // Static

    println!("Stack: {}, Heap: {}, Static: {}", x, s, GREETING);

    demonstrate_stack();
    demonstrate_heap();
}

fn demonstrate_stack() {
    println!("=== Demonstrando Stack ===");
    let a = 10;
    let b = 20;
    println!("Variáveis na stack: a = {}, b = {}", a, b);
}

fn demonstrate_heap() {
    println!("=== Demonstrando Heap ===");
    let vec = vec![1, 2, 3]; // Heap
    println!("Vetor na heap: {:?}", vec);
}