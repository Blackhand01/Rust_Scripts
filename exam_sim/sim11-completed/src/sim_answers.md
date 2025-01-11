### APPELLO DD/MM/YYYY

---

#### **Domanda 1**
- **Q1**: "Si implementi un sistema di comunicazione tra thread utilizzando una struttura `BidirectionalChannel<T: Send>`. Ogni thread può inviare e ricevere messaggi bidirezionali. Il canale deve garantire che:
  - Ogni messaggio venga consegnato esattamente una volta.
  - Non ci siano deadlock o perdita di messaggi."
  
- **Risposta**:
```rust
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};

pub struct BidirectionalChannel<T> {
    sender: Mutex<Sender<T>>,
    receiver: Mutex<Receiver<T>>,
}

impl<T: Send + 'static> BidirectionalChannel<T> {
    pub fn new() -> (Arc<Self>, Arc<Self>) {
        let (tx1, rx1) = channel();
        let (tx2, rx2) = channel();

        let channel1 = Arc::new(BidirectionalChannel {
            sender: Mutex::new(tx1),
            receiver: Mutex::new(rx2),
        });

        let channel2 = Arc::new(BidirectionalChannel {
            sender: Mutex::new(tx2),
            receiver: Mutex::new(rx1),
        });

        (channel1, channel2)
    }

    pub fn send(&self, msg: T) {
        let sender = self.sender.lock().unwrap();
        sender.send(msg).unwrap();
    }

    pub fn receive(&self) -> T {
        let receiver = self.receiver.lock().unwrap();
        receiver.recv().unwrap()
    }
}
```

- **Main di test**:
```rust
use std::thread;

fn main() {
    let (channel1, channel2) = BidirectionalChannel::new();

    let t1 = thread::spawn({
        let channel = channel1.clone();
        move || {
            channel.send("Hello from thread 1");
            let response = channel.receive();
            println!("Thread 1 received: {}", response);
        }
    });

    let t2 = thread::spawn({
        let channel = channel2.clone();
        move || {
            let message = channel.receive();
            println!("Thread 2 received: {}", message);
            channel.send("Hello from thread 2");
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
```

---

#### **Domanda 2**
- **Q2**: "Si implementi una struttura dati `ConcurrentLazyCache<K, V>` che utilizza `Arc<RwLock<HashMap<K, V>>>` per fornire accesso concorrente a una cache lazy. Spiegare come prevenire le race condition e assicurare che ogni chiave venga valutata una sola volta."

- **Risposta**:
```rust
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct ConcurrentLazyCache<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Clone,
    V: Clone,
{
    data: Arc<RwLock<HashMap<K, V>>>,
}

impl<K, V> ConcurrentLazyCache<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        ConcurrentLazyCache {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get_or_insert_with<F>(&self, key: K, value_fn: F) -> V
    where
        F: FnOnce() -> V,
    {
        {
            let read_guard = self.data.read().unwrap();
            if let Some(value) = read_guard.get(&key) {
                return value.clone();
            }
        }

        let mut write_guard = self.data.write().unwrap();
        write_guard.entry(key.clone()).or_insert_with(value_fn).clone()
    }
}
```

- **Main di test**:
```rust
fn main() {
    let cache = ConcurrentLazyCache::new();

    let v1 = cache.get_or_insert_with(1, || 42);
    let v2 = cache.get_or_insert_with(1, || 100);

    println!("v1: {}, v2: {}", v1, v2);
    assert_eq!(v1, v2); // Assicura che la chiave 1 venga calcolata una sola volta
}
```

---

#### **Domanda 3**
- **Q3**: "Realizzare una variante della `RankingBarrier` che supporti una chiamata `abort()`. Tale chiamata interrompe tutti i thread in attesa e impedisce ulteriori utilizzi della barriera. Garantire la thread-safety."

- **Risposta**:
```rust
use std::sync::{Condvar, Mutex};

pub struct RankingBarrier {
    parties: usize,
    count: Mutex<usize>,
    cycle: Mutex<usize>,
    cv: Condvar,
    aborted: Mutex<bool>,
}

impl RankingBarrier {
    pub fn new(parties: usize) -> Self {
        RankingBarrier {
            parties,
            count: Mutex::new(0),
            cycle: Mutex::new(0),
            cv: Condvar::new(),
            aborted: Mutex::new(false),
        }
    }

    pub fn wait(&self) -> Result<usize, &'static str> {
        let mut count = self.count.lock().unwrap();
        let mut cycle = self.cycle.lock().unwrap();

        if *self.aborted.lock().unwrap() {
            return Err("Barrier aborted");
        }

        *count += 1;
        if *count == self.parties {
            *count = 0;
            *cycle += 1;
            self.cv.notify_all();
            Ok(*cycle)
        } else {
            let current_cycle = *cycle;
            while *cycle == current_cycle {
                count = self.cv.wait(count).unwrap();
                if *self.aborted.lock().unwrap() {
                    return Err("Barrier aborted");
                }
            }
            Ok(*cycle)
        }
    }

    pub fn abort(&self) {
        let mut aborted = self.aborted.lock().unwrap();
        *aborted = true;
        self.cv.notify_all();
    }
}
```

- **Main di test**:
```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let barrier = Arc::new(RankingBarrier::new(3));

    let handles: Vec<_> = (0..3)
        .map(|_| {
            let barrier = barrier.clone();
            thread::spawn(move || match barrier.wait() {
                Ok(rank) => println!("Thread synchronized in cycle {}", rank),
                Err(e) => println!("Thread aborted: {}", e),
            })
        })
        .collect();

    barrier.abort();

    for handle in handles {
        handle.join().unwrap();
    }
}
```

---

#### **Domanda 4**
- **Q4**: "Si costruisca una pipeline concorrente composta da più fasi in cui ogni fase è rappresentata da un `Looper`. Ogni fase deve:
  - Ricevere un messaggio dalla fase precedente.
  - Eseguire una trasformazione sul messaggio.
  - Inviare il risultato alla fase successiva.
  Implementare e dimostrare la pipeline con un caso pratico."

- **Risposta**:
```rust
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

pub fn looper<F, T>(rx: Receiver<T>, tx: Sender<T>, transform: F)
where
    F: Fn(T) -> T + Send + 'static,
    T: Send + 'static,
{
    thread::spawn(move || {
        for item in rx {
            let result = transform(item);
            tx.send(result).unwrap();
        }
    });
}
```

- **Main di test**:
```rust
fn main() {
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();
    let (tx3, rx3) = channel();

    looper(rx1, tx2, |x| x * 2);
    looper(rx2, tx3, |x| x + 1);

    tx1.send(5).unwrap();
    tx1.send(10).unwrap();
    drop(tx1); // Fine della fase 1

    for result in rx3 {
        println!("Result: {}", result); // Output atteso: 11, 21
    }
}
```