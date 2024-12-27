# APPELLO 24/12/2024

### Domanda 1
- **Q1**: "Si implementi una struttura dati `ThreadSafeMap<K, V>` che garantisca l'accesso concorrente a una mappa chiave-valore.  
La struttura deve supportare i metodi:
  - `fn new() -> Self` per creare una nuova mappa vuota.
  - `fn insert(&self, key: K, value: V)` per inserire un elemento.
  - `fn get(&self, key: &K) -> Option<V>` per recuperare un valore associato a una chiave.
  - `fn remove(&self, key: &K) -> Option<V>` per rimuovere un elemento dato una chiave.  
Garantire che la struttura sia thread-safe."

---

### Domanda 2
- **Q2**: "Si descriva il comportamento del seguente programma.  
Indicare come correggere eventuali errori di runtime e ottimizzare la gestione delle risorse:
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let mut handles = vec![];

    for _ in 0..3 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let mut lock = data.lock().unwrap();
            lock.push(4);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", *data.lock().unwrap());
}
```

---

### Domanda 3
- **Q3**: "Si realizzi la struttura dati `DynamicLimiter` che consente di eseguire un massimo di `N` operazioni concorrenti.  
La struttura offre i metodi:
  - `fn new(limit: usize) -> Self` per inizializzare il limite massimo.
  - `fn execute<F, R>(&self, operation: F) -> Result<R, String>` dove `F` è una funzione senza parametri che ritorna `R`.  
Se si superano le operazioni concorrenti consentite, il thread deve attendere che una si liberi, senza consumare cicli CPU."

---

### Domanda 4
- **Q4**: "Si implementi una struttura dati `TokenBucket` che implementa un meccanismo di rate limiting utilizzando un approccio con `Token Bucket`.  
Il costruttore accetta i parametri:
  - `capacity`: capacità massima del bucket.
  - `rate`: numero massimo di token aggiunti per secondo.
  
Il metodo:
  - `fn consume(&self, n: usize) -> bool` restituisce `true` se è possibile consumare `n` token dal bucket, `false` altrimenti.  
Assicurarsi che la struttura sia thread-safe e possa essere usata in contesti concorrenti."