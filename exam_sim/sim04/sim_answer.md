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
Il programma funziona correttamente e stampa 1, 2, 3, 4, 4, 4
---
### Domanda 1
- **Q1**: "Implementa una struttura thread-safe chiamata `ReadWriteCounter` che permetta:
  - Letture concorrenti del valore del contatore.
  - Incrementi esclusivi del contatore.
  - I seguenti metodi:
    ```rust
    fn increment(&self);
    fn get(&self) -> i32;
    ```

```rust
use std::sync::{RwLock, Condvar}

pub struct ReadWriteCounter{
    counter: RwLock<usize>,
    cv: Condvar,
}

impl ReadWriteCounter{
    fn new() -> Self {
        ReadWriteCounter {
            value: RwLock::new(0),
        }
    }

    fn increment(&self){
        let mut cnt = self.counter.write().unwrap();
        cnt += 1;
    }

    fn get(&self) -> i32{
        self.counter.read().unwrap()
    }
}
```
---

### Domanda 3
- **Q3**: "Realizza una struttura `TaskQueue<T>` che permetta di accodare task che verranno eseguiti da un pool di thread. La struttura deve supportare i seguenti metodi:
    ```rust
    fn new(pool_size: usize) -> Self;
    fn enqueue(&self, task: impl FnOnce() + Send + 'static);
    ```

```rust
use std::sync::{Mutex, Condvar, Arc};
use std::thread;

pub struct TaskQueue{
    tasks: Mutex<Vec<Box<dyn FnOnce() + Send>>>,
    cv: Condvar,
}

impl TaskQueue{
    pub fn new(pool_size: usize) -> Arc<Self>{
        let queue = Arc::new(TaskQueue{
            tasks: Mutex::new(Vec::new()),
            cv: Condvar::new(),
        });
        

        for _ in 0..pool_size{
            let queue = Arc::clone(&queue);
            thread::spawn(move || {
                loop{
                    let t = {
                        let mut tasks = queue.tasks.lock().unwrap();
                        while tasks.is_empty(){
                            tasks = queue.cv.wait(tasks).unwrap(); // quando faccio wait, lo devo assegnare
                        }
                        tasks.pop()
                    };
                    if let Some(task) = t{
                        task();
                    }
                }
            });
        }
        queue
    }

    pub fn enqueue(&self, task: impl FnOnce() + Send + 'static){
        let mut tasks = self.tasks.lock().unwrap();
        tasks.push(Box::new(task)); // task è dyn FnOnce() + Send => serve un Box
        self.cv.notify_one();
    }
}

fn main() {
    let queue = TaskQueue::new(3); // Crea un pool con 3 thread

    for i in 0..10 {
        let task = move || {
            println!("Executing task {}", i);
        };
        queue.enqueue(task);
    }

    // Aspetta un po' per consentire l'esecuzione dei task
    thread::sleep(std::time::Duration::from_secs(1));
}

```
---

### Domanda 4
- **Q4**: "Progetta e implementa una versione semplice di `Barrier<T>` che permette ai thread di sincronizzarsi e ottenere un valore aggregato calcolato attraverso una funzione di riduzione fornita dall'utente. Il metodo `wait_and_reduce` deve avere la seguente firma:
    ```rust
    fn wait_and_reduce(&self, value: T, reduce_fn: impl Fn(T, T) -> T + Send + Sync) -> T;

    ```
Soluzione:

    ```rust
    fn wait_and_reduce(&self, value: T, reduce_fn: impl Fn(T, T) -> T + Send + Sync) -> T;

    ```
---