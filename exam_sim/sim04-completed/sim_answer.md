# APPELLO 24/12/2024

### Domanda 1
- **Q1**: "Si implementi una struttura dati `ThreadSafeMap<K, V>` che garantisca l'accesso concorrente a una mappa chiave-valore.  
La struttura deve supportare i metodi:
  - `fn new() -> Self` per creare una nuova mappa vuota.
  - `fn insert(&self, key: K, value: V)` per inserire un elemento.
  - `fn get(&self, key: &K) -> Option<V>` per recuperare un valore associato a una chiave.
  - `fn remove(&self, key: &K) -> Option<V>` per rimuovere un elemento dato una chiave.  
Garantire che la struttura sia thread-safe."

```rust
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

struct ThreadSafeMap<K, V> {
    map: RwLock<HashMap<K, V>>,
}

impl<K: std::hash::Hash + Eq, V> ThreadSafeMap<K, V> {
    fn new() -> Arc<Self> {
        Arc::new(ThreadSafeMap {
            map: RwLock::new(HashMap::new()),
        })
    }

    fn insert(&self, key: K, value: V) {
        let mut map = self.map.write().unwrap();
        map.insert(key, value);
    }

    fn get(&self, key: &K) -> Option<V> where V: Clone {
        let map = self.map.read().unwrap();
        map.get(key).cloned()
    }

    fn remove(&self, key: &K) -> Option<V> {
        let mut map = self.map.write().unwrap();
        map.remove(key)
    }
}

fn main() {
    let map = ThreadSafeMap::new();

    // Clonare la mappa per usarla in più thread
    let map1 = Arc::clone(&map);
    let map2 = Arc::clone(&map);

    // Creazione di un thread che inserisce valori
    let handle1 = std::thread::spawn(move || {
        map1.insert(1, "Thread 1");
        map1.insert(2, "Thread 2");
    });

    // Creazione di un altro thread che legge e rimuove valori
    let handle2 = std::thread::spawn(move || {
        map2.insert(3, "Thread 3");
        println!("Value for key 1: {:?}", map2.get(&1));
        println!("Removing key 2: {:?}", map2.remove(&2));
    });

    // Attendere che i thread completino
    handle1.join().unwrap();
    handle2.join().unwrap();

    // Lettura finale nella mappa principale
    println!("Final value for key 1: {:?}", map.get(&1));
    println!("Final value for key 3: {:?}", map.get(&3));
}

```
---


### Domanda 3
- **Q3**: "Si realizzi la struttura dati `DynamicLimiter` che consente di eseguire un massimo di `N` operazioni concorrenti.  
La struttura offre i metodi:
  - `fn new(limit: usize) -> Self` per inizializzare il limite massimo.
  - `fn execute<F, R>(&self, operation: F) -> Result<R, String>` dove `F` è una funzione senza parametri che ritorna `R`.  
Se si superano le operazioni concorrenti consentite, il thread deve attendere che una si liberi, senza consumare cicli CPU."

```rust
pub struct DynamicLimiter<F>{
  dim: usize,
  limiter: Mutex<Vec<F>>,
}

impl <F> DynamicLimiter<F>{
  fn new(limit: usize) -> Self{
    DynamicLimiter{
      dim: limit,
      counter: Mutex::new(),
      cv: Condvar,
    }
  }
  
  fn execute<F, R>(&self, operation: F) -> Result<R, String>{
    let mut cnt = self.counter.lock().unwrap();

    while cnt>self.dim{
      cnt = self.cv.wait(cnt).unwrap();
    }

    *cnt += 1; // si può arrivare al massimo a dim
    drop(cnt); // si rilascia il lock in modo che si possa arrivare al massimo a dim

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(operation));
    
    *cnt -=1; // la coda si svuota
    self.cv.notify_one(); // avviso gli altri che possono acquisire il lock

    match result {
          Ok(res) => Ok(res),
          Err(_) => Err("Operation failed".to_string()),
      }
  }
}
```
---

### Domanda 4
- **Q4**: "Si implementi una struttura dati `TokenBucket` che implementa un meccanismo di rate limiting utilizzando un approccio con `Token Bucket`.  
Il costruttore accetta i parametri:
  - `capacity`: capacità massima del bucket.
  - `rate`: numero massimo di token aggiunti per secondo.
  
Il metodo:
  - `fn consume(&self, n: usize) -> bool` restituisce `true` se è possibile consumare `n` token dal bucket, `false` altrimenti.  
Assicurarsi che la struttura sia thread-safe e possa essere usata in contesti concorrenti."

```rust
use std::sync::{Mutex, Arc};
use std::time::Instant;

pub struct TokenBucket {
    capacity: usize,
    tokens: Mutex<usize>,
    last_refill: Mutex<Instant>,
    rate: usize,
}

impl TokenBucket{
  fn new(capacity: usize, rate: usize){
    Arc::new(TokenBucket{
      capacity: capacity,
      last_refill: Mutex::new(Instant::now()), // L'ultima ricarica è adesso
      tokens: Mutex::new(capacity),
      rate: rate,
    })
  }

  fn consume(&self, n: usize) -> bool{
    let mut last_refill = self.last_refill.lock().unwrap();
    let mut tokens = self.tokens.lock().unwrap();
    
    let now = Instant::now();
    let elapsed = now.duration_since(*last_refill).as_secs();
    let added_token = (elapsed as usize)*self.rate;
    *last_refill = now;
    *tokens += added_token.min(self.capacity);

    if tokens >= n {
      tokens - = n;
      true
    }
    else{
      false
    }
  }

}

```