pub fn print_compound_types() {
    // === Compound Types ===
    // Tuplas
    let tuple_example: (i32, f64, char) = (42, 3.14, 'A');
    let (x, y, z) = tuple_example; // Desestruturação, extraindo os valores da tupla
    println!("Tuple: ({}, {}, {})", x, y, z); // Imprime a tupla

    // Arrays
    let days_in_week: [i32; 7] = [1, 2, 3, 4, 5, 6, 7]; // Array fixo com 7 elementos
    println!("Primeiro dia da semana: {}", days_in_week[0]); // Acessando o primeiro elemento do array
}