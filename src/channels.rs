use std::sync::mpsc;
use std::thread;

pub fn channel_example() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("Mensagem do thread").unwrap();
    });

    println!("Recebido: {}", rx.recv().unwrap());
}