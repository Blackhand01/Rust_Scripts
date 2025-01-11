# Esercizi sulle Strutture Dati

## Esercizio 1
Considera la seguente struttura dati:
```rust
struct Node {
    value: u32,
    next: Option<Box<Node>>,
}

let node1 = Node {
    value: 1,
    next: None,
};

let node2 = Node {
    value: 2,
    next: Some(Box::new(node1)),
};
```
Calcola la dimensione della memoria occupata da `node2` nello stack e nello heap su un'architettura a 64 bit. Spiega il motivo.

stack 16
- value: 4 Byte + 4
- next: 
    - enum Option: 1B tag + 7  // !non serve per NPO!
    - Box<Node>: 8B puntatore
heap 16
    - <node1>:
    - value: 4 Byte + 4
    - next: 
        - Box<Node>: 8B puntatore



---

## Esercizio 2
Dato il seguente codice:
```rust
struct Container {
    data: Vec<u64>,
    metadata: [u32; 3],
}

let container = Container {
    data: vec![10, 20, 30],
    metadata: [1, 2, 3],
};
```
Calcola la memoria allocata nello stack e nello heap per l'oggetto `container`.

---

## Esercizio 3
Si consideri il seguente codice:
```rust
struct Wrapper {
    pointer: Rc<i32>,
    additional: u64,
}

let wrapper = Wrapper {
    pointer: Rc::new(10),
    additional: 50,
};
```
Calcola la memoria allocata nello stack e nello heap. Spiega il ruolo del contatore di riferimenti in `Rc`.

---

## Esercizio 4
Considera la seguente struttura:
```rust
struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

let matrix = Matrix {
    rows: 2,
    cols: 3,
    data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
};
```
Calcola la memoria allocata nello stack e nello heap.

---

## Esercizio 5
Analizza la seguente struttura:
```rust
struct TreeNode {
    value: i64,
    children: Vec<Box<TreeNode>>,
}

let root = TreeNode {
    value: 10,
    children: vec![
        Box::new(TreeNode {
            value: 20,
            children: vec![],
        }),
        Box::new(TreeNode {
            value: 30,
            children: vec![],
        }),
    ],
};
```
Determina la memoria occupata da `root` nello stack e nello heap.
```

--- 
