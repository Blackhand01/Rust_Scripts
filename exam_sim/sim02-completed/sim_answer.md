### Domanda 2
- **Q2**: "Si analizzi il seguente codice Rust e si spieghi perché genera un panic:
```rust
fn main() {
    let v = vec![1, 2, 3];
    let mut iter = v.iter();
    println!("{:?}", iter.next().unwrap());
    println!("{:?}", iter.next().unwrap());
    println!("{:?}", iter.next().unwrap());
    println!("{:?}", iter.next().unwrap());
}
```
Come può essere corretto per evitare il panic?"
next() restituisce un Option<&T> (Some() o None), unwrap() serve per prendere il valore interno, 
- nel caso positivo: &T
- nel caso negativo: apre None e questo genera panic perchè None non ha wrapper

Essendoci 3 valori nel vettore ma 4 chiamate a next(), l'ultima prenderà None e chiamando unwrap(), genererà panic
Si può risolvere così:
```rust
while let Some(val) = iter.next().unwrap(){
    println!("{:?}", val);
}
```
---
### Domanda 4
- **Q4**: "Si definisca una versione personalizzata di `RankingBarrier` che, oltre a sincronizzare `N` thread, restituisca una stringa con il seguente formato:  
`"Thread <ID> arrived at position <rank>"` dove `<ID>` è un identificatore del thread e `<rank>` è il suo ordine di arrivo.  
L'implementazione deve essere thread-safe."

```rust
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

pub struct RankingBarrier {
    n: usize,
    counter: Mutex<usize>,
    cv: Condvar,
}


impl RankingBarrier {
    pub fn new(n: usize) -> Self {
        RankingBarrier {
            n,
            counter: Mutex::new(0),
            cv: Condvar::new(),
        }
    }

    pub fn wait(&self, id: usize) -> String {
        let mut cnt = self.counter.lock().unwrap();
        *cnt += 1;
        let my_rank = *cnt;

        if *cnt == self.n {
            // Reset the state for future use
            *cnt = 0;
            // Notifica tutti i thread in attesa
            self.cv.notify_all();
        } else {
            // Attende finché la condizione non viene notificata
            self.cv.wait(cnt).unwrap();
        }

        format!("Thread {} arrived at position {}", id, my_rank)
    }
}

fn main() {
    // Creazione di una barriera per sincronizzare 5 thread
    let barrier = Arc::new(RankingBarrier::new(5));
    let mut handles = vec![];

    for i in 0..5 {
        let barrier_clone = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            let result = barrier_clone.wait(i);
            println!("{}", result);
        }));
    }

    // Attendi che tutti i thread completino
    for handle in handles {
        handle.join().unwrap();
    }
}

```
---
### Domanda 1
- **Q1**: "Si implementi la struttura `CircularBuffer<T>` che consente di gestire una coda circolare di dimensione fissa.  
La struttura offre:
    - `fn new(size: usize) -> Self` per inizializzare la coda.
    - `fn push(&self, item: T)` per inserire un elemento nella coda.
    - `fn pop(&self) -> Option<T>` per rimuovere e restituire un elemento dalla coda.
    - La coda deve essere thread-safe."

```rust

use std::sync::{Mutex, Condvar};

pub struct CircularBuffer<T> { // FIFO
    buffer: Mutex<Vec<T>>,
    size: usize,
    cv: Condvar,
}

impl<T> CircularBuffer<T> {
    fn new(size: usize) -> Self {
        CircularBuffer { 
            buffer: Mutex::new(Vec::with_capacity(size)),
            size,
            cv: Condvar::new(),
        }
    }

    fn push(&self, item: T) {
        let mut buffer = self.buffer.lock().unwrap();
        if buffer.len() == self.size {
            buffer.remove(0); // Rimuovo il più vecchio
        }
        buffer.push(item); // Inserisco il nuovo elemento

        self.cv.notify_one(); // Notifica thread in attesa
    }

    fn pop(&self) -> Option<T> {
        let mut buffer = self.buffer.lock().unwrap(); // Deve essere mutabile
        if buffer.is_empty() {
            None
        } else {
            Some(buffer.remove(0)) // Restituisco il più vecchio
        }
    }

    fn size(&self) -> usize {
        let buffer = self.buffer.lock().unwrap();
        buffer.len()
    }
}

fn main() {
    let buffer = CircularBuffer::new(3);
    buffer.push(1);
    buffer.push(2);
    buffer.push(3);
    buffer.push(4);

    println!("Popped: {:?}", buffer.pop().unwrap()); // Popped: Some(2)
    println!("Size: {}", buffer.size());    // Size: 2
}


```
---
### Domanda 3
- **Q3**: "Si realizzi una `ThreadSafeCache<K, V>` che garantisce l'accesso concorrente ai dati.  
Il metodo `fn get_or_compute(&self, key: K, compute: impl FnOnce() -> V) -> V` deve:
   - Calcolare il valore per una chiave solo se non già presente.
   - Garantire che il calcolo avvenga una sola volta per chiave."

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

// Definizione della struttura ThreadSafeCache
struct ThreadSafeCache<K: Eq + std::hash::Hash, V> {
    map: Mutex<HashMap<K, V>>, // Protezione thread-safe
}

impl<K: Eq + std::hash::Hash, V: Clone> ThreadSafeCache<K, V> {
    // Costruttore: Inizializza un HashMap protetto da Mutex
    fn new() -> Self {
        ThreadSafeCache {
            map: Mutex::new(HashMap::new()), // HashMap inizializzato
        }
    }

    // Metodo get_or_compute
    fn get_or_compute(&self, key: K, compute: impl FnOnce() -> V) -> V {
        let mut map = self.map.lock().unwrap(); // Blocca il Mutex per proteggere il HashMap

        if let Some(value) = map.get(&key) {
            return value.clone(); 
        }

        let value = compute(); // Calcola il valore se non presente
        map.insert(key, value.clone()); // Inserisce il valore calcolato nella cache
        value // Restituisce il valore
    }
}

// Esempio di utilizzo
fn main() {
    // Avvolgi la cache in un Arc per permettere la condivisione sicura tra thread
    let cache = Arc::new(ThreadSafeCache::new());

    // Una funzione costosa per calcolare un valore
    let compute = |key: i32| {
        println!("Calcolo per la chiave: {}", key);
        key * 2
    };

    // Thread per testare il comportamento della cache
    let handles: Vec<_> = (0..5)
        .map(|i| {
            // Clona l'Arc per trasferirlo nel thread
            let cache_ref = Arc::clone(&cache);
            let compute_clone = compute.clone(); // Clona la closure se necessario

            thread::spawn(move || {
                // Ogni thread calcola o recupera un valore
                let value = cache_ref.get_or_compute(i, || compute_clone(i));
                println!("Thread {} ha ottenuto valore: {}", i, value);
            })
        })
        .collect();

    // Attendi il completamento di tutti i thread
    for handle in handles {
        handle.join().unwrap();
    }

    // Testa il recupero di un valore già calcolato
    let existing_value = cache.get_or_compute(2, || compute(2));
    println!("Recuperato valore esistente: {}", existing_value);
}


```
