### APPELLO 27/12/2024

---

### Domanda 1 (3 pt)
#### Teoria #1

Si consideri il programma seguente:

```rust
fn main() {
    let numbers = vec![10, 15, 20, 25, 30];

    let res = numbers
        .iter()
        .filter(|&x| x % 5 == 0)
        .zip('a'..'z');

    let last = res
        .clone()
        .map(|(a, b)| format!("{b}{a}"))
        .last();

    println!("last: {:?}", last);
    println!("res: {:?}", res.count());
}
```

**Domande**:
1. Che cosa stampa questo codice?
2. Che cosa fanno le istruzioni alle righe 5, 6, 7?
3. Che cosa capita se si omette la riga 10? Perché?

---

### Domanda 2 (3 pt)
#### Teoria #2

Si descriva il comportamento del programma seguente.  
Indicare eventuali problematiche e come possono essere corrette:

```rust
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    println!("Waiting...");
    thread::sleep(Duration::from_secs(1));

    let mut started = lock.lock().unwrap();
    started = cvar.wait(started).unwrap();

    println!("End!");
}
```

---

### Domanda 3 (3 pt)
#### Teoria #3

Il seguente programma genera un errore di compilazione.  
Spiegare il motivo e indicare come correggere la struttura `S` per renderlo compilabile.

```rust
#[derive(Debug)]
struct S {
    i: i32,
}

impl From<i32> for S {
    fn from(value: i32) -> Self {
        S { i: value }
    }
}

fn main() {
    let mut v = Vec::<S>::new();
    let s = 42.into();
    for _ in 0..3 {
        v.push(s);
    }
    println!("{:?}", v);
}
```

---

### Domanda 4 (6 pt)
#### Programmazione

Si realizzi una struttura dati `Exchanger<T: Send>` per una comunicazione bidirezionale.  
Il metodo `exchange` consente lo scambio tra due istanze in due thread diversi.  
**Requisiti**:
- **Thread-safe**.
- Il metodo si blocca finché l'altra istanza non completa lo scambio.
- Deve supportare scambi multipli.
- Se una delle due istanze viene distrutta, le chiamate pendenti devono restituire `None`.

Implementare:
```rust
fn exchange(&self, t: T) -> Option<T>;
```
