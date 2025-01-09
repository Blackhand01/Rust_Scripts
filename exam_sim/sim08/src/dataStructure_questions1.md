### Esercizio 1
Considera il seguente codice:
```rust
struct Point {
    x: f64,
    y: f64,
}

let mut p = Point { x: 3.14, y: 2.71 };
let p_ref = &p;
let p_box = Box::new(p);
```
Calcola la dimensione della memoria allocata nello stack e nello heap su un’architettura a 64 bit. Spiega come la creazione di `Box::new(p)` modifica l’allocazione della memoria rispetto a `p` e `p_ref`.

---

### Esercizio 2
Dato il seguente codice:
```rust
let array = [1u8; 128];
let slice = &array[32..64];
```
Calcola la dimensione della memoria occupata dallo stack e dalla heap. Spiega il funzionamento della slice e la sua influenza sull'allocazione della memoria.

---

### Esercizio 3
Considera il seguente codice:
```rust
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

let root = TreeNode {
    value: 10,
    left: Some(Box::new(TreeNode {
        value: 5,
        left: None,
        right: None,
    })),
    right: Some(Box::new(TreeNode {
        value: 20,
        left: None,
        right: None,
    })),
};
```
Calcola la memoria occupata nello stack e nello heap per la struttura `root` su un’architettura a 64 bit.

---

### Esercizio 4
Dati i seguenti frammenti di codice:
```rust
use std::rc::Rc;

struct Data {
    values: Vec<u64>,
}

let data = Rc::new(Data {
    values: vec![10, 20, 30, 40],
});

let data_clone = Rc::clone(&data);
```
Calcola la dimensione della memoria allocata nello stack e nello heap. Spiega come `Rc` gestisce la memoria condivisa.

---

### Esercizio 5
Considera il seguente codice:
```rust
enum List {
    Empty,
    Cons(i32, Box<List>),
}

let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Empty))));
```
Calcola la memoria occupata dalla struttura `list` nello stack e nello heap. Spiega come `Box` influenza la ricorsione.
```