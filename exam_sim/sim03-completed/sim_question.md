### APPELLO 18/12/2024

#### Versione con sole domande:

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

---

### Domanda 2
- **Q2**: "Implementa una struttura thread-safe chiamata `ThreadedCounter` che consente di incrementare un contatore in modo concorrente da parte di più thread. La struttura deve supportare i seguenti metodi:
```rust
fn increment(&self);
fn get(&self) -> i32;
```
Fornisci un esempio di utilizzo della struttura in cui più thread incrementano il contatore e il valore finale viene stampato."

---

### Domanda 3
- **Q3**: "Il seguente codice genera un deadlock. Spiega il motivo e fornisci una versione corretta:
```rust
use std::sync::{Mutex, Condvar};
use std::thread;

fn main() {
    let pair = (Mutex::new(false), Condvar::new());

    let (lock, cvar) = &pair;
    let guard = lock.lock().unwrap();

    thread::spawn(move || {
        let (lock, cvar) = &pair;
        let mut guard = lock.lock().unwrap();
        *guard = true;
        cvar.notify_one();
    });

    cvar.wait(guard).unwrap();

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
