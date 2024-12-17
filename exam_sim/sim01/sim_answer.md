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

---
#### Risposta
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

