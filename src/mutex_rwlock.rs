use std::sync::{Mutex, RwLock};
use std::sync::Arc;
use std::thread;

pub fn mutex_example() {
    let m = Arc::new(Mutex::new(0));

    let handles: Vec<_> = (0..10).map(|_| {
        let m = Arc::clone(&m);
        thread::spawn(move || {
            let mut num = m.lock().unwrap();
            *num += 1;
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Mutex resultado: {:?}", m.lock().unwrap());
}

pub fn rwlock_example() {
    let rw = Arc::new(RwLock::new(0));

    let handles: Vec<_> = (0..10).map(|_| {
        let rw = Arc::clone(&rw);
        thread::spawn(move || {
            let mut num = rw.write().unwrap();
            *num += 1;
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("RwLock resultado: {:?}", rw.read().unwrap());
}