## Simulazione d'Esame - Versione Domande

### APPELLO 16/12/2024

#### Domanda 1
- **Q1**: Si implementi una struttura `PriorityQueue<T: Ord>` che consente di inserire elementi in modo tale che il metodo `pop` restituisca sempre l'elemento con la priorità più alta, mantenendo il thread-safety.

#### Domanda 2
- **Q2**: Si analizzi il seguente codice e si spieghi il motivo del panic in fase di runtime:
```rust
use std::sync::Arc;
fn main() {
    let mut data = Arc::new(vec![1, 2, 3]);
    let data_clone = Arc::clone(&data); 
    let first = Arc::get_mut(&mut data).unwrap();
    first.push(4);
    println!("{:?}", data);
}
```
Che cosa sarebbe necessario per risolvere il problema?

#### Domanda 3
- **Q3**: Si definisca una versione del pattern producer-consumer utilizzando un `Channel` di Rust e si mostri come garantire che ogni messaggio venga consumato una sola volta.

#### Domanda 4
- **Q4**: La struttura `BatchProcessor` accetta fino a `N` task, li esegue in parallelo, e si blocca fino al completamento di tutti i task correnti prima di accettarne di nuovi. Si implementi il metodo `process(tasks: Vec<impl FnOnce() -> T>) -> Vec<T>`.

---
