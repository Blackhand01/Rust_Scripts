# APPELLO DD/MM/YYYY

## 1. Sincronizzazione tra Thread

### Esercizio 1
1. Si consideri il programma seguente che riporta la numerazione delle linee di codice:
 - Si tratta di un programma in cui ci sono 4 thread.
Il thread principale mette in condivisione la risorsa data attraverso Arc che punta a una tupla formata da una Mutex che protegge un vettore e una condvar. Dopodichè crea il thread writer, il thread reader 1 e il thread reader 2, li attende e li dealloca.

- Il thread writer legge un clone di data per accedere alla mutex e alla condvar (lock, cvar).
Dopodichè in un ciclo, dormirà per un secondo e poi acquisirà il lock su lock e metterà nel vettore il valore del contatore del ciclo (i). Alla fine, notificherà a tutti gli altri thread che ha finito.

- Il thread reader 1 legge un altro clone di data per accedere ai  campi della tupla e entra in un ciclo.
Dopodichè acquisirà il lock e attenderà una notiica su cvar, se dentro data trova qualcosa lo stampa, altrimenti stamperà solo l'id del reader ("1").

- Il thread reader 2 legge direttamente data per accedere ai  campi della tupla e entra in un ciclo.
Dopodichè acquisirà il lock e attenderà una notiica su cvar.
Ricevuta la notifica, se data contiene qualcosa, lo stamperà altrimenti stamperà solo 2

2. La prima problematica è che reader 2 usa direttamente data e non un suo clone, il controllo su data lo ha quindi il thread reader 2, rompendo la condivisione della risorsa tra thread.

3. no, mai perchè quelle print si verificano solo se facendo pop non ci sia più niente nel vettore, ma nel vettore non ci sarà più niente dopo il 5° ciclo del writer. Tuttavia, dopo che verranno piazzati 5 elementi e verranno chiamate altrettante notify_one, i reader rimarrando in attesa (rimarranno su wait). 
```rust


use std::sync::{Arc, Mutex, Condvar};
use std::thread;

fn main() {
    let data = Arc::new((Mutex::new(Vec::new()), Condvar::new()));

    let data_clone1 = Arc::clone(&data);
    let data_clone2 = Arc::clone(&data);

    let writer = thread::spawn(move || {
        let (lock, cvar) = &*data_clone1;
        for i in 1..=5 {
            thread::sleep(std::time::Duration::from_secs(1));
            let mut data = lock.lock().unwrap();
            data.push(i);
            cvar.notify_all();
        }
    });

    let reader1 = thread::spawn(move || {
        let (lock, cvar) = &*data_clone2;
        loop {
            let mut data = lock.lock().unwrap();
            data = cvar.wait(data).unwrap();

            if let Some(value) = data.pop() {
                println!("Reader 1: read {}", value);
            } else {
                println!("(1)");
            }
        }
    });

    let reader2 = thread::spawn(move || {
        let (lock, cvar) = &*data;
        loop {
            let mut data = lock.lock().unwrap();
            data = cvar.wait(data).unwrap();

            if let Some(value) = data.pop() {
                println!("Reader 2: read {}", value);
            } else {
                println!("(2)");
            }
        }
    });
            let (lock, cvar) = &*data;

    let mut data = lock.lock().unwrap();
    data.push(2);

    writer.join().unwrap();
    reader1.join().unwrap();
    reader2.join().unwrap();
}

----------------------------------------
1 use std::sync::{Arc, Mutex, Condvar};
2 use std::thread;
3
4 fn main() {
5 let data = Arc::new((Mutex::new(Vec::new()), Condvar::new()));
6
7 let data_clone1 = Arc::clone(&data);
8 let data_clone2 = Arc::clone(&data);
9
10 let writer = thread::spawn(move || {
11 let (lock, cvar) = &*data_clone1;
12 for i in 1..=5 {
13 thread::sleep(std::time::Duration::from_secs(1));
14
15 let mut data = lock.lock().unwrap();
16 data.push(i);
17 cvar.notify_all();
18 }
19 });
20
21 let reader1 = thread::spawn(move || {
22 let (lock, cvar) = &*data_clone2;
23 loop {
24 let mut data = lock.lock().unwrap();
25
26 data = cvar.wait(data).unwrap();
27
28 if let Some(value) = data.pop() {
29 println!("Reader 1: read {}", value);
30 }
31 else {
32 println!("(1)");
33 }
34 }
35 });
36
37 let reader2 = thread::spawn(move || {
38 let (lock, cvar) = &*data;
39 loop {
40 let mut data = lock.lock().unwrap();
41
42 data = cvar.wait(data).unwrap();
43
44 if let Some(value) = data.pop() {
45 println!("Reader 2: read {}", value);
46 }
47 else {
48 println!("(2)");
49 }
50 }
51 });
52 writer.join().unwrap();
53 reader1.join().unwrap();
54 reader2.join().unwrap();
55 }
```

- **Domanda 1**: Tenendo conto che non ci sono errori di compilazione:
  1. Si descriva il comportamento del programma.
  2. Si indichino le possibili problematiche e le eventuali azioni correttive.
  3. Si dica se possono essere eseguite le istruzioni alle righe 32 e 48, eventualmente spiegando il motivo per cui ci si trovi in quella situazione.

---

## 2. Borrow Checker

### Esercizio 1
Il codice seguente genera un errore di compilazione. Si spieghi il motivo e si indichi come modificare la struct `S` (attraverso l'aggiunta di tratti) per renderlo compilabile ed eseguibile.

```rust
#[derive(Debug, Clone)]
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
    for i in 0..3 {
        v.push(s.clone());
    }
    println!("{:?}", v);
}
```
Dopo la prima push, s non è più disponibile. è necessario implementare il tratto clone.

---

## 3. Allocazione di Memoria

### Esercizio 1
Dato il seguente codice, si descrivano le allocazioni in memoria dello stack e dello heap.

```rust
use std::rc::Rc;

let rc: Rc<u64> = Rc::new(42);
let rc2 = rc.clone();
let wk = Rc::downgrade(&rc);
```

- **Domanda 1**: Spiegare come vengono allocati i byte nello stack e nello heap, includendo le strutture di controllo degli `Rc`.

- stack
    - rc -> puntatore = 8 Byte
    - rc2 -> puntatore = 8 Byte
    - wk -> puntatore = 8 Byte


- heap
     - 42 -> è un valore di 8 Byte
     - strong counter -> 1 usize = 8 Byte
     - weak counter -> 1 usize = 8 Byte


heap 16 Byte
stack: 24 Byte 
    
---

## 4. Chiusure

### Esercizio 1
Si descriva nel dettaglio il comportamento del seguente programma, spiegando l’errore di compilazione generato. Inoltre, si indichi come modificarlo per ottenere la visualizzazione di `Numero: 4`.

```rust
fn main() {
    let mut data = vec![1, 2, 3, 4, 5];
    data.push(60); // 1, 2, 3, 4, 5, 60, 40 e poi data viene consegnato in proprietà a process_data che dopo la stampa lo consuma e poi si cerca di inserire il valore 30. per stampare il numero 4

    let mut process_data = move || {
        data.push(50);
        let count = data.iter().filter(|&x| x % 2 == 0).count();
        println!("Numero: {:?}", count);
    };

    process_data();
}
```

---

## 5. Programmazione

### Esercizio 1
Si implementi in Rust la struct `CountDownLock` che permette a uno o più thread di attendere che un gruppo di operazioni eseguite da altri thread sia completato.  

Essa incapsula un contatore e offre i seguenti tre metodi thread-safe (oltre alla propria funzione costruttrice):

```rust

pub struct CountDownLock{
    counter: Mutex<usize>,
    cv: Condvar,
}

impl CountDownLock{
    fn new(n: usize) -> Self{
        CountDownLock{
            counter: Mutex::new(n),
            cv: Condvar::new(),
        }
    }

    fn count_down(&self){
        let mut cnt = self.counter.lock().unwrap();
        if *cnt > 0{
            cnt -= 1;
        }
        else{
            self.cv.notify_all();
        }
    }

    // Decrementa il contatore, se maggiore di 0.

    // Blocca l'esecuzione del chiamante senza consumare cicli di CPU, finché il contatore non diventa zero.
    fn wait(&self){ 
        let mut cnt = self.cnt.lock().unwrap();
        while *cnt != 0{
            *cnt = self.cv.wait(*cnt).unwrap();
            }
    }

    fn wait_timeout(&self, d: Duration) -> std::sync::WaitTimeoutResult{
        let mut cnt = self.cnt.lock().unwrap();
        *cnt = self.cv.wait_timeout_while(cnt, d, |cnt| *cnt>0).unwrap();
        
    }
    // Blocca l'esecuzione del chiamante senza consumare cicli di CPU, in attesa che il contatore raggiunga 0 per una durata massima pari a `d`,
    // restituendo il risultato dell'attesa.
}

```
```