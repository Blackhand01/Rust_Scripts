Ecco una nuova simulazione d'esame Rust, con argomenti variati basati sulle tue richieste.

---

# Simulazione d'esame Rust

**Data: DD/MM/YYYY**

---

### Domanda 1: Borrow Checker e Alias Mutabili

#### Q1
Analizza il seguente codice e descrivi il comportamento del compilatore. Proponi una versione corretta che compili ed esegua senza errori.

```rust
fn main() {
    let mut x = 10;
    let r1 = &mut x;
    let r2 = &mut x;
    *r1 += 5;
    *r2 += 10;
    println!("x: {}", x);
}
```

1. Perché il codice non compila?  
2. Correggi il codice mantenendo lo stesso comportamento.

---

### Soluzione

1. **Errore**: Non è consentito avere due alias mutabili attivi contemporaneamente su una variabile. Il compilatore segnala che `r1` e `r2` sono due prestiti mutabili simultanei.  
2. **Versione corretta**:
   ```rust
   fn main() {
       let mut x = 10;
       {
           let r1 = &mut x;
           *r1 += 5;
       }
       {
           let r2 = &mut x;
           *r2 += 10;
       }
       println!("x: {}", x); // x: 25
   }
   ```

---

### Domanda 2: Cell e RefCell

#### Q2
Esamina il seguente programma e rispondi alle domande:
```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(vec![1, 2, 3]);

    {
        let mut borrowed = data.borrow_mut();
        borrowed.push(4);
    }

    println!("Data: {:?}", data.borrow());

    {
        let borrowed = data.borrow();
        println!("Length: {}", borrowed.len());
    }

    let mut borrowed_mut = data.borrow_mut(); // Questo genera un errore
    borrowed_mut.push(5);
}
```

1. Qual è il problema del codice?  
2. Come si può risolvere mantenendo lo stesso comportamento?

---

### Soluzione

1. **Problema**: Non è possibile prendere un `borrow_mut` mentre è ancora attivo un `borrow` immutabile. L'ultima riga genera un errore runtime perché Rust rileva un doppio prestito incompatibile.  
2. **Soluzione**:
   ```rust
   use std::cell::RefCell;

   fn main() {
       let data = RefCell::new(vec![1, 2, 3]);

       {
           let mut borrowed = data.borrow_mut();
           borrowed.push(4);
       }

       println!("Data: {:?}", data.borrow());

       {
           let borrowed = data.borrow();
           println!("Length: {}", borrowed.len());
       }

       {
           let mut borrowed_mut = data.borrow_mut();
           borrowed_mut.push(5);
       }

       println!("Final data: {:?}", data.borrow());
   }
   ```

---

### Domanda 3: Chiusure e Iteratori

#### Q3
Completa il seguente programma in modo che utilizzi una chiusura per calcolare la somma dei numeri pari in un range specificato.

```rust
fn main() {
    let range = 1..=10;
    let is_even = |x: i32| x % 2 == 0;

    // Usa un iteratore e la chiusura `is_even` per calcolare la somma dei numeri pari
    let sum = range
        .filter(|&x| /* TODO: usa la chiusura */)
        .sum::<i32>();

    println!("Somma dei numeri pari: {}", sum);
}
```

---

### Soluzione
```rust
fn main() {
    let range = 1..=10;
    let is_even = |x: i32| x % 2 == 0;

    // Usa un iteratore e la chiusura `is_even` per calcolare la somma dei numeri pari
    let sum = range
        .filter(|&x| is_even(x)) // Applica la chiusura
        .sum::<i32>();

    println!("Somma dei numeri pari: {}", sum); // Output: 30
}
```

---

### Domanda 4: Programmazione e Tratti - Funzionalità Polimorfiche

#### Q4
Implementa un tratto `Resettable` per gestire la reimpostazione dello stato di un oggetto a un valore predefinito. La struttura deve includere:
- Un metodo `reset(&mut self)` per ripristinare il valore.
- Un metodo `set(&mut self, value: T)` per impostare un nuovo valore.

Scrivi un'implementazione per una struttura generica `Config<T>`.

---

### Soluzione

```rust
trait Resettable<T> {
    fn reset(&mut self);
    fn set(&mut self, value: T);
}

struct Config<T> {
    value: T,
    default: T,
}

impl<T: Clone> Resettable<T> for Config<T> {
    fn reset(&mut self) {
        self.value = self.default.clone();
    }

    fn set(&mut self, value: T) {
        self.value = value;
    }
}

fn main() {
    let mut config = Config {
        value: 10,
        default: 0,
    };

    println!("Initial value: {}", config.value);

    config.set(42);
    println!("Updated value: {}", config.value);

    config.reset();
    println!("Reset value: {}", config.value);
}
```

---

Se hai bisogno di ulteriori simulazioni o modifiche, fammi sapere! 🚀