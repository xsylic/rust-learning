pub fn print_scalar_types() {
    // === Scalar Types ===
    // Inteiros
    let signed_integer: i32 = -42; // Inteiro com sinal (pode ser negativo ou positivo)
    let unsigned_integer: u32 = 42; // Inteiro sem sinal (apenas valores positivos ou zero)

    // Pontos flutuantes
    let pi: f64 = 3.14159; // Ponto flutuante de precisão dupla (64 bits)
    let euler: f32 = 2.71828; // Ponto flutuante de precisão simples (32 bits)

    // Booleanos
    let is_rust_fun: bool = true; // Representa valores verdadeiro ou falso

    // Caracteres
    let emoji: char = '🔥'; // Um caractere Unicode (pode ser um emoji ou qualquer símbolo)
    let letter: char = 'R'; // Um caractere simples

    // Imprimindo os valores
    println!(
        "Inteiros: signed = {}, unsigned = {}",
        signed_integer, unsigned_integer
    );
    println!("Pontos flutuantes: pi = {}, euler = {}", pi, euler);
    println!("Booleano: {}", is_rust_fun);
    println!("Caracteres: {}, {}", emoji, letter);
}