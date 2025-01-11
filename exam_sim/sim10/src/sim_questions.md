# APPELLO DD/MM/YYYY

### Domanda 1
- **Q1**: "Si implementi una struttura `CircularBuffer<T>` che rappresenti un buffer circolare thread-safe di capacità fissa. Deve offrire i seguenti metodi:
  - `new(capacity: usize) -> Self`: costruisce un nuovo buffer con capacità massima `capacity`.
  - `push(&self, item: T) -> Result<(), T>`: aggiunge un elemento al buffer, ritornando `Err(item)` se il buffer è pieno.
  - `pop(&self) -> Option<T>`: rimuove e ritorna l'elemento più vecchio nel buffer o `None` se il buffer è vuoto."

---

### Domanda 2
- **Q2**: "Implementare un semaforo thread-safe, utilizzando Mutex e Condvar. Deve supportare i metodi:
  - `new(initial_count: usize) -> Self`
  - `acquire(&self)`
  - `release(&self)`"

---

### Domanda 3
- **Q3**: "Si implementi una `RankingBarrier` ciclica che permette ai thread di sincronizzarsi su più cicli. Utilizzare Mutex e Condvar."

---

### Domanda 4 (6 punti)
- **Q4**: "Si implementi una pipeline concorrente con i seguenti requisiti:
  - Pattern: `Producer-Consumer`, `Fan-out/Fan-in`.
  - Una fase `Producer` genera numeri casuali.
  - Una fase `Worker` elabora i numeri (moltiplicandoli per un fattore casuale).
  - Una fase `Consumer` raccoglie i risultati e stampa i numeri elaborati.
  - Deve supportare più worker concorrenti e un unico consumer."
