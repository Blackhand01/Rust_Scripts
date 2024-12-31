**APPELLO DD/MM/YYYY**

#### Domanda 1
Si realizzi una struttura dati `Latch` che consente a più thread di sincronizzarsi su un punto comune. La struttura deve offrire un metodo `count_down()` per ridurre il contatore interno e un metodo `await()` per attendere che il contatore arrivi a zero. Il `Latch` deve essere thread-safe e implementato usando i meccanismi di sincronizzazione di Rust.

---

#### Domanda 2
Si implementi un canale multi-producer, multi-consumer (MPMC). La struttura deve supportare metodi per inviare e ricevere dati:
- `send(&self, data: T) -> Result<(), SendError<T>>`
- `recv(&self) -> Option<T>`

Il canale deve essere thread-safe e permettere la comunicazione tra thread multipli.

---

#### Domanda 3
Il seguente codice presenta un errore di borrow checker. Si identifichi il problema e si proponga una soluzione.
```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);
}
```

---

#### Domanda 4
Si realizzi la struttura dati `ExecutionLimiter` che permette di eseguire al massimo `N` operazioni simultanee. Il metodo principale deve essere:
```rust
fn execute<R>(&self, f: impl FnOnce() -> R + UnwindSafe) -> Result<R, ExecutionError>
```
Il metodo deve gestire eventuali panico della funzione `f` riducendo correttamente il conteggio delle esecuzioni attive.

---

### Versione con soluzioni

**APPELLO DD/MM/YYYY**

#### Domanda 1 - Soluzione
```rust
use std::sync::{Arc, Mutex, Condvar};

struct Latch {
    counter: Mutex<usize>,
    cv: Condvar,
}

impl Latch {
    fn new(count: usize) -> Arc<Self> {
        Arc::new(Self {
            counter: Mutex::new(count),
            cv: Condvar::new(),
        })
    }

    fn count_down(&self) {
        let mut count = self.counter.lock().unwrap();
        if *count > 0 {
            *count -= 1;
        }
        if *count == 0 {
            self.cv.notify_all();
        }
    }

    fn await(&self) {
        let mut count = self.counter.lock().unwrap();
        while *count > 0 {
            count = self.cv.wait(count).unwrap();
        }
    }
}
```

---

#### Domanda 2 - Soluzione
```rust
use std::sync::{Arc, Mutex, Condvar};
use std::collections::VecDeque;

struct MPMC<T> {
    queue: Mutex<VecDeque<T>>,
    cv: Condvar,
}

impl<T> MPMC<T> {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            queue: Mutex::new(VecDeque::new()),
            cv: Condvar::new(),
        })
    }

    fn send(&self, data: T) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(data);
        self.cv.notify_one();
    }

    fn recv(&self) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        while queue.is_empty() {
            queue = self.cv.wait(queue).unwrap();
        }
        queue.pop_front()
    }
}
```

---

#### Domanda 3 - Soluzione
Il problema è che Rust non consente mutabili e immutabili prestiti contemporaneamente. Correzione:
```rust
fn main() {
    let mut s = String::from("hello");
    {
        let r1 = &s;
        println!("{}", r1);
    }
    let r2 = &mut s;
    println!("{}", r2);
}
```

---

#### Domanda 4 - Soluzione
```rust
use std::sync::{Arc, Condvar, Mutex};
use std::panic::{UnwindSafe, catch_unwind};

struct ExecutionLimiter {
    limit: usize,
    active: Mutex<usize>,
    cv: Condvar,
}

impl ExecutionLimiter {
    fn new(limit: usize) -> Arc<Self> {
        Arc::new(Self {
            limit,
            active: Mutex::new(0),
            cv: Condvar::new(),
        })
    }

    fn execute<R>(&self, f: impl FnOnce() -> R + UnwindSafe) -> Result<R, ExecutionError> {
        let mut active = self.active.lock().unwrap();
        while *active >= self.limit {
            active = self.cv.wait(active).unwrap();
        }
        *active += 1;
        drop(active);

        let result = catch_unwind(f);

        let mut active = self.active.lock().unwrap();
        *active -= 1;
        self.cv.notify_one();

        result.map_err(|_| ExecutionError {})
    }
}

struct ExecutionError;
```