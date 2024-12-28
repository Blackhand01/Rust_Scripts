### APPELLO 27/12/2024

---

### Domanda 1 (3 pt)
#### Teoria #1

Si consideri il programma seguente:

```rust
fn main() {
    let numbers = vec![10, 15, 20, 25, 30];

    let res = numbers // 10a, 15b, 20c, 25d, 30e
        .iter()
        .filter(|&x| x % 5 == 0)
        .zip('a'..'z');

    let last = res
        .clone()
        .map(|(a, b)| format!("{b}{a}")) //10a -> a10, ..., 30e -> e30
        .last(); // 30e

    println!("last: {:?}", last); // 30e
    println!("res: {:?}", res.count()); //5
}
```

**Domande**:
1. Che cosa stampa questo codice?
2. Che cosa fanno le istruzioni alle righe 5, 6, 7?
3. Che cosa capita se si omette la riga 10? Perché?
Risposte:
1. last: 30e res: 5
2. rende il vettore iterabile negli elementi, li filtra divisibili per 5, li zippa con le lettere dell'alfabeto
3. res viene consumato e res.count() non restituisce il valore desiderato
```rust

```
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
il thread principale si addormenta quando riceve la notifica, questo porta a race conditions poichè il thread principale non riceverà più alcuna notifica

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
s va clonato, altrimenti, viene consumato e si perdono i suoi riferimenti alla prima iterazione nel ciclo dopo la push.
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
use std::sync::{Arc, Condvar, Mutex};

pub struct Exchanger<T: Send>{
    channel: Mutex<Option<T>>,
    cv: Condvar,
    active: Mutex<bool>
}

impl<T: Send> Exchanger<T>{
 fn new() -> Arc<Self> {
        Arc::new(Self {
            channel: Mutex::new(None),
            cv: Condvar::new(),
            active: Mutex::new(true),
        })
    }

    fn exchange(&self, t: T) -> Option<T>{
        let mut channel = self.channel.lock().unwrap();
        if let Some(value) = channel.take(){
            self.cv.notify_one();
            return Some(value); // restituisco il valore
        }
        else{
            *channel = Some(value); // inserisco il valore
            while *self.active.lock().unwrap() && channel.is_some(){
               channel = self.cv.wait(channel).unwrap();
            }
            channel.take()
        }
    }
}

impl <T> Drop for Exchanger<T>{
    fn drop(&mut self){
        *self.active.lock().unwrap() = false;
        self.cv.notify_all();
    }
}

```

<!-- 
Scenario senza il controllo di "attivo"
Immagina due thread che usano l'oggetto Exchanger per scambiarsi messaggi:
Il thread A chiama exchange e si blocca, in attesa di un valore dal thread B.
Prima che il thread B chiami exchange, l'oggetto Exchanger viene distrutto.
Il thread A rimane bloccato per sempre, perché nessuno lo sveglia per informarlo che l'oggetto non esiste più.

Perché serve sapere se l'oggetto è attivo?
Se aggiungiamo un campo active al nostro Exchanger, possiamo fare così:
Il thread A chiama exchange e si blocca, in attesa di un valore dal thread B.
Prima che il thread B chiami exchange, l'oggetto Exchanger viene distrutto.
Il Drop dell'oggetto imposta active = false e sveglia tutti i thread bloccati.
Il thread A, appena svegliato, controlla active:
Se active è false, capisce che l'oggetto è stato distrutto e ritorna None. -->
