## 1. Borrow Checker

### Domanda
Spiegare il comportamento del codice seguente e indicare perché genera un errore di compilazione. Successivamente, fornire una versione corretta del codice che possa essere compilata ed eseguita.

```rust
struct Container {
    data: Vec<i32>,
}

impl Container {
    fn add_item(&mut self, item: i32) {
        self.data.push(item);
    }

    fn get_first(&self) -> Option<i32> {
        self.data.first() //re
    }
}

fn main() {
    let mut container = Container { data: vec![] };
    container.add_item(42);

    let first = container.get_first();
    container.add_item(50);

    println!("First item: {:?}", first);
}
```

1. Perché il codice genera un errore di borrow checker?
2. Proporre una versione corretta del codice.

Risposte:
1. Perchè first è un riferimento condiviso immutabile a container.  
Tra l'acquisizione di first tramite metodo e la sua stampa, viene creato un riferimento mutabile a container, questo il borrow checker non lo permette.
2. versione corretta:
```rust
struct Container {
    data: Vec<i32>,
}

impl Container {
    fn add_item(&mut self, item: i32) {
        self.data.push(item);
    }

    fn get_first(&self) -> Option<&i32> {
        self.data.first() 
    }
}

fn main() {
    let mut container = Container { data: vec![] };
    container.add_item(42);

    let first = container.get_first();
    println!("First item: {:?}", first);
    container.add_item(50);

}
```
oppure
```rust
struct Container {
    data: Vec<i32>,
}

impl Container {
    fn add_item(&mut self, item: i32) {
        self.data.push(item);
    }

    fn get_first(&self) -> Option<i32> {
        self.data.first().copied() // restituisce una copia, non un riferimento (no clone perchè restituisce un riferimento)
    }
}

fn main() {
    let mut container = Container { data: vec![] };
    container.add_item(42);

    let first = container.get_first();
    println!("First item: {:?}", first);
    container.add_item(50);

}
```
---

## 2. Smart Pointers

### Domanda
Si consideri il seguente codice che utilizza un `Rc`:

```rust
use std::rc::Rc;

fn main() {
    let shared = Rc::new(vec![10, 20, 30]);

    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);

    println!("Shared vector: {:?}", shared);

    let dropped = Rc::try_unwrap(shared);
    println!("Unwrapped vector: {:?}", dropped);
}
```

1. Spiegare cosa accade durante l'esecuzione di questo codice.
2. Indicare il comportamento dell'istruzione `Rc::try_unwrap`.
3. Modificare il codice affinché l'oggetto condiviso possa essere deallocato correttamente e il vettore venga ottenuto senza errori.

---
