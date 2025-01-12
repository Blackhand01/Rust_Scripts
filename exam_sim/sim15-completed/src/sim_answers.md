Ecco una nuova simulazione d'esame Rust con argomenti variati e soluzioni incluse:

---

# Simulazione d'esame Rust

**Data: DD/MM/YYYY**

---

### Domanda 1: Borrow Checker e Lifetime

#### Q1
Esamina il seguente codice e risolvi i problemi di compilazione relativi ai tempi di vita (`lifetime`):

```rust
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let string1 = String::from("Rust");
    let result;
    {
        let string2 = String::from("Programming");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is: {}", result);
}
```

1. Qual è il problema con il codice?  
2. Riscrivi il codice in modo che compili e funzioni correttamente.

---

### Soluzione

1. **Problema**: La stringa `string2` viene deallocata al termine del blocco interno, ma il riferimento `result` punta ancora a essa, causando un problema di lifetime.  
2. **Versione corretta**:
   ```rust
   fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
       if s1.len() > s2.len() {
           s1
       } else {
           s2
       }
   }

   fn main() {
       let string1 = String::from("Rust");
       let string2 = String::from("Programming");
       let result = longest(string1.as_str(), string2.as_str());
       println!("The longest string is: {}", result);
   }
   ```

---

### Domanda 2: Concorrenza e Arc<Mutex<T>>

#### Q2
Completa il seguente codice per creare un contatore thread-safe condiviso tra più thread. Ogni thread deve incrementare il contatore 10 volte.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let mut num = counter_clone.lock().unwrap();
                // Incrementa il contatore
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
}
```

---

### Soluzione

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1; // Incrementa il contatore
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap()); // Output: 50
}
```

---

### Domanda 3: Chiusure e Map

#### Q3
Scrivi una chiusura che trasformi ogni elemento di un vettore di numeri interi in una stringa formattata come `"Numero: x"`, dove `x` è il numero.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Applica una chiusura per trasformare i numeri in stringhe
    let formatted: Vec<String> = numbers.into_iter().map(/* TODO */).collect();

    for s in formatted {
        println!("{}", s);
    }
}
```

---

### Soluzione

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Applica una chiusura per trasformare i numeri in stringhe
    let formatted: Vec<String> = numbers
        .into_iter()
        .map(|x| format!("Numero: {}", x))
        .collect();

    for s in formatted {
        println!("{}", s);
    }
}
```

---

### Domanda 4: Programmazione - Tratti e Tipi Generici

#### Q4
Crea un tratto `Measurable` che calcola la lunghezza di un oggetto. Implementa il tratto per le seguenti strutture:
- `Line` (linea tra due punti 2D)
- `Rectangle` (rettangolo con larghezza e altezza)

Il tratto deve includere un metodo:
```rust
fn length(&self) -> f64;
```

---

### Soluzione

```rust
use std::f64;

trait Measurable {
    fn length(&self) -> f64;
}

struct Line {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Measurable for Line {
    fn length(&self) -> f64 {
        ((self.x2 - self.x1).powi(2) + (self.y2 - self.y1).powi(2)).sqrt()
    }
}

impl Measurable for Rectangle {
    fn length(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

fn main() {
    let line = Line {
        x1: 0.0,
        y1: 0.0,
        x2: 3.0,
        y2: 4.0,
    };

    let rectangle = Rectangle {
        width: 5.0,
        height: 10.0,
    };

    println!("Line length: {}", line.length()); // Output: 5.0
    println!("Rectangle perimeter: {}", rectangle.length()); // Output: 30.0
}
```

---

Se hai bisogno di ulteriori simulazioni o chiarimenti, sono qui per aiutarti! 🚀