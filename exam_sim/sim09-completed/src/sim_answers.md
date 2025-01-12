```markdown
# Simulazione d'Esame con Soluzioni

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
   - Il metodo `get_first` prende in prestito `self` in maniera immutabile, ma successivamente `add_item` richiede un mutabile. In Rust, non è possibile avere un riferimento mutabile a `self` mentre esiste un riferimento immutabile attivo.

2. Versione corretta:
```rust
struct Container {
    data: Vec<i32>,
}

impl Container {
    fn add_item(&mut self, item: i32) {
        self.data.push(item);
    }

    fn get_first(&self) -> Option<i32> {
        self.data.first().copied()
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
   - La funzione `Rc::try_unwrap` fallisce perché esistono ancora altri riferimenti (`a` e `b`) al dato condiviso.

2. Comportamento dell'istruzione `Rc::try_unwrap`.
   - `Rc::try_unwrap` restituisce il valore contenuto se non ci sono altri riferimenti attivi. Altrimenti, restituisce un errore.

3. Versione corretta:
```rust
use std::rc::Rc;

fn main() {
    let shared = Rc::new(vec![10, 20, 30]);

    println!("Shared vector: {:?}", shared);

    match Rc::try_unwrap(shared) {
        Ok(vec) => println!("Unwrapped vector: {:?}", vec),
        Err(_) => println!("Cannot unwrap; more references exist."),
    }
}
```

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

1. Problemi del codice:
   - Possibile deadlock se il thread principale non entra in attesa prima della notifica. Il thread principale potrebbe perdere la notifica.

2. Versione corretta:
```rust
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

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
    started = cvar.wait_while(started, |flag| !*flag).unwrap();

    handle.join().unwrap();
}
```

---

## 4. Programmazione

### Domanda
Implementare una struttura dati `ThreadSafeCounter` che consenta di incrementare e leggere un valore condiviso in modo thread-safe. La struttura deve offrire i seguenti metodi:

- `new() -> Self`: inizializza il contatore a 0.
- `increment(&self)`: incrementa il contatore di 1.
- `get(&self) -> usize`: restituisce il valore attuale del contatore.

Inoltre, creare un test che verifica che, utilizzando più thread, il contatore restituisca il valore corretto dopo un certo numero di incrementi.

### Soluzione:
```rust
use std::sync::{Arc, Mutex};
use std::thread;

struct ThreadSafeCounter {
    counter: Mutex<usize>,
}

impl ThreadSafeCounter {
    fn new() -> Self {
        Self {
            counter: Mutex::new(0),
        }
    }

    fn increment(&self) {
        let mut lock = self.counter.lock().unwrap();
        *lock += 1;
    }

    fn get(&self) -> usize {
        let lock = self.counter.lock().unwrap();
        *lock
    }
}

fn main() {
    let counter = Arc::new(ThreadSafeCounter::new());
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                counter_clone.increment();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", counter.get());
}
```
```