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


```rust
pub struct Exchanger<T>{
    data: Mutex<Option<T>>,
    cv: Condvar,
    state: Mutex<bool>,
}

impl<T: Send> Exchanger<T>{
    fn new() -> Self{
        Exchanger{
            data: Mutex::new(None),
            cv: Condvar::new(),
            state: Mutex::new(true),
        }
    }

    fn exchange(&self, t: T) -> Option<T>{

        let mut data = self.data.lock().unwrap();
        let state = self.state.lock().unwrap();
         if !*state{
            return None;
        }

        if *data.is_none(){ 
            // non c'è nessun messaggio,
            // metto il mio msg
            // restituisco il msg dell'altro
            *data = Some(t);
            while data.is_none(){
                *data = self.cv.wait(data).unwrap();
            }
            data.take()
        }
        else{
            
            let Some(val) = data.take();
            cv.notify_one();
            Some(val)
        }

    }
}

impl Drop for Exchanger{
    fn drop(&self){
        let mut state = self.state.lock().unwrap();
        *state = false;
        self.cv.notify_all();
    }
}

```

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

- stack
    - vector: 
        - puntatore a Vec -> 64bit = 8 Byte
        - capacity -> 1 usize = 8 Byte
        - length -> 1 usize = 8 Byte
    - vslice: 
        - puntatore a Vec[1..3] -> 64bit = 8 Byte
        - length -> usize = 8 Byte

- heap
    - vector: 8*8 = 64 Byte
1. 24 Byte (puntatore, capacity[8], length [5])
2. 64 Byte (capacity * u64)
3. 16 Byte

---

### Esercizio 2

Si consideri il programma seguente che riporta la numerazione delle linee di codice.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 8];

    let res = numbers
        .iter()
        .filter(|&x| x % 2 == 0) // 2, 4, 8
        .zip('a'..'z'); //2a, 4b, 8c

    let last = res
        .clone()
        .map(|(a, b)| format!("{b}{a}")) //a2, ...
        .last(); // c8

    println!("last: {:?}", last);
    println!("res: {:?}", res.count());
}
```

Rispondere alle seguenti domande:
1. Che cosa stampa questo codice?
2. Che cosa fanno le istruzioni alle righe 5, 6, 7?
3. Che cosa capita se si omette la riga 10? Perché?

1. c8 3
2. numbers entra in uno stream di procedure, la prima rende iterable il vettore, quindi posso accede ai singoli valori, poi filtra i valori pari, poi li zippa con le lettere dell'alfabeto
3. res, dopo non sarà più disponibile per la stampa, perchè sarà stato consumato da last
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

Il codice presenta un programam che fa uso di 2 thread.
Il thread principale crea una tupla, "pair" a cui si accede tramite smart pointer Arc, il quale permetter operazioni atomiche attraverso il suo contatore dei riferimenti. La tupla è formata da una mutex che protegge un valore booleano e da una conditional variable. Viene poi creata pair2 che è un clone di pair e verrà passato al thread figlio per poter accedere alla mutex e alla condvar. Dopodichè il thread principale dorme per 1 secondo acquisisce il lock sulla mutex e rimane in attesa con wait (attende in maniera ottimizzata spreca poca CPU).
Nel mentre il thread secondario richiede anche lui il lock, metti il valore booleano a true e notifica all'altro thread che ha finito attraverso la conditiona variable.
Il problema può verificarsi se il thread secondario finisce la sua serie di azioni (acquisisce il lock, mette il boolean a true, notifica che ha finito) mentre il thread principale dorme e non è ancora in wait.
Può essere corretto così:

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
    while !*started{
        started = cvar.wait(started).unwrap();
    }

    println!("End!");
}
```
---

