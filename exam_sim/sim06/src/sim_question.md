**APPELLO 31/12/2024**

#### Domanda 1
Si realizzi una struttura dati `Latch` che consente a più thread di sincronizzarsi su un punto comune. La struttura deve offrire un metodo `count_down()` per ridurre il contatore interno e un metodo `await()` per attendere che il contatore arrivi a zero. Il `Latch` deve essere thread-safe e implementato usando i meccanismi di sincronizzazione di Rust.

```rust
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

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

    fn wait_for_zero(&self) {
        let mut count = self.counter.lock().unwrap();
        while *count > 0 {
            count = self.cv.wait(count).unwrap();
        }
    }
}

fn main() {
    let latch = Latch::new(5);

    let mut handles = vec![];

    for i in 0..5 {
        let latch_clone = latch.clone();
        handles.push(thread::spawn(move || {
            println!("Thread {}: lavoro in corso...", i);
            thread::sleep(Duration::from_secs(1));
            println!("Thread {}: lavoro completato, count_down()", i);
            latch_clone.count_down();
        }));
    }

    println!("Thread principale: in attesa che i thread completino il lavoro...");
    latch.wait_for_zero();
    println!("Thread principale: tutti i thread hanno completato!");

    for handle in handles {
        handle.join().unwrap();
    }
}

```
---

#### Domanda 2
Si implementi un canale multi-producer, multi-consumer (MPMC). La struttura deve supportare metodi per inviare e ricevere dati:
- `send(&self, data: T) -> Result<(), SendError<T>>`
- `recv(&self) -> Option<T>`

Il canale deve essere thread-safe e permettere la comunicazione tra thread multipli.
```rust
pub struct MPMC<T>{
    queue: Mutex<Vec<T>>,
    cv: Condvar,
}

impl<T> MPMC<T>{
    fn send(&self, data: T) -> Result<(), SendError<T>>{
        let mut queue = self.queue.lock().unwrap();
        queue.push(data);
        self.cv.notify_all();
    }

    fn recv(&self) -> Option<T>{
        let mut queue = self.queue.lock().unwrap();
        while !queue.is_some(){
            queue = self.cv.wait(queue).unwrap();
        }
        queue.remove(0);
    }
}
```

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
L'errore del borrow checker è relativo al fatto che viene creo un riferimento immulabile a s e poi uno immutabile, questo non è permesso in Rust.
Il borrow checker permette letture multiple e scritture singole, ci può essere un singolo riferimento mutabile o tanti riferimenti immutabili per evitare problemi di sicurezza della memoria: dangling pointer, race conditions, o altri problemi legati alla coerenza dei dati in memoria.
Si potrebbe risolvere così:
```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s.clone();
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

```rust
pub struct ExecutionLimiter{
    limit: usize,
    counter: Mutex<usize>,
    cv: Condvar,
}

impl ExecutionLimiter{
    fn execute<R>(&self, f: impl FnOnce() -> R + UnwindSafe) -> Result<R, ExecutionError>{
        let mut cnt = self.counter.lock().unwrap();
        while *cnt > self.limit{
            cnt = self.wait(cnt).unwrap();
        }
        *cnt += 1;
        drop(cnt);

        let result = catch_unwind(f);
        
        let mut cnt = self.counter.lock().unwrap();
        *cnt -= 1;
        self.cv.notify_one();
    }
        
}
```