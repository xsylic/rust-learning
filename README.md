# Jornada Rust: Lifetimes, Ownership & Borrowing

Este repositório explora os conceitos avançados de Rust, que tornam o gerenciamento de memória seguro e eficiente. Nesta branch, o foco está nos fundamentos de **ownership**, **borrowing** e **lifetimes**, além de como o Rust utiliza a memória. Vamos abordar:

- **Ownership**: Como funciona a propriedade no Rust, transferências e boas práticas.
- **Borrowing**: Emprestando referências de forma segura sem transferir propriedade.
- **Lifetimes**: Garantindo a validade das referências durante seu uso.
- **Memory Awareness**: Compreendendo os diferentes tipos de memória no Rust (stack, heap, static).

---

## Estrutura do Projeto

O projeto está organizado em diferentes arquivos, cada um abordando um conceito de forma detalhada:

- **ownership.rs**: Introdução ao conceito de propriedade no Rust.
- **borrowing.rs**: Como usar referências mutáveis e imutáveis.
- **lifetimes.rs**: Demonstrações de como lifetimes ajudam a evitar referências inválidas.
- **memory_awareness.rs**: Explicações e exemplos de como Rust aloca memória na stack, heap e memória estática.

Cada arquivo inclui:
1. O que é o conceito e por que é importante.
2. Exemplos práticos com explicações linha a linha.
3. Erros comuns e como evitá-los.
4. Boas práticas e casos de uso ideais.

### Ownership
Ownership é a base do gerenciamento de memória no Rust. Cada valor no Rust tem um **proprietário**, e apenas um proprietário é válido por vez.

**O que evitar**:
- Usar um valor após sua propriedade ter sido transferida (erro de uso pós-movimento).
  
**Exemplo de Ownership**:
```rust
let s1 = String::from("hello");
let s2 = s1; // A propriedade foi movida de `s1` para `s2`.
// println!("{}", s1); // Erro: `s1` não é mais válido.

println!("{}", s2); // Correto: `s2` é o dono.
```

---

### Borrowing
Borrowing permite emprestar uma referência a um valor sem transferir sua propriedade. Existem dois tipos:
- **Referência Imutável**: Pode ler os dados, mas não modificá-los.
- **Referência Mutável**: Pode modificar os dados, mas exige exclusividade.

**O que evitar**:
- Ter múltiplas referências mutáveis ao mesmo tempo.
- Usar uma referência mutável enquanto outras referências imutáveis existem.

**Exemplo de Borrowing**:
```rust
let mut s = String::from("hello");

// Referência imutável
let len = calculate_length(&s);
println!("Comprimento: {}", len);

// Referência mutável
change(&mut s);
println!("String modificada: {}", s);

fn calculate_length(s: &String) -> usize {
    s.len() // Acessa dados sem transferir propriedade
}

fn change(s: &mut String) {
    s.push_str(" world!"); // Modifica o valor
}
```

---

### Lifetimes
Lifetimes garantem que referências sejam válidas enquanto estiverem em uso. Elas são usadas principalmente para evitar **dangling references** (referências inválidas).

**O que evitar**:
- Criar referências que ultrapassem a vida útil do valor original.

**Exemplo de Lifetimes**:
```rust
fn main() {
    let string1 = String::from("long string");
    let string2 = String::from("short");

    let result = longest(&string1, &string2);
    println!("A string mais longa é '{}'", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

---

### Memory Awareness
Compreender como Rust gerencia a memória ajuda a escrever código eficiente. Aqui estão os tipos de memória usados no Rust:

1. **Stack**: Para variáveis com tamanho conhecido em tempo de compilação.
2. **Heap**: Para dados alocados dinamicamente.
3. **Static**: Para valores com vida útil do programa inteiro.

**Exemplo**:
```rust
fn main() {
    let x = 42; // Alocado na stack
    let s = String::from("hello"); // Alocado na heap
    static GREETING: &str = "Olá, mundo!"; // Memória estática

    println!("Stack: {}, Heap: {}, Static: {}", x, s, GREETING);
}
```

---

## Caso queira aprender também

1. Clone o repositório:
   ```bash
   git clone https://github.com/xsylic/rust-learning.git -b 3_lifetimes-ownership-borrowing
   ```

2. Navegue até o diretório do projeto:
   ```bash
   cd rust-learning
   ```

3. Compile e execute o código:
   ```bash
   cargo run
   ```

4. As funções estão distribuídas por módulos como `borrowing.rs`, `lifetimes.rs`, `memory_awareness.rs` e `ownership.rs`, e podem ser chamadas no `main.rs`.

---

## Contribuindo

Se você gostaria de contribuir para este projeto, fique à vontade para fazer um fork e enviar um pull request. Caso encontre algum problema, abra uma issue.