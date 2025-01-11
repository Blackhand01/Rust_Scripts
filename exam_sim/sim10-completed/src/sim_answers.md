```markdown
# APPELLO DD/MM/YYYY

## Domanda 1

### Q1
Si implementi una struttura `CircularBuffer<T>` che rappresenti un buffer circolare thread-safe di capacità fissa. Deve offrire i seguenti metodi:
  - `new(capacity: usize) -> Self`: costruisce un nuovo buffer con capacità massima `capacity`.
  - `push(&self, item: T) -> Result<(), T>`: aggiunge un elemento al buffer, ritornando `Err(item)` se il buffer è pieno.
  - `pop(&self) -> Option<T>`: rimuove e ritorna l'elemento più vecchio nel buffer o `None` se il buffer è vuoto.

### Soluzione
```rust
use std::sync::{Arc, Mutex};

struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    capacity: usize,
    head: usize,
    tail: usize,
    size: usize,
    lock: Mutex<()>,
}

impl<T> CircularBuffer<T> {
    fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![None; capacity],
            capacity,
            head: 0,
            tail: 0,
            size: 0,
            lock: Mutex::new(()),
        }
    }

    fn push(&self, item: T) -> Result<(), T> {
        let _guard = self.lock.lock().unwrap();
        if self.size == self.capacity {
            return Err(item);
        }
        self.buffer[self.head] = Some(item);
        self.head = (self.head + 1) % self.capacity;
        self.size += 1;
        Ok(())
    }

    fn pop(&self) -> Option<T> {
        let _guard = self.lock.lock().unwrap();
        if self.size == 0 {
            return None;
        }
        let item = self.buffer[self.tail].take();
        self.tail = (self.tail + 1) % self.capacity;
        self.size -= 1;
        item
    }
}
```

---

## Domanda 2

### Q2
Implementare un semaforo thread-safe, utilizzando Mutex e Condvar. Deve supportare i metodi:
  - `new(initial_count: usize) -> Self`
  - `acquire(&self)`
  - `release(&self)`

### Soluzione
```rust
use std::sync::{Arc, Condvar, Mutex};

struct Semaphore {
    count: Mutex<usize>,
    cv: Condvar,
}

impl Semaphore {
    fn new(initial_count: usize) -> Self {
        Self {
            count: Mutex::new(initial_count),
            cv: Condvar::new(),
        }
    }

    fn acquire(&self) {
        let mut count = self.count.lock().unwrap();
        while *count == 0 {
            count = self.cv.wait(count).unwrap();
        }
        *count -= 1;
    }

    fn release(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
        self.cv.notify_one();
    }
}
```

---

## Domanda 3

### Q3
Si implementi una `RankingBarrier` ciclica che permette ai thread di sincronizzarsi su più cicli. Utilizzare Mutex e Condvar.

### Soluzione
```rust
use std::sync::{Arc, Condvar, Mutex};

struct RankingBarrier {
    n_threads: usize,
    count: Mutex<usize>,
    cv: Condvar,
    phase: Mutex<usize>,
}

impl RankingBarrier {
    fn new(n_threads: usize) -> Arc<Self> {
        Arc::new(Self {
            n_threads,
            count: Mutex::new(0),
            cv: Condvar::new(),
            phase: Mutex::new(0),
        })
    }

    fn wait(&self) -> usize {
        let mut count = self.count.lock().unwrap();
        let mut phase = self.phase.lock().unwrap();

        *count += 1;
        if *count == self.n_threads {
            *count = 0;
            *phase += 1;
            self.cv.notify_all();
        } else {
            let current_phase = *phase;
            self.cv.wait_while(count, |c| *c < self.n_threads && *phase == current_phase)
                .unwrap();
        }

        *phase
    }
}
```

---

## Domanda 4

### Q4
Si implementi una pipeline concorrente con i seguenti requisiti:
  - Pattern: `Producer-Consumer`, `Fan-out/Fan-in`.
  - Una fase `Producer` genera numeri casuali.
  - Una fase `Worker` elabora i numeri (moltiplicandoli per un fattore casuale).
  - Una fase `Consumer` raccoglie i risultati e stampa i numeri elaborati.
  - Deve supportare più worker concorrenti e un unico consumer.

### Soluzione
```rust
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use rand::Rng;

fn producer(tx: Sender<i32>, count: usize) {
    thread::spawn(move || {
        for _ in 0..count {
            let num = rand::thread_rng().gen_range(1..100);
            tx.send(num).unwrap();
        }
    });
}

fn worker(rx: Arc<Mutex<Sender<i32>>>, tx: Sender<i32>) {
    thread::spawn(move || {
        while let Ok(num) = rx.lock().unwrap().recv() {
            let result = num * rand::thread_rng().gen_range(1..10);
            tx.send(result).unwrap();
        }
    });
}

fn consumer(rx: Arc<Mutex<Sender<i32>>>) {
    thread::spawn(move || {
        while let Ok(result) = rx.lock().unwrap().recv() {
            println!("Processed: {}", result);
        }
    });
}

fn main() {
    let (producer_tx, producer_rx) = channel();
    let producer_rx = Arc::new(Mutex::new(producer_rx));

    let (worker_tx, worker_rx) = channel();
    let worker_rx = Arc::new(Mutex::new(worker_rx));

    let consumer_tx = worker_rx.clone();

    producer(producer_tx, 100);
    for _ in 0..5 {
        worker(worker_rx.clone(), worker_tx.clone());
    }
    consumer(consumer_tx);
}
```
```