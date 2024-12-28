use std::collections::{HashMap, HashSet};
use std::collections::BinaryHeap;
use std::collections::LinkedList;

pub fn print_collections() {
    // === Coleções ===

    // Vector
    let mut vector: Vec<i32> = Vec::new(); // Criando um vetor vazio
    vector.push(1); // Adicionando elementos ao vetor
    vector.push(2);
    vector.push(3);
    println!("Vector: {:?}", vector); // Vetor de inteiros

    // String
    let mut string = String::from("Rust"); // Criando uma string mutável
    string.push_str(" Language"); // Concatenando uma string
    println!("String: {}", string); // String final

    // HashMap
    let mut map = HashMap::new(); // Criando um HashMap vazio
    map.insert("Rust", 1); // Adicionando chave-valor
    map.insert("Go", 2);
    println!("HashMap: {:?}", map); // HashMap que mapeia strings para inteiros

    // HashSet
    let mut set = HashSet::new(); // Criando um HashSet vazio
    set.insert(1); // Adicionando elementos ao HashSet (sem duplicatas)
    set.insert(2);
    set.insert(3);
    println!("HashSet: {:?}", set); // Set que não permite valores duplicados

    // LinkedList
    let mut linked_list = LinkedList::new(); // Criando uma lista encadeada vazia
    linked_list.push_back(1); // Adicionando elementos ao final da lista
    linked_list.push_back(2);
    linked_list.push_back(3);
    println!("LinkedList: {:?}", linked_list); // Lista encadeada

    // BinaryHeap
    let mut heap = BinaryHeap::new(); // Criando um heap binário vazio
    heap.push(3); // Inserindo elementos no heap
    heap.push(1);
    heap.push(2);
    println!("BinaryHeap: {:?}", heap); // Heap binário (max-heap por padrão)

    // Stack (usando Vec como pilha)
    let mut stack: Vec<i32> = Vec::new(); // Criando uma pilha com vetor
    stack.push(1); // Empilhando elementos
    stack.push(2);
    stack.push(3);
    println!("Stack (LIFO): {:?}", stack); // Acessa o último item primeiro

    // Queue (usando VecDeque como fila)
    use std::collections::VecDeque;
    let mut queue: VecDeque<i32> = VecDeque::new(); // Criando uma fila vazia
    queue.push_back(1); // Enfileirando elementos
    queue.push_back(2);
    queue.push_back(3);
    println!("Queue (FIFO): {:?}", queue); // Acessa o primeiro item primeiro
}