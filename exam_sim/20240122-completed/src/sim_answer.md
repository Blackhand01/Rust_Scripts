Riscaldamento:
- I tipi generics, sono tipi di dato che possono essere specificati al momento del loro utilizzo, invece che essere fissato a un singolo tipo.
Sono utili per implementare sistemi che prescindono dal tipo di dato in cui si può riutilizzare il codice.

pub struct Pair<T, U>{
    a: T,
    b: U,
}

trait Describable{
    fn describe(&self) -> String{
        "descrizione".to_string()
    }
}

impl Describable for i32{
    fn describe(&self) -> String{
        format("sono un numero {}", self)
    }
}

#[derive(Clone)]
pub struct Person{
    name: String,
    age: i32,
}

- Cosa succede se rimuoviamo handle.join()?
potrebbe succedere che il thread secondario non termini l'esecuzione e quindi non stampi il msg, è necessario aspettarlo con handle.join() per poterlo terminare in maneira controllata. Domanda: Perché abbiamo bisogno di Arc quando condividiamo il Mutex tra thread?
perchè un arc permette di fare operazioni atomiche su una struttura condivisa tramite contatore di riferimenti. Domanda: Cosa succede se un thread di scrittura tenta di acquisire il lock mentre altri thread stanno leggendo?
i thread di scrittura viene bloccato fino a quando non è terminata la lettura degli altri threadDomanda: Perché utilizziamo un Mutex insieme a una Condvar?
 li usiamo insieme quando bisogna sincronizzare tra lettore e scrittore in un ambiente multithread. Domanda: Cosa succede se il thread produttore termina prima che il consumatore abbia ricevuto tutti i messaggi?
succede che send(i).unwrap() riceve un None

Result:
    match result{
        Ok(file) => println!("{:?}", file),
        Err(err) =>println!("{:?}", err),
    }

Option:
    match option{
        Some(valore) => println!("{:?}", valore),
        None => println!("nulla"),
    }


Ottimo! Iniziamo con **Domanda 1:**

---

### Domanda 1: LinkedList
Hai una struttura dati `LinkedList<T>` definita come segue:

```rust
pub struct LinkedList<'a, T> {
    pub val: Option<T>,
    pub next: Option<&'a Box<LinkedList<'a, T>>>,
}
```

**Richieste:**
1. **Definire l’occupazione di memoria di un elemento della lista.**
2. **Indicare come sia possibile definire il fine lista.**


Option è un enum, occupa
1 byte di tag per Some/None
e lo spazio per T 

val: 1Byte + T + padding
next: essendo (il valore di option) un puntatore, per Some occuperà lo spazio di un box 8 byte, per None, essendo un puntatore/riferimento userà NPO per non occupare spazio aggiuntivo
---
### Domanda 2: 

Si definisca un esempio in cui, data la necessità di creare N thread, si possano evitare race-
conditions nel momento in cui i thread debbano accedere in scrittura alla stessa risorsa. Si
distingua il caso in cui tale risorsa sia uno scalare e quella in cui sia una struttura più articolata.

In un programma multi-thread, se più thread accedono in scrittura a una risorsa possono verificarsi race-conditions.
Nel caso in cui si tratti di uno scalare, si può evitare con i tipi atomici (AtomicUsize)

let counter = AtomicUsize::new(0)

for _ in 0..10{
    thread::spawn(move ||{
        let counter = &counter; // ogni thread può accedere a counter in maniera atomica
    })
}

Nel caso in cui si tratti di una struttura più articolata, si può evitare con una Mutex che permette la mutua esclusione di una risorsa,
utilizzata con un Arc che permette l'uso della risorsa da più thread

let data = Arc::new(Mutex::new(Vec::new()))

for _ in 0..10{
    thread::spawn(move || {
        let data = Arc::clone(&data);
    })
}


trait Shape{
    fn area(&self) -> f64;
}

struct Circle{
    radius: f64,
}

struct Square{
    side: f64,
}

impl Shape for Square{
    fn area(&self) -> f64{
        self.side*self.side
    }
}

impl Shape for Circle{
    fn area(&self) -> f64{
        let mut r = self.radius*self.radius;
        r*3.14
    }
}

### Domanda 4:
La struttura MultiChannel implementa il concetto di canale con molti mittenti e molti ricevitori.
I messaggi inviati a questo tipo di canale sono composti da singoli byte che vengono recapitati
a tutti i ricevitori attualmente collegati.
Riferimenti a tipi:
use std::result::Result;
use std::sync::mpsc::{Receiver, SendError};


```rust

pub struct MultiChannel{
    canale: Mutex<Vec<Sender<u8>>>, // vettore dei canali
}

impl MultiChannel{
    fn new() -> Self{
        MultiChannel{
            canale: Mutex::new(Vec::new()),
        }
    }
    // crea un nuovo canale senza alcun ricevitore collegato
    fn subscribe(&self) -> Receiver<u8>{
        let (tx, rx) = mpsc::channel();
        self.canale.lock().unwrap().push(tx);
        rx
    }
    // collega un nuovo ricevitore al canale: da quando
    // questo metodo viene invocato, gli eventuali byte
    // inviati al canale saranno recapitati al ricevitore.
    // Se il ricevitore viene eliminato, il canale
    // continuerà a funzionare inviando i propri dati
    // ai ricevitori restanti (se presenti), altrimenti
    // ritornerà un errore
    fn send(&self, data: u8) -> Result<(), SendError<u8>>{
        let mut canale = self.canale.lock().unwrap();
        canale.pop()

    }
    // invia a tutti i sottoscrittori un byte
    // se non c'è alcun sottoscrittore, notifica l'errore
    // indicando il byte che non è stato trasmesso
}


```