use std::rc::Rc;
use std::sync::Arc;

pub fn rc_example() {
    let rc = Rc::new(5); // Rc: contagem de referência para uso em um único thread
    println!("Valor Rc: {}", rc);
}

pub fn arc_example() {
    let arc = Arc::new(10); // Arc: contagem de referência para uso em múltiplos threads
    println!("Valor Arc: {}", arc);
}