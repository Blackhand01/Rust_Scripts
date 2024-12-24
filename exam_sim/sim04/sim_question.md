### APPELLO 20/12/2024

#### Versione **Domande**

### Domanda 1
- **Q1**: "Implementa una struttura thread-safe chiamata `ReadWriteCounter` che permetta:
  - Letture concorrenti del valore del contatore.
  - Incrementi esclusivi del contatore.
  - I seguenti metodi:
    ```rust
    fn increment(&self);
    fn get(&self) -> i32;
    ```"

---

### Domanda 2
- **Q2**: "Spiega il comportamento del seguente programma. Se presenta errori, correggili:
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let mut handles = vec![];

    for _ in 0..3 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let mut d = data.lock().unwrap();
            d.push(4);
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
- **Q3**: "Realizza una struttura `TaskQueue<T>` che permetta di accodare task che verranno eseguiti da un pool di thread. La struttura deve supportare i seguenti metodi:
    ```rust
    fn new(pool_size: usize) -> Self;
    fn enqueue(&self, task: impl FnOnce() + Send + 'static);
    ```"

---

### Domanda 4
- **Q4**: "Progetta e implementa una versione semplice di `Barrier<T>` che permette ai thread di sincronizzarsi e ottenere un valore aggregato calcolato attraverso una funzione di riduzione fornita dall'utente. Il metodo `wait_and_reduce` deve avere la seguente firma:
    ```rust
    fn wait_and_reduce(&self, value: T, reduce_fn: impl Fn(T, T) -> T + Send + Sync) -> T;
    ```"
