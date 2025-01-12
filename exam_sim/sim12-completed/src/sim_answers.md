Ecco la simulazione d'esame con soluzioni incluse, in formato Markdown, con un esercizio sui tratti al posto degli Exchanger.

---

```markdown
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
1. **Funzionamento e ruolo di `Rc<T>`:**
   - `Rc<T>` è uno smart pointer che consente la condivisione di un dato immutabile tra più possessori. Ogni volta che un nuovo `Rc` viene clonato, il contatore dei riferimenti aumenta. Quando tutti i riferimenti vengono eliminati, la memoria viene rilasciata.
   - In questo codice, `Rc` viene utilizzato per costruire una lista collegata immutabile, in cui più nodi condividono lo stesso nodo successivo.

2. **Problemi con un puntatore semplice:**
   - Un semplice puntatore (`Box` o `*mut`) non supporta il conteggio dei riferimenti. Se uno dei nodi venisse eliminato, i riferimenti ai nodi successivi diventerebbero invalidi, causando un comportamento indefinito.

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
1. **Necessità di `Arc<Mutex<T>>`:**
   - `Arc` consente la condivisione sicura del puntatore `Mutex` tra i thread.
   - `Mutex` garantisce che solo un thread alla volta possa accedere alla variabile protetta, evitando race conditions.

2. **Sostituzione del `Mutex`:**
   - Se si utilizzasse una variabile intera, i thread potrebbero accedere simultaneamente, causando race conditions e risultati imprevedibili.

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
1. **Sincronizzazione:**
   - Ogni thread incrementa il contatore e attende sulla `Condvar` fino a quando il contatore non raggiunge il numero totale. A quel punto, tutti i thread vengono svegliati e il contatore viene resettato per un nuovo ciclo.

2. **Timeout:**
   - Sostituisci `self.condvar.wait(count).unwrap()` con `self.condvar.wait_timeout(count, Duration::from_secs(1)).unwrap()`.

---

## Domanda 4: Programmazione - Tratti

### Domanda
Crea un tratto `Shape` che definisca:
1. Un metodo `area(&self) -> f64` per calcolare l'area.
2. Un metodo `perimeter(&self) -> f64` per calcolare il perimetro.

Implementa questo tratto per due strutture: `Circle` (con raggio) e `Rectangle` (con larghezza e altezza). Scrivi un programma che utilizza polimorfismo per calcolare l'area e il perimetro di un insieme di forme.

### Soluzione
```rust
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Rectangle { width: 4.0, height: 5.0 }),
    ];

    for shape in shapes {
        println!("Area: {}, Perimeter: {}", shape.area(), shape.perimeter());
    }
}
```
---
```