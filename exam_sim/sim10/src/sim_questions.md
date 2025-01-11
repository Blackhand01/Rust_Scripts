# APPELLO DD/MM/YYYY

### Domanda 1
- **Q1**: "Si implementi una struttura `CircularBuffer<T>` che rappresenti un buffer circolare thread-safe di capacità fissa. Deve offrire i seguenti metodi:
  - `new(capacity: usize) -> Self`: costruisce un nuovo buffer con capacità massima `capacity`.
  - `push(&self, item: T) -> Result<(), T>`: aggiunge un elemento al buffer, ritornando `Err(item)` se il buffer è pieno.
  - `pop(&self) -> Option<T>`: rimuove e ritorna l'elemento più vecchio nel buffer o `None` se il buffer è vuoto."
- **Risposta**:
```rust
pub struct CircularBuffer<T>{
  buffer: Mutex<Vec<T>>,
  dim: usize,
  head: Mutex<usize>,
  tail: Mutex<usize>,

}

impl<T> CircularBuffer<T>{
  fn new(capacity: usize) -> Self{
    CircularBuffer<T>{
      buffer: Mutex::new(vec![None; capacity]),
      dim: capacity,
      head: Mutex::new(0),
      tail: Mutex::new(0),
    }
  }

  fn push(&self, item: T) -> Result<(), T>{
    let mut buffer = self.buffer.lock().unwrap();
    let mut head = self.head.lock().unwrap();
    let mut tail = self.tail.lock().unwrap();


    let next_tail = (*tail+1) % self.dim;
    if next_tail == *head {
      return Err(item);
    }
    buffer[*tail] = item;
    *tail = next_tail; 
    Ok(())
  }

  fn pop(&self) -> Option<T>{
    let mut buffer = self.buffer.lock().unwrap();
    let mut head = self.head.lock().unwrap();
    let mut tail = self.tail.lock().unwrap();    
    if *head == *tail{
      None,
    }
    else{
      let item = buffer[*head].take();
      *head = (*head+1) % self.dim;
      Some(item)
    }

  }

}

```

---

### Domanda 2
- **Q2**: "Implementare un semaforo thread-safe, utilizzando Mutex e Condvar. Deve supportare i metodi:
  - `new(initial_count: usize) -> Self`
  - `acquire(&self)`
  - `release(&self)`"

- **Risposta**:
```rust
pub struct Semaphore{
  rsc_counter: Mutex<usize>,
  cv: Condvar,
}

impl Semaphore{

  fn new(initial_count: usize) -> Self{
    Semaphore{
      rsc_counter: Mutex::new(initial_count),
      cv: Condvar::new(),
    }
  }

  fn acquire(&self) {
    let mut cnt = self.rsc_counter.lock().unwrap();
    while *cnt == 0{
      cnt = self.cv.wait(cnt).unwrap();
    }
    *cnt-=1;
  }

  fn release(&self) {
    let mut cnt = self.rsc_counter.lock().unwrap();
    *cnt+=1;
    self.cv.notify_all();
  }
}

```


---

### Domanda 3
- **Q3**: "Si implementi una `RankingBarrier` ciclica che permette ai thread di sincronizzarsi su più cicli."
- **Risposta**:
```rust

pub struct BarrierState{
  cnt: usize,
  state: bool,
}

pub struct RankingBarrier{
  barrier: Mutex<BarrierState>,
  cv: Condvar,
  dim: usize,
}


impl RankingBarrier{
  fn new(parties: usize) -> Self{
    RankingBarrier{
      barrier: Mutex::new(BarrierState{
        cnt: 0,
        state: true,
      }),
      cv: Condvar::new(),
      dim: parties,
    }
  }

  // I thread chiamano `wait` per sincronizzarsi sulla barriera.
  // Ritorna il "rank" del thread nella barriera (il suo ordine di arrivo).
  fn wait(&self) -> usize{
    let mut barrier = self.barrier.lock().unwrap();

    barrier.cnt += 1;
    let rank = barrier.cnt;

    if barrier.cnt == self.dim{  // barriera disattivata
      barrier.state = false;
      self.cv.notify_all();
    }
    else{ // barriera attiva
      while barrier.state == true{
        barrier = self.cv.wait(barrier).unwrap();
      }
    }
    
    barrier.cnt-=1;
    if barrier.cnt==0{
      barrier.state = true;
    }
    
    rank
  }
}
```

---

### Domanda 4 (6 punti)
- **Q4**: "Si implementi una pipeline concorrente con i seguenti requisiti:
  - Pattern: `Producer-Consumer`, `Fan-out/Fan-in`.
  - Una fase `Producer` genera numeri casuali.
  - Una fase `Worker` elabora i numeri (moltiplicandoli per un fattore casuale).
  - Una fase `Consumer` raccoglie i risultati e stampa i numeri elaborati.
  - Deve supportare più worker concorrenti e un unico consumer."
