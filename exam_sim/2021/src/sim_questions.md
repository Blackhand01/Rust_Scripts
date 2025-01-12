Ecco la simulazione aggiornata con la domanda mancante e tutte le date degli appelli:

---

# Simulazione d'esame - Rust

---

### **Domanda 1: Struttura Generica `Exchanger<T>`**  
**Appello del 15 febbraio 2021**  

**Q1**: La struttura generica `Exchanger<T>` permette a due thread di scambiarsi un valore di tipo `T`. Essa offre esclusivamente il metodo pubblico:  
```rust
fn exchange(&self, t: T) -> Option<T>
```  
Il metodo:
- Blocca il thread chiamante senza consumare CPU fino a che un altro thread non invoca lo stesso metodo sulla stessa istanza.
- Restituisce l’oggetto passato come parametro dal thread opposto.

Si implementi tale struttura utilizzando la libreria standard di Rust.

---

### **Domanda 2: Struttura `SingleThreadExecutor`**  
**Appello del 19 giugno 2021**  

**Q2**: Si implementi una struttura `SingleThreadExecutor<F: FnOnce() + Send + 'static>` che realizzi il concetto di ThreadPool basato su un singolo thread.

**Caratteristiche richieste:**
1. **Coda dei compiti**: La struttura deve utilizzare una coda per accodare i compiti da eseguire. I compiti vengono rappresentati come funzioni o chiusure che soddisfano i tratti `FnOnce() + Send`.
2. **Metodo `submit(...)`**: Permette di affidare un compito all'istanza di `SingleThreadExecutor`. I compiti vengono accodati e saranno disponibili per l'elaborazione. Se l'esecutore è stato chiuso, eventuali tentativi di invocare `submit(...)` devono restituire un errore.
3. **Metodo `close()`**: Impedisce l'ulteriore accodamento di compiti. Dopo la chiamata a `close()`, eventuali invocazioni di `submit(...)` devono fallire con un errore. Tuttavia, i compiti già accodati devono essere eseguiti.

**Implementazione iniziale:**
```rust
struct SingleThreadExecutor<F: FnOnce() + Send> {...}

impl<F: FnOnce() + Send> SingleThreadExecutor<F> {
    pub fn new() -> (Self, Receiver<Box<F>>);
    pub fn submit(&self, task: F) -> Result<(), String>;
    pub fn close(&mut self);
}
```

Si completi l’implementazione utilizzando le funzionalità della libreria standard di Rust.

---

### **Domanda 3: Struttura Generica `Buffer<T>`**  
**Appello del 5 luglio 2021**  

**Q3**: Si implementi una struttura generica `Buffer<T>` che modelli una struttura dati condivisa tra due thread concorrenti: un produttore e un consumatore. La struttura deve consentire al produttore di inserire valori e notificare la terminazione della produzione, rispettando le seguenti regole:

1. **Metodo `next(...)`**: Aggiunge un nuovo valore al buffer. Se è stata invocata una chiamata a `terminate()` o `fail(...)`, il metodo deve fallire e il buffer deve rimanere inalterato.
2. **Metodo `terminate()`**: Notifica che non saranno disponibili ulteriori valori. Dopo questa chiamata:
   - Il buffer non accetta più nuovi valori.
   - Se non ci sono valori nel buffer, `consume()` deve restituire `None`.
3. **Metodo `fail(...)`**: Notifica un errore e indica che non saranno disponibili ulteriori valori. Dopo questa chiamata:
   - Il buffer non accetta più nuovi valori.
   - Se non ci sono valori nel buffer, `consume()` deve restituire un errore.
4. **Metodo `consume()`**: Preleva un valore dal buffer in modalità FIFO. Se non ci sono valori disponibili, il metodo si blocca in attesa di nuovi valori o di una condizione di terminazione, senza consumare CPU.

**Implementazione iniziale:**
```rust
impl<T: Send> Buffer<T> {
    pub fn new() -> Self;
    
    // Metodi relativi alla produzione di valori
    pub fn next(&self, value: T);
    pub fn terminate(&self);
    pub fn fail(&self, error: Box<dyn Any + Send>);

    // Metodo relativo al consumo di valori
    pub fn consume(&self) -> Result<Option<T>, Box<dyn Any + Send>>;
}
```

---

### Domanda 4: Buffer Circolare `CircularBuffer<T>`
**Appello del 2 settembre 2021**  

**Q4**: In un sistema concorrente, due gruppi di thread (produttori e consumatori) utilizzano una struttura dati condivisa thread-safe che implementa il concetto di buffer circolare. La struttura ospita un array di `N` elementi (specificato a livello di tipo) e gestisce i dati in modalità FIFO.

**Operazioni richieste:**
1. **`insert(&self, t: T)`**:
   - Inserisce un elemento.
   - Se il buffer è pieno, l’operazione si blocca in attesa di spazio libero.
2. **`extract(&self) -> T`**:
   - Estrae un elemento.
   - Se il buffer è vuoto, l’operazione si blocca in attesa di nuovi valori.
3. **`try_insert_for(&self, t: T, d: Duration) -> TimeoutResult`**:
   - Cerca di inserire un elemento con attesa limitata.
4. **`try_extract_for(&self, d: Duration) -> (Option<T>, TimeoutResult)`**:
   - Cerca di estrarre un elemento con attesa limitata.

**Implementazione iniziale:**
```rust
struct CircularBuffer<T: Send + 'static> {...}

impl<T: Send + 'static> CircularBuffer<T> {
    pub fn insert(&self, t: T);
    pub fn extract(&self) -> T;
    pub fn try_insert_for(&self, t: T, d: Duration) -> TimeoutResult;
    pub fn try_extract_for(&self, d: Duration) -> (Option<T>, TimeoutResult);
}
```

Si implementi la classe generica utilizzando le funzionalità offerte dal linguaggio Rust.

---


---
### **Domanda 5: Struttura Generica `Processor<T>`**  
**Appello del 18 ottobre 2021**  

**Q5**: La struttura generica `Processor<T>` consente a un insieme di thread produttori di inviare oggetti istanza del tipo `T` (che si assume copiabile) a un thread consumatore, il cui comportamento è definito tramite una funzione fornita come parametro del costruttore.

**Caratteristiche richieste:**
1. **Metodo `send(&self, item: T)`**: Permette ai produttori di sottomettere un oggetto da elaborare. L'oggetto viene inserito in una coda in attesa che il thread consumatore esterno lo elabori.  
   - Se il metodo `close(...)` è stato invocato, eventuali chiamate a `send(...)` devono generare un errore o produrre un comportamento indefinito.  
2. **Metodo `close(&self)`**: Segnala la fine dell'accettazione di nuovi elementi. Dopo l'invocazione:
   - Non sarà più possibile inviare nuovi dati tramite `send(...)`.
   - Il metodo non ritorna fino a quando la coda non è vuota e tutte le operazioni di elaborazione sono state completate.

**Implementazione iniziale:**
```rust
impl<T: Send> Processor<T> {
    fn new<F>(f: F) -> Self where F: Fn(T) + Send {}
    fn send(&self, item: T) {}
    fn close(&self) {}
}
```

Si implementi la classe `Processor<T>` utilizzando le primitive di sincronizzazione di Rust per garantire il comportamento richiesto. La logica di elaborazione e il ciclo di vita del thread consumatore devono essere gestiti esternamente.

---

Se hai bisogno di ulteriori modifiche o chiarimenti, fammi sapere! 🚀

