/**
 * Aprendendo Rust: Variáveis e Tipos
 *
 * @author xsylic <xsylic@proton.me>
 */

const ONE_DAY: i8 = 1;

fn main() {
    // === Scalar Types ===
    // Integers
    let signed_integer: i32 = -42; // signed (aceita valores negativos)
    let unsigned_integer: u32 = 42; // unsigned (apenas valores positivos ou zero)

    // Floating Point
    let pi: f64 = 3.14159; // Double-precision (64 bits)
    let euler: f32 = 2.71828; // Single-precision (32 bits)

    // Booleans
    let is_rust_fun: bool = true;

    // Caracters
    let emoji: char = '🔥';
    let letter: char = 'R';

    println!(
        "Inteiros: signed = {}, unsigned = {}",
        signed_integer, unsigned_integer
    );
    println!("Pontos flutuantes: pi = {}, euler = {}", pi, euler);
    println!("Booleano: {}", is_rust_fun);
    println!("Caracteres: {}, {}", emoji, letter);

    // === Compound Types ===
    // Tuples
    let tuple_example: (i32, f64, char) = (42, 3.14, 'A');
    let (x, y, z) = tuple_example; // Desestruturação
    println!("Tuple: ({}, {}, {})", x, y, z);

    // Arrays
    let days_in_week: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    println!("Primeiro dia da semana: {}", days_in_week[0]);

    // === Inteiros (signed e unsigned) ===
    // Inteiros assinados (signed)
    let signed_8bit: i8 = -128; // Valores de -128 a 127
    let signed_16bit: i16 = -32_768; // Valores de -32,768 a 32,767

    // Inteiros não assinados (unsigned)
    let unsigned_8bit: u8 = 255; // Valores de 0 a 255
    let unsigned_16bit: u16 = 65_535; // Valores de 0 a 65,535

    println!(
        "Signed: i8 = {}, i16 = {}, Unsigned: u8 = {}, u16 = {}",
        signed_8bit, signed_16bit, unsigned_8bit, unsigned_16bit
    );

    // Testes e aprendizados com `debug_assert`
    let two_days /* Possível pôr ":i8" mas o compilador automaticamente já faz isso */ = ONE_DAY * 2;
    let seven_days: i8 = ONE_DAY * 7;

    {
        /* Escopo de lifetime */
        let mut five_days: i8 = ONE_DAY;
        five_days *= 5; /* Com a variável ONE_DAY (que cujo valor é um), é multiplicada por cinco para dar 5 dias */
        debug_assert_eq!(
            five_days, 5,
            "Esperava que fosse {} dias, mas foi obtido: {}",
            5, five_days
        );

        println!("Cinco dias: {}!", five_days);
    }

    println!("Dois dias: {}!", two_days);
    debug_assert_eq!(
        two_days, 2,
        "Esperava que fosse {} dias, mas foi obtido: {}",
        2, two_days
    );

    println!("Sete dias: {}!", seven_days);
    debug_assert_eq!(
        seven_days, 7,
        "Esperava que fosse {} dias, mas foi obtido: {}",
        7, seven_days
    );
}