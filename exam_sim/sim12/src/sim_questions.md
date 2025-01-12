# Simulazione d'Esame - Rust

**Data: DD/MM/YYYY**

---

## Domanda 1: Borrow Checker e Smart Pointers

### Domanda
Considera il seguente programma:
```rust
use std::rc::Rc;

struct Node {
    value: i32,
    next: Option<Rc<Node>>,
}

fn main() {
    let node1 = Rc::new(Node { value: 10, next: None });
    let node2 = Rc::new(Node { value: 20, next: Some(node1.clone()) });

    let node3 = Rc::new(Node { value: 30, next: Some(node2.clone()) });
    println!("Node 3 points to {:?}", node3.next);
}
```
1. Spiega il funzionamento del codice e il ruolo di `Rc<T>`.
2. Quali problemi sorgerebbero se si utilizzasse un semplice puntatore anziché `Rc<T>`?

### Soluzione

---

## Domanda 2: Concorrenza e Mutex

### Domanda
Esamina il seguente programma e descrivi il suo comportamento.
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter: {}", *counter.lock().unwrap());
}
```
1. Perché è necessario utilizzare `Arc<Mutex<T>>` in questo esempio?
2. Cosa accadrebbe se il `Mutex` fosse sostituito da una semplice variabile intera?

### Soluzione

---

## Domanda 3: Gestione dei Thread

### Domanda
Completa l'implementazione di una barriera ciclica per N thread, dove ogni thread deve aspettare che tutti gli altri raggiungano un certo punto prima di procedere. Usa il seguente scheletro:
```rust
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct CyclicBarrier {
    count: Mutex<usize>,
    condvar: Condvar,
    total: usize,
}

impl CyclicBarrier {
    fn new(total: usize) -> Self {
        CyclicBarrier {
            count: Mutex::new(0),
            condvar: Condvar::new(),
            total,
        }
    }

    fn wait(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
        if *count == self.total {
            *count = 0; // Reset for the next cycle
            self.condvar.notify_all();
        } else {
            self.condvar.wait(count).unwrap();
        }
    }
}

fn main() {
    let barrier = Arc::new(CyclicBarrier::new(3));
    let mut handles = vec![];

    for i in 0..3 {
        let barrier = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            println!("Thread {} waiting", i);
            barrier.wait();
            println!("Thread {} proceeding", i);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```
1. Descrivi come funziona la sincronizzazione.
2. Quali modifiche sono necessarie per aggiungere un timeout alla barriera?

### Soluzione

---

## Domanda 4: Programmazione - Tratti

### Domanda
Crea un tratto `Shape` che definisca:
1. Un metodo `area(&self) -> f64` per calcolare l'area.
2. Un metodo `perimeter(&self) -> f64` per calcolare il perimetro.

Implementa questo tratto per due strutture: `Circle` (con raggio) e `Rectangle` (con larghezza e altezza). Scrivi un programma che utilizza polimorfismo per calcolare l'area e il perimetro di un insieme di forme.

### Soluzione
```rust

```