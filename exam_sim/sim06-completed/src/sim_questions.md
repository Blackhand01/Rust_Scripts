# APPELLO DD/MM/YYYY

## 6. Programmazione

### Esercizio 1

Si realizzi l’implementazione della struttura dati `Exchanger<T: Send>` (e dei metodi e delle funzioni necessarie) utile per realizzare una comunicazione bidirezionale.

Ciascun lato della comunicazione dispone di un’istanza della struttura `Exchanger<T: Send>`. La comunicazione avviene invocando il metodo:

```rust
fn exchange(&self, t: T) -> Option<T>
```

che, una volta invocato, si blocca fino a quando non viene invocato il metodo corrispettivo sulla struttura corrispondente al lato opposto della comunicazione. Dopodiché, restituisce il valore che è stato passato come argomento al metodo corrispondente al lato opposto (che farà altrettanto), sotto forma di `Some(t)`.

- Lo scambio può essere ripetuto un numero arbitrario di volte.
- Se una delle due strutture formanti la coppia viene distrutta, un'eventuale chiamata bloccata sul metodo della struttura restante terminerà restituendo il valore `None`.

Si implementi tale struttura in linguaggio Rust avendo cura che la sua implementazione sia thread-safe.

---

## 4. Strutture Dati

### Esercizio 1

Si considerino le seguenti strutture dati e rispettive porzioni di codice. Per ciascuna di esse si indichi la dimensione di memoria allocata nello stack e nello heap, ipotizzando un’architettura a 64 bit.

```rust
let mut vector = Vec::<u64>::with_capacity(8);
for i in 0..5 {
    vector.push(i);
}
let vslice = &vector[1..3];
```

Indicare:
1. La dimensione allocata per `vector` nello stack.
2. La dimensione allocata per i dati di `vector` nello heap.
3. La dimensione di memoria riferita da `vslice`.

---

### Esercizio 2

Si consideri il programma seguente che riporta la numerazione delle linee di codice.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 8];

    let res = numbers
        .iter()
        .filter(|&x| x % 2 == 0)
        .zip('a'..'z');

    let last = res
        .clone()
        .map(|(a, b)| format!("{b}{a}"))
        .last();

    println!("last: {:?}", last);
    println!("res: {:?}", res.count());
}
```

Rispondere alle seguenti domande:
1. Che cosa stampa questo codice?
2. Che cosa fanno le istruzioni alle righe 5, 6, 7?
3. Che cosa capita se si omette la riga 10? Perché?

---

### Esercizio 3

Si descriva il comportamento del seguente programma. Se presenta delle problematiche, indicare come può essere modificato:

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

    println!("Waiting ...");
    thread::sleep(Duration::from_secs(1));

    let mut started = lock.lock().unwrap();
    started = cvar.wait(started).unwrap();

    println!("End!");
}
```

---

