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

stack
- p = 16 B
- p_ref = 8 B
- p_box = 8 B
heap
- Box[p] occupa 16 B
---

### Esercizio 2
Dato il seguente codice:
```rust
let array = [1u8; 128];
let slice = &array[32..64];
```
Calcola la dimensione della memoria occupata dallo stack e dalla heap. Spiega il funzionamento della slice e la sua influenza sull'allocazione della memoria.

stack
- array = 128 Byte
- slice = 8 Byte puntatore + length (usize) = 8 Byte
heap
- 0 Byte

Una slice consente di accedere a una porzione di un array senza allocare memoria ma tramite puntatore
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
stack: 20 B -> allineati a 24 B
- value: i32 = 4 Byte, (+4B allineamento)
- left: Optional<Box> = 8 Byte di Box (0 Byte di Tag per NPO)
- right: Optional<Box> =  8 Byte di Box

heap: (20 + 4)B x2 = 48 Byte
- <ThreeNode> x2 (left e right):
    - value = 4 Byte, (+4B per allineamento)
    - left = 8 Byte di Box + 0 (None -> NPO)
    - right = 8 Byte di Box + 0 (None -> NPO)
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

Risposta:
stack 16 Byte
- data: 8 Byte puntatore (Rc)
- data_clone: 8 Byte puntatore (Rc)

heap 16 + 24 + 32 = 72 Byte
- data: 16 Byte metadati puntatore (Rc), strong e weak cnt (2 usize)
- values:
    - Vec: 8 byte di puntatore + 8 byte length + 8 byte capacity
    - vec! : 8x4 = 32 byte
Totale 88 byte

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
stack
List{
- enum occupa 1 Byte di tag + 7 Byte di allineamento 
    - Cons ha 2 campi:
        - Box: puntatore (8 Byte)
        - valore: i32 (4 Byte + 4 allineamneto)
} Occupa: 8 + 16 = 24 byte su stack per il primo elemento (1)

heap
- 2 oggetti List (2, empty):
    - per 2 occuperemo 24 Byte
    - per empty occuperemo comunque 24 Byte

Totale: 56 Byte
