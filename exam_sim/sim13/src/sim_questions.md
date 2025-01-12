Ecco una nuova simulazione d'esame basata sui tuoi argomenti richiesti (concorrenza, Cell e RefCell, tratti e chiusure), strutturata in Markdown con soluzioni:

---

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

---

### Domanda 2: Chiusure

#### Q2
Si consideri il seguente codice:
```rust

```

1. Quale tipo di cattura viene utilizzato nella chiusura?  
2. Che cosa accade se proviamo a utilizzare `x` direttamente dopo aver passato la chiusura a un'altra funzione?  
3. Modifica il codice per utilizzare `move` e spiega come cambia il comportamento.

---

### Soluzione

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

```
