Ecco una simulazione d'esame in stile Rust, basata sui concetti esposti nei documenti forniti. La struttura segue il formato degli esempi indicati.

---

# Simulazione d'Esame Rust

## 1. Borrow Checker

### Domanda
Spiegare il comportamento del codice seguente e indicare perché genera un errore di compilazione. Successivamente, fornire una versione corretta del codice che possa essere compilata ed eseguita.

```rust
struct Container {
    data: Vec<i32>,
}

impl Container {
    fn add_item(&mut self, item: i32) {
        self.data.push(item);
    }

    fn get_first(&self) -> Option<&i32> {
        self.data.first()
    }
}

fn main() {
    let mut container = Container { data: vec![] };
    container.add_item(42);

    let first = container.get_first();
    container.add_item(50);

    println!("First item: {:?}", first);
}
```

1. Perché il codice genera un errore di borrow checker?
2. Proporre una versione corretta del codice.

---

## 2. Smart Pointers

### Domanda
Si consideri il seguente codice che utilizza un `Rc`:

```rust
use std::rc::Rc;

fn main() {
    let shared = Rc::new(vec![10, 20, 30]);

    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);

    println!("Shared vector: {:?}", shared);

    let dropped = Rc::try_unwrap(shared);
    println!("Unwrapped vector: {:?}", dropped);
}
```

1. Spiegare cosa accade durante l'esecuzione di questo codice.
2. Indicare il comportamento dell'istruzione `Rc::try_unwrap`.
3. Modificare il codice affinché l'oggetto condiviso possa essere deallocato correttamente e il vettore venga ottenuto senza errori.

---

## 3. Concorrenza e gestione dei thread

### Domanda
Descrivere il comportamento del programma seguente e identificare eventuali problematiche. Successivamente, proporre le modifiche necessarie affinché il programma funzioni senza errori.

```rust
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    let handle = thread::spawn(move || {
        let (lock, cvar) = &*pair_clone;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    started = cvar.wait(started).unwrap();

    handle.join().unwrap();
}
```

1. Quali problemi possono sorgere nel programma sopra riportato?
2. Modificare il codice per risolvere eventuali problemi di sincronizzazione.

---

## 4. Programmazione

### Domanda
Implementare una struttura dati `ThreadSafeCounter` che consenta di incrementare e leggere un valore condiviso in modo thread-safe. La struttura deve offrire i seguenti metodi:

- `new() -> Self`: inizializza il contatore a 0.
- `increment(&self)`: incrementa il contatore di 1.
- `get(&self) -> usize`: restituisce il valore attuale del contatore.

Inoltre, creare un test che verifica che, utilizzando più thread, il contatore restituisca il valore corretto dopo un certo numero di incrementi.

```rust
// Struttura da completare
use std::sync::{Arc, Mutex};
use std::thread;

struct ThreadSafeCounter {
    // Implementazione
}

impl ThreadSafeCounter {
    fn new() -> Self {
        // Implementazione
    }

    fn increment(&self) {
        // Implementazione
    }

    fn get(&self) -> usize {
        // Implementazione
    }
}

fn main() {
    // Test multithreading
}
```

---

Se hai bisogno di ulteriori chiarimenti o di aiuto con le soluzioni, fammi sapere!