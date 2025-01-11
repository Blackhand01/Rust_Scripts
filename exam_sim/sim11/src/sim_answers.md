
### APPELLO DD/MM/YYYY

#### Domanda 1
- **Q1**: "Si implementi un sistema di comunicazione tra thread utilizzando una struttura `BidirectionalChannel<T: Send>`. Ogni thread può inviare e ricevere messaggi bidirezionali. Il canale deve garantire che:
  - Ogni messaggio venga consegnato esattamente una volta.
  - Non ci siano deadlock o perdita di messaggi."
- **Risposta**:
  La soluzione utilizza `Arc<Mutex<>>` per garantire la thread-safety e canali `std::sync::mpsc`.
```rust
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};

struct BidirectionalChannel<T: Send> {
    sender_a: Sender<T>,
    receiver_a: Receiver<T>,
    sender_b: Sender<T>,
    receiver_b: Receiver<T>,
}

impl<T: Send + Clone> BidirectionalChannel<T> {
    fn new() -> Self {
        let (tx_a, rx_a) = channel();
        let (tx_b, rx_b) = channel();
        BidirectionalChannel {
            sender_a: tx_a,
            receiver_a: rx_a,
            sender_b: tx_b,
            receiver_b: rx_b,
        }
    }

    fn send_from_a(&self, msg: T) {
        self.sender_a.send(msg).unwrap();
    }

    fn send_from_b(&self, msg: T) {
        self.sender_b.send(msg).unwrap();
    }

    fn recv_at_a(&self) -> Option<T> {
        self.receiver_b.recv().ok()
    }

    fn recv_at_b(&self) -> Option<T> {
        self.receiver_a.recv().ok()
    }
}
```

---

#### Domanda 2
- **Q2**: "Si implementi una struttura dati `ConcurrentLazyCache<K, V>` che utilizza `Arc<RwLock<HashMap<K, V>>>` per fornire accesso concorrente a una cache lazy. Spiegare come prevenire le race condition e assicurare che ogni chiave venga valutata una sola volta."
- **Risposta**:
```rust
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::hash::Hash;

struct ConcurrentLazyCache<K: Eq + Hash, V> {
    cache: Arc<RwLock<HashMap<K, V>>>,
}

impl<K: Eq + Hash + Clone, V: Clone> ConcurrentLazyCache<K, V> {
    fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn get_or_insert_with<F>(&self, key: K, f: F) -> V
    where
        F: FnOnce() -> V,
    {
        {
            let read_lock = self.cache.read().unwrap();
            if let Some(value) = read_lock.get(&key) {
                return value.clone();
            }
        }
        let mut write_lock = self.cache.write().unwrap();
        write_lock.entry(key.clone()).or_insert_with(f).clone()
    }
}
```

---

#### Domanda 3
- **Q3**: "Realizzare una variante della `RankingBarrier` che supporti una chiamata `abort()`. Tale chiamata interrompe tutti i thread in attesa e impedisce ulteriori utilizzi della barriera. Garantire la thread-safety."
- **Risposta**:
```rust
use std::sync::{Arc, Condvar, Mutex};

struct AbortableBarrier {
    n_threads: usize,
    counter: Mutex<usize>,
    cv: Condvar,
    aborted: Mutex<bool>,
}

impl AbortableBarrier {
    fn new(n_threads: usize) -> Arc<Self> {
        Arc::new(Self {
            n_threads,
            counter: Mutex::new(0),
            cv: Condvar::new(),
            aborted: Mutex::new(false),
        })
    }

    fn wait(&self) -> Result<(), &'static str> {
        let mut counter = self.counter.lock().unwrap();
        if *self.aborted.lock().unwrap() {
            return Err("Barrier aborted");
        }
        *counter += 1;
        if *counter == self.n_threads {
            *counter = 0;
            self.cv.notify_all();
            Ok(())
        } else {
            self.cv.wait(counter).unwrap();
            if *self.aborted.lock().unwrap() {
                Err("Barrier aborted")
            } else {
                Ok(())
            }
        }
    }

    fn abort(&self) {
        *self.aborted.lock().unwrap() = true;
        self.cv.notify_all();
    }
}
```

---

#### Domanda 4
- **Q4**: "Si costruisca una pipeline concorrente composta da più fasi in cui ogni fase è rappresentata da un `Looper`. Ogni fase deve:
  - Ricevere un messaggio dalla fase precedente.
  - Eseguire una trasformazione sul messaggio.
  - Inviare il risultato alla fase successiva.
  Implementare e dimostrare la pipeline con un caso pratico."
- **Risposta**:
```rust
use std::sync::{mpsc, Arc};
use std::thread;

fn looper<F, T>(process: F) -> mpsc::Sender<T>
where
    F: Fn(T) -> T + Send + 'static,
    T: Send + 'static,
{
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for msg in rx {
            let result = process(msg);
            println!("Processed: {:?}", result);
        }
    });
    tx
}

fn main() {
    let stage1 = looper(|x: i32| x + 1);
    let stage2 = looper(|x: i32| x * 2);

    for i in 0..5 {
        stage1.send(i).unwrap();
    }

    for i in 0..5 {
        stage2.send(i).unwrap();
    }
}
```

---

Posso fornire ulteriori dettagli o migliorare l'innovatività della simulazione. Vuoi procedere con modifiche?