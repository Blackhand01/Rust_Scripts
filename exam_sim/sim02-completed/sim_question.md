# APPELLO 17/12/2024

### Domanda 1
- **Q1**: "Si implementi la struttura `CircularBuffer<T>` che consente di gestire una coda circolare di dimensione fissa.  
La struttura offre:
    - `fn new(size: usize) -> Self` per inizializzare la coda.
    - `fn push(&self, item: T)` per inserire un elemento nella coda.
    - `fn pop(&self) -> Option<T>` per rimuovere e restituire un elemento dalla coda.
    - La coda deve essere thread-safe."

---

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

---

### Domanda 3
- **Q3**: "Si realizzi una `ThreadSafeCache<K, V>` che garantisce l'accesso concorrente ai dati.  
Il metodo `fn get_or_compute(&self, key: K, compute: impl FnOnce() -> V) -> V` deve:
   - Calcolare il valore per una chiave solo se non già presente.
   - Garantire che il calcolo avvenga una sola volta per chiave."

---

### Domanda 4
- **Q4**: "Si definisca una versione personalizzata di `RankingBarrier` che, oltre a sincronizzare `N` thread, restituisca una stringa con il seguente formato:  
`"Thread <ID> arrived at position <rank>"` dove `<ID>` è un identificatore del thread e `<rank>` è il suo ordine di arrivo.  
L'implementazione deve essere thread-safe."

