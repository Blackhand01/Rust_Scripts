# Simulazione d'esame Rust

**Data: DD/MM/YYYY**

---

### Domanda 1: Cell e RefCell

#### Q1
Analizza il seguente codice e rispondi alle domande:
```rust
use std::cell::{Cell, RefCell};

fn main() {
    let c = Cell::new(42);
    let rc = RefCell::new(vec![1, 2, 3]);

    // Modifica tramite Cell
    c.set(100);
    println!("Cell value: {}", c.get());

    // Modifica tramite RefCell
    {
        let mut v = rc.borrow_mut();
        v.push(4);
    }
    println!("RefCell value: {:?}", rc.borrow());
}
```

1. Spiega la differenza tra `Cell` e `RefCell`.
2. Perché `Cell` non espone `borrow` o `borrow_mut`?
3. Che cosa succede se, nel blocco che modifica `RefCell`, eseguiamo un'altra `borrow_mut` contemporanea? Prova a implementare la modifica e spiega il risultato.

---

### Soluzione

1. Cell e RefCell sono 2 smartpointer:
 - Cell permette di modificare un valore attraverso metodi propri del puntatore come ad esempio "get() e set()", evitando prestiti.
 - RefCell permette di avere in prestito quel valore e quindi manipolarlo, con "borrow" si avrà un riferimento condiviso, con "borrow_mut" si avrà un riferimento mutabile.
2. Cell non espone un riferimento perchè non implementa la mutabilità interna.
3. Succede che, potendo esserci solo un riferimento mutabile alla volta, il borrow checker darà l'errore relativo.
```rust
use std::cell::{Cell, RefCell};

fn main() {
    let c = Cell::new(42);
    let rc = RefCell::new(vec![1, 2, 3]);

    // Modifica tramite Cell
    c.set(100);
    println!("Cell value: {}", c.get());

    // Modifica tramite RefCell
    {
        let mut v = rc.borrow_mut();
        let mut t = rc.borrow_mut();
        v.push(4);
    }
    println!("RefCell value: {:?}", rc.borrow());
}
```

---

### Domanda 2: Chiusure

#### Q2
Si consideri il seguente codice:
```rust
fn main() {
    let mut x = 10;
    let mut closure = |delta: i32| {
        x += delta;
        println!("x is now: {}", x);
    };

    closure(5);
    closure(10);
}
```

1. Quale tipo di cattura viene utilizzato nella chiusura?  
2. Che cosa accade se proviamo a utilizzare `x` direttamente dopo aver passato la chiusura a un'altra funzione?  
3. Modifica il codice per utilizzare `move` e spiega come cambia il comportamento.

---

### Soluzione
1. La chiusura effettua una cattura per riferimento mutabile 
2. nulla, dopo closure(5), la stampa è "x is now 15". Dopo closure(10), la stampa è "x is now 25".
3. se usiammo move, x viene deallocata dopo la prima chiusura (closure(5)) in quanto move trasferisce la proprietà di x alla closure e quindi il dovere di dealloacarla alla fine del suo scope.

---

### Domanda 3: Tratti

#### Q3
Si implementi un tratto `Logger` che permette di registrare messaggi in un file. La struttura deve supportare:
- Un metodo per registrare un messaggio (`log`).
- Un metodo per restituire tutti i messaggi (`get_logs`).

Si fornisca un'implementazione del tratto per una struttura `FileLogger` che salva i messaggi in memoria.

---

### Soluzione
```rust

trait Logger{
    fn log(&mut self, msg: &str);
    fn get_logs(&self) -> Vec<String>;
}

struct FileLogger{
    logs: Vec<String>,
}

impl Logger for FileLogger{
    fn log(&mut self, stringa: &str){
        self.logs.push(stringa.to_string());
    }

    fn get_logs(&self) -> Vec<String>{
        self.logs.clone()
    }
}

fn main() {
    let mut logger = FileLogger { logs: vec![] };
    logger.log("First message");
    logger.log("Second message");
    println!("{:?}", logger.get_logs());
}
```

---

### Domanda 4: Concorrenza

#### Q4
Implementa un tratto `Latch` per una struttura thread-safe che consente ai thread di aspettare che una condizione venga soddisfatta. I metodi da implementare sono:
```rust
trait Latch {
    fn new() -> Self;
    fn wait(&self);
    fn trigger(&self);
}
```

La struttura deve usare un `Condvar` per gestire i thread in attesa.

---

### Soluzione
```rust
struct ThreadSafeLatch{
    counter: Mutex<bool>,
    cv: Condvar,
}

impl Latch for ThreadSafeLatch{
    fn new() -> Self{
        ThreadSafeLatch{
            counter: Mutex::new(false),
            cv: Condvar::new(),
        }
    }

    fn wait(&self){
        let mut cnt = self.counter.lock().unwrap();
        // while !*cnt{
        //     cnt = self.cv.wait(cnt).unwrap();
        // }
        cnt = self.cv.wait_while(cnt, cnt, |cnt| !*cnt).unwrap();
    }

    fn trigger(&self){
        let mut cnt = self.counter.lock().unwrap();
        *cnt=true;
        self.cv.notify_one();
    }
}
```
