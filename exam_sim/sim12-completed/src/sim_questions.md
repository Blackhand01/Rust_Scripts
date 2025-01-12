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
1. Il programma implementa una lsita concatenata foramta da 3 nodi con i rispettivi valori 10, 20, 30. 
    - Il nodo1 non punta a nulla (NPO)
    - Il nodo2 punta al nodo1
    - Il nodo3 punta al nodo2
    Il programma stampa ciò a cui punta node3, quindi stamperà la lista a partire dal nodo2:
    "Some(Node { value: 20, next: Some(Node { value: 10, next: None }) })"
2. In Rust ogni valore possiede un proprietario, nel caso un valore come node1 fosse referenziato da più puntatori (come in questo caso) ma semplici (o Box), non ci sarebbe il conteggio dei riferimenti e quindi non si saprebbe quando il valore vada deallocato (strong_counter=0) portando così a dangling pointer (puntatori che puntano a un area di memoria valida ma non più attiva).
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
1. Si creano 10 thread che appena acquisiranno il lock sulla mutex, potranno incrementare il counter.
    è necessario usare Arc per permettere a più thread di condividere lo stesso oggetto counter. Se si 
    utilizzassero altri puntatori, o non si potrebbe usare la mutex in maniera atomica (Ref, RefCell) o non ci sarebbe il conteggio dei riferimenti (puntatori semplici, Box). Si potrebbe usare in alternativa AtomicUsize.
2. Se non ci fosse un mutex, i thread non avrebbero la sicurezza di accedere al valore intero in maneira mutualmente esclusiva, ciò porterebbe a problemi di race conditions e quindi a un output non deterministico
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

    fn wait(&self, timeout: Duration) -> bool {
        let mut count = self.count.lock().unwrap();
        *count += 1;
        if *count == self.total {
            *count = 0; // Reset for the next cycle
            self.condvar.notify_all();
        } else {
            let (_, result) = self.condvar.wait_timeout(count, timeout).unwrap();
            if result.timed_out(){
                *count -= 1; // Rilascia il posto nel conteggio
                false // Timeout scaduto, la sincronizzazione non è avvenuta
            } else {
                true // La sincronizzazione è avvenuta
            }
            
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
```rust
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;
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

    fn wait(&self, timeout: Duration) -> Result<(), &'static str> {
        let mut count = self.count.lock().unwrap();
        *count += 1;

        if *count == self.total {
            // Tutti i thread hanno raggiunto la barriera
            *count = 0; // Reset per il prossimo ciclo
            self.condvar.notify_all();
            Ok(())
        } else {
            // Attendi con timeout
            let (mut count, result) = self.condvar.wait_timeout(count, timeout).unwrap();
            
            if result.timed_out() {
                return Err("Timeout expired");
            }
            
            if *count == 0 {
                Ok(())
            } else {
                Err("Unexpected state")
            }

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
            let timeout = Duration::from_secs(1);

            match barrier.wait(timeout) {
                Ok(_) => println!("Thread {} proceeding", i),
                Err(err) => println!("Thread {} timed out: {}", i, err),
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```
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
pub trait Shape{
    fn shape(&self) -> usize;
}

pub struct Circle{
    radius: usize,
}

pub struct Rectangle{
    width: usize,
    height: usize,
}

impl Shape for Circle{
    fn shape()
}

impl Shape for Rectangle{

}
```