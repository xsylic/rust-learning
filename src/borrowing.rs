pub fn run() {
    println!("Borrowing em Rust");

    let mut s = String::from("hello");

    // Referência imutável
    let len = calculate_length(&s);
    println!("Comprimento de '{}': {}", s, len);

    // Referência mutável
    change(&mut s);
    println!("String modificada: {}", s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" world!");
}