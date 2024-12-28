# Jornada Rust: Variáveis e Tipos

Este repositório é um projeto para aprender Rust, explorando os diferentes tipos de dados, desde os tipos escalares básicos até coleções mais avançadas e estruturas de dados. Nesta branch, o objetivo é ensinar como usar as várias ferramentas do Rust para trabalhar com dados, além de entender os conceitos fundamentais por trás de cada tipo de dado e coleção (Por favor, corrija-me caso eu esteja errada e faça uma PR! As informações foram retiradas de videoaulas e da própria documentação).

## Estrutura do Projeto

O projeto está organizado em vários arquivos que demonstram como usar diferentes tipos de dados e coleções em Rust. Cada arquivo contém exemplos e explicações de como e quando utilizar cada tipo.

- **scalar_types.rs**: Exemplo e explicação de tipos escalares (inteiros, ponto flutuante, booleanos e caracteres).
- **compound_types.rs**: Demonstração de tipos compostos como tuplas e arrays.
- **collections.rs**: Demonstração de coleções mais complexas como `Vec`, `String`, `HashMap`, `HashSet`, `LinkedList`, `BinaryHeap`, `Stack` e `Queue`.
- **utils.rs**: Funções utilitárias para depuração e verificações durante a execução.

## Tipos Escalares

Os **tipos escalares** são os tipos de dados mais simples, representando um único valor. Eles são fundamentais para qualquer linguagem de programação e são amplamente utilizados em Rust para cálculos numéricos, controle de fluxo e muito mais.

### 1. Inteiros (signed e unsigned)

Inteiros podem ser **signed** (com sinal) ou **unsigned** (sem sinal). Inteiros signed podem armazenar valores negativos, enquanto os unsigned só podem armazenar valores positivos e zero. Rust oferece diferentes tamanhos de inteiros, para que você possa escolher o tipo mais adequado para o seu caso.

#### Exemplos:

```rust
let signed_integer: i32 = -42; // Um inteiro com sinal, pode ser negativo.
let unsigned_integer: u32 = 42; // Um inteiro sem sinal, apenas positivo.
```

- **signed**: `i8`, `i16`, `i32`, `i64`, `i128`
- **unsigned**: `u8`, `u16`, `u32`, `u64`, `u128`

Usamos inteiros para armazenar valores numéricos que não requerem casas decimais, como contagens, id's, ou resultados de operações aritméticas.

### 2. Ponto Flutuante

Os números de ponto flutuante são usados para representar números que precisam de casas decimais. Rust oferece dois tipos de ponto flutuante com diferentes precisões:

- **f32**: Ponto flutuante de precisão simples (32 bits).
- **f64**: Ponto flutuante de precisão dupla (64 bits).

#### Exemplos:

```rust
let pi: f64 = 3.14159;  // Um número de ponto flutuante de precisão dupla.
let euler: f32 = 2.71828; // Um número de ponto flutuante de precisão simples.
```

O tipo `f64` é o tipo padrão para ponto flutuante em Rust, pois oferece maior precisão.

### 3. Booleano

O tipo **booleano** (`bool`) tem dois valores possíveis: `true` e `false`. É usado para controle de fluxo e expressões condicionais.

#### Exemplos:

```rust
let is_rust_fun: bool = true;  // Um valor booleano indicando se Rust é divertido.
```

É usado principalmente em estruturas de controle como `if` e `while`.

### 4. Caracteres

O tipo **char** em Rust é usado para armazenar um único caractere Unicode. Isso significa que você pode representar qualquer caractere, como letras, números ou até emojis.

#### Exemplos:

```rust
let emoji: char = '🔥';  // Um emoji.
let letter: char = 'R';   // Um caractere alfabético.
```

O tipo `char` é mais flexível que o tipo `String`, pois pode armazenar caracteres de qualquer sistema de escrita.

---

## Tipos Compostos

Os **tipos compostos** são utilizados para agrupar múltiplos valores em uma única estrutura. Em Rust, os principais tipos compostos são **tuplas** e **arrays**.

### 1. Tuplas

As **tuplas** são tipos compostos que podem armazenar valores de diferentes tipos. Você pode acessar os valores de uma tupla usando a desestruturação ou os índices.

#### Exemplos:

```rust
let tuple_example: (i32, f64, char) = (42, 3.14, 'A');
let (x, y, z) = tuple_example; // Desestruturação da tupla

println!("Tuple: ({}, {}, {})", x, y, z); // Acessando valores da tupla
```

As tuplas são úteis quando você precisa agrupar valores de tipos diferentes que não precisam ser manipulados como uma coleção homogênea (como no caso de arrays ou vetores).

### 2. Arrays

Os **arrays** são coleções de elementos de mesmo tipo e tamanho fixo. Eles são mais eficientes que os vetores para armazenar um número fixo de elementos, pois o tamanho é conhecido em tempo de compilação.

#### Exemplos:

```rust
let days_in_week: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
println!("Primeiro dia da semana: {}", days_in_week[0]); // Acessando o primeiro valor do array
```

O uso de arrays é indicado quando você conhece o número exato de elementos e não precisa adicionar ou remover elementos dinamicamente.

---

## Coleções

As **coleções** são tipos de dados que armazenam múltiplos valores. Em Rust, existem várias coleções, e cada uma tem seus próprios usos dependendo da situação.

### 1. `Vec` (Vetor)

O **`Vec`** (abreviação de "vector") é uma coleção que pode crescer dinamicamente. Usamos `Vec` quando não sabemos o número de elementos com antecedência e precisamos de uma coleção que pode mudar de tamanho durante a execução.

#### Exemplos:

```rust
let mut v: Vec<i32> = Vec::new(); // Criando um vetor vazio
v.push(10); // Adicionando um valor ao vetor
v.push(20);

println!("{:?}", v); // Exibindo o vetor
```

O `Vec` é útil quando você precisa de uma lista mutável de valores que pode aumentar ou diminuir conforme necessário.

### 2. `String`

A **`String`** em Rust é uma sequência de caracteres mutáveis. Use `String` quando precisar manipular texto dinâmico (por exemplo, concatenação de strings).

#### Exemplos:

```rust
let mut hello = String::from("Hello");
hello.push_str(", world!"); // Concatenando outra string
println!("{}", hello); // Exibindo a string resultante
```

Enquanto `&str` é imutável e eficiente para texto estático, `String` é mais flexível para trabalhar com texto mutável.

### 3. `HashMap`

O **`HashMap`** é uma coleção de pares chave-valor. Ele oferece uma busca muito rápida associando uma chave única a um valor. Ideal para situações onde você precisa acessar dados rapidamente usando uma chave.

#### Exemplos:

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("nome", "João");
map.insert("idade", "30");

println!("{}", map["nome"]); // Acessando o valor usando a chave
```

Use `HashMap` quando precisar associar uma chave a um valor e realizar buscas rápidas com base nessa chave.

### 4. `HashSet`

O **`HashSet`** é uma coleção que armazena valores únicos, sem duplicatas. Se você precisar garantir que um conjunto de dados contenha apenas elementos distintos, use `HashSet`.

#### Exemplos:

```rust
use std::collections::HashSet;

let mut set = HashSet::new();
set.insert(1);
set.insert(2);
set.insert(3);
set.insert(2); // Duplicata, não será inserida

println!("{:?}", set); // Mostra {1, 2, 3}
```

### 5. `LinkedList`

A **`LinkedList`** é uma lista encadeada. A principal vantagem da `LinkedList` é que ela permite inserções e remoções eficientes no início ou meio da lista, ao contrário de `Vec`, que é mais eficiente no final.

#### Exemplos:

```rust
use std::collections::LinkedList;

let mut list = LinkedList::new();
list.push_back(1);
list.push_front(2); // Adiciona no início

println!("{:?}", list); // Mostra [2, 1]
```

Use `LinkedList` quando precisar de operações de inserção e remoção frequentes no início ou no meio da coleção.

### 6. `BinaryHeap`

O **`BinaryHeap`** é uma estrutura de dados baseada em heap binário. Ele é usado principalmente para criar uma fila de prioridade, onde o maior ou menor valor é sempre acessado primeiro.

#### Exemplos:

```rust
use std::collections::BinaryHeap;

let mut heap = BinaryHeap::new();
heap.push(1);
heap.push(5);
heap.push(3);

println!("{:?}", heap); // O maior valor será acessado primeiro
```

### 7. `Stack` e `Queue`

Rust não tem uma estrutura de dados Stack ou Queue dedicada por padrão, mas podemos usá-las facilmente usando `Vec` ou `VecDeque`.

- **Stack**: Usado com `Vec`, funciona com a lógica LIFO (Last-In, First-Out).
- **Queue**: Usado com `VecDeque`, funciona com a lógica FIFO (First-In, First-Out).

#### Exemplos:

```rust
// Stack (LIFO)
let mut stack = Vec::new();
stack.push(1);
stack.push(2);
let top = stack.pop(); // Remove o último item

// Queue (FIFO)
use std::collections::VecDeque;
let mut queue = VecDeque::new();
queue.push_back(1);
queue.push_back(2);
let first = queue.pop_front(); // Remove o primeiro item
```

---

## Caso queira aprender também

1. Clone o repositório:
   ```bash
   git clone https://github.com/xsylic/rust-learning.git -b 1_variables-and-types
   ```

2. Navegue até o diretório do projeto:
   ```bash
   cd rust-learning
   ```

3. Compile e execute o código:
   ```bash
   cargo run
   ```

4. As funções estão distribuídas por módulos como `scalar_types.rs`, `compound_types.rs`, e `collections.rs`, e podem ser chamadas no `main.rs`.

---

## Contribuindo

Se você gostaria de contribuir para este projeto, fique à vontade para fazer um fork e enviar um pull request. Caso encontre algum problema, abra uma issue.