### Domanda 1
- **Q1**: "Spiega cosa stampa il seguente codice e perché:
```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let shared = Arc::new(RwLock::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let shared = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            let mut data = shared.write().unwrap();
            *data += 1;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final value: {}", *shared.read().unwrap());
}
```  
Quali meccanismi sono utilizzati per garantire la sicurezza dei thread?"


La RwLock è una primitiva di sincronizzatione che permette più letture in contemporanea e una sola scrittura esclusiva, garantendo la sicurezza dei thread.
Arc consente la condivisione di shared tra i thread senza duplciare il dato e violare la sicurezza

---

### Domanda 2
- **Q2**: "Implementa una struttura thread-safe chiamata `ThreadedCounter` che consente di incrementare un contatore in modo concorrente da parte di più thread. La struttura deve supportare i seguenti metodi:
```rust
fn increment(&self);
fn get(&self) -> i32;
```
Fornisci un esempio di utilizzo della struttura in cui più thread incrementano il contatore e il valore finale viene stampato."

```rust
use::std::sync::{Arc, Mutex};

pub struct ThreadedCounter{
    counter: Arc<Mutex<i32>>,
}

impl ThreadedCounter{
    fn new() -> Self{
        ThreadedCounter{
        counter: Arc::new(Mutex::new(0)),
            
        }
    }

    fn increment(&self){
        let mut cnt = self.counter.lock().unwrap();
        *cnt += 1;
    }

    fn get(&self) -> i32{
        *self.counter.lock().unwrap()
    }
}
```

---
### Domanda 3
- **Q3**: "Il seguente codice genera un deadlock. Spiega il motivo e fornisci una versione corretta:
```rust
use std::sync::{Mutex, Condvar};
use std::thread;

fn main() {
    let pair = (Mutex::new(false), Condvar::new());

    let (lock, cvar) = &pair;

    thread::spawn(move || {
        let (lock, cvar) = &pair;
        let mut guard = lock.lock().unwrap();
        *guard = true;
        cvar.notify_one();
    });
    let guard = lock.lock().unwrap();

    cvar.wait(guard).unwrap();

    println!("Finished!");
}
```
Un deadlock si verifica in un sistema concorrente quando 2 o più thread rimangono bloccati indefinitamente, ciascuno in attessa che l'altro rilasci una risorsa necessaria a proseguire. In questo caso sia il thread principale che il thread figlio richiedono la mutex
su lock, ma il thread figlio non lo avrà mai poichè il thread padre non rilascia la mutex.
Per risolverlo:
```rust
use std::sync::{Mutex, Condvar};
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));

    // viene clonato per poterlo usare nel thread, se spostiamo pair con move, 
    // poi non possiamo più chiamarlo in questo thread principale
    let pair_cloned = Arc::clone(&pair)

    thread::spawn(move || {
        // &pair_clone: &Arc<(Mutex, Condvar)>, &*pair_clone: (Mutex, Condvar) punta alla Mutex
        let (lock, cvar) = &*pair_cloned;
        let mut guard = lock.lock().unwrap();
        *guard = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let guard = lock.lock().unwrap();
    while !*guard{
        cvar.wait(guard).unwrap();
    }

    println!("Finished!");
}
```
---

### Domanda 4
- **Q4**: "Realizza la struttura `CircularBuffer<T>` che supporti i seguenti metodi:
```rust
fn new(size: usize) -> Self;
fn push(&self, item: T);
fn pop(&self) -> Option<T>;
fn size(&self) -> usize;
```
Garantisci che la struttura sia thread-safe. Descrivi il comportamento in caso di buffer pieno e vuoto."

```rust
use std::sync::{Mutex, Condvar};

pub struct CircularBuffer<T> { // FIFO
    buffer: Mutex<Vec<T>>,
    size: usize,
    cv: Condvar,
}

impl<T> CircularBuffer<T> {
    fn new(size: usize) -> Self {
        CircularBuffer { 
            buffer: Mutex::new(Vec::with_capacity(size)),
            size,
            cv: Condvar::new(),
        }
    }

    fn push(&self, item: T) {
        let mut buffer = self.buffer.lock().unwrap();
        if buffer.len() == self.size {
            buffer.remove(0); // Rimuovo il più vecchio
        }
        buffer.push(item); // Inserisco il nuovo elemento

        self.cv.notify_one(); // Notifica thread in attesa
    }

    fn pop(&self) -> Option<T> {
        let mut buffer = self.buffer.lock().unwrap(); // Deve essere mutabile
        if buffer.is_empty() {
            None
        } 
        else {
            Some(buffer.remove(0)) // Restituisco il più vecchio
        }
    }

    fn size(&self) -> usize {
        let buffer = self.buffer.lock().unwrap();
        buffer.len()
    }
}

fn main() {
    let buffer = CircularBuffer::new(3);
    buffer.push(1);
    buffer.push(2);
    buffer.push(3);
    buffer.push(4);

    println!("Popped: {:?}", buffer.pop().unwrap()); // Popped: Some(2)
    println!("Size: {}", buffer.size());    // Size: 2
}

```