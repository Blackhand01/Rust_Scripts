### Domanda 2
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

Arc crea un puntatore che consente la condivisione sicura di un valore nell'heap anche in contesti di multithreading.
Ogni volta che si clona un arc, il valore del suo reference counter si incrementa, quando viene droppato invece il counter si decrementa. Solo quando il contatore raggiunge 0, viene deallocato dall'heap.

"Arc::get_mut(...).unwrap()" ottiene un valore MUTABILE al dato nell'heap, che è condiviso (sola lettura) e questo può portare a errori di coerenza in contesti con più thread (race conditions), quindi get_mut() restituirà "None" e sarà "unwrap()" che restituirà "panic"


Per risolvere:
```rust
use std::sync::{Arc, Mutex};

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let data_clone = Arc::clone(&data);
    {
        let mut first = data.lock().unwrap();
        first.push(4);
    }
    println!("{:?}", data.lock().unwrap());
}

```
---

### Domanda 3
- **Q3**: Si definisca una versione del pattern producer-consumer utilizzando un `Channel` di Rust e si mostri come garantire che ogni messaggio venga consumato una sola volta.

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main(){

    let (tx, rx) = mpsc::channel();

    let producer = thread::spawn(move || {
        let messages = vec![
            String::from("Message 1"),
            String::from("Message 2"),
            String::from("Message 3"),
        ];

        for message in messages{
            println!("Producer: sto inviando -> {}", message);
            tx.send(message).unwrap(); // send cattura un Result da rx che potrebbe essere Er se rx è stato chiuso prima di ricevere l'invio
            thread::sleep(Duration::from_millis(500));
        }
    });

    let consumer = thread::spawn(move || {
        for message in rx{
            println!("Consumer: ho ricevuto -> {}", message);
            thread::sleep(Duration::from_millis(500));
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
```
alternativa:
```rust
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

// ricevono e inviano solo valori i32

fn producer(tx: Sender<i32>){
    for i in 0..10{
        tx.send(i).unwrap();
    }
}

fn consumer(rx: Receiver<i32>){
    for received in rx{
        println!{"Consumed: {}", received};
    }
}

fn main(){
    let (tx, rx) = mpsc::channel();
    let producer_thread = thread::spawn(move || producer(tx));
    let consumer_thread = thread::spawn(move || consumer(rx));

    producer_thread.join().unwrap();
    consumer_thread.join().unwrap();
}
```
---
#### Domanda 4
- **Q4**: La struttura `BatchProcessor` accetta fino a `N` task, li esegue in parallelo, e si blocca fino al completamento di tutti i task correnti prima di accettarne di nuovi. Si implementi il metodo `process(tasks: Vec<impl FnOnce() -> T>) -> Vec<T>`.

```rust
use std::thread;
use std::sync::mpsc;
use std::time::Duration;


pub struct BatchProcessor{
    limit: usize,
}
impl BatchProcessor{
    fn new(n: usize){
        BatchProcessor{ limit }
    }

    fn process(tasks: Vec<impl FnOnce() -> T>) -> Vec<T>{
        // 1. Creare un vettore condiviso per salvare i risultati in modo thread-safe
        let results = Arc::new(Mutex::new(Vec::new()));
        let mut thread_handles = Vec::new();

        // 2. Dividere i task in batch di dimensione limit
        // - Clonare il riferimento ai risultati condivisi
        // - Creare un nuovo thread per ogni batch
        // - Eseguire i task nel batch e salvare i risultati
        for chunk in tasks.chunks(self.limit) {

            let results_copy = Arc::clone(&results); // ogni thread ha il suo r
            let batch = chunk.to_vec();

            thread_handles.push(
                thread::spawn(move ||){
                    let thread_result = Vec::new();
                    // task in batch (sottoparte di tasks) è una funzione (impl FnOnce()), che viene eseguita dichiarando task()
                    for task in batch{
                        thread_result.push(task());
                    }
                    results_copy.lock().unwrap().extend(thread_result);
                    // se ogni thread ha il suo results_copy, che senso ha fare la lock?
                    // anche se ogni thread ha una copia di result tramite Arc::clone, 
                    // il Vec contenuto dentro Mutex è condiviso tra tutti i thread (anche quello principale), 
                    // quindi serve il lock per proteggere l'accesso concorrente.
                }
            );
        }

        // 3. Aspettare la terminazione di tutti i thread
        for handle in thread_handles{
            handle.join().unwrap();
        }
        // 4. Restituire i risultati
        Arc::try_unwrap(results).unwrap().into_inner().unwrap()
    }
}

```
---

#### Domanda 1
- **Q1**: Si implementi una struttura `PriorityQueue<T: Ord>` che consente di inserire elementi in modo tale che il metodo `pop` restituisca sempre l'elemento con la priorità più alta, mantenendo il thread-safety.


```rust
// BinaryHeap è una struttura dati della libreria standard di Rust che implementa una coda con priorità
use std::sync::Mutex;

struct PriorityQueue<T: Ord> {
    heap: Mutex<Vec<T>>,
}

impl <T: Ord> PriorityQueue<T>{
    pub fn new() -> Self{
        PriorityQueue(Vec::new())
    }

    pub fn push(&self, value: T){
        let mut heap = self.heap.lock().unwrap();
        heap.push(value);
        heap.sort_by(|a,b| b.cmp(a));
    }

    pub fn pop(&self) -> Option<T>{
        let mut heap = self.heap.lock().unwrap();
        heap.pop()
    }
}

```
