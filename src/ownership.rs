pub fn run() {
    println!("Ownership em Rust");

    let s1 = String::from("hello");
    let s2 = s1; // Transferência de propriedade
    // println!("{}", s1); // Erro: `s1` não é mais válido.

    println!("s2 é o dono: {}", s2);

    // Função que toma posse
    let s3 = String::from("world");
    takes_ownership(s3);
    // println!("{}", s3); // Erro: `s3` não é mais válido.

    // Retornando propriedade
    let s4 = String::from("Rust");
    let s5 = gives_ownership(s4);
    println!("s5 agora é o dono: {}", s5);
}

fn takes_ownership(s: String) {
    println!("Toma posse de: {}", s);
}

fn gives_ownership(s: String) -> String {
    s
}