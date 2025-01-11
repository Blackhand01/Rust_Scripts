### APPELLO DD/MM/YYYY

---

#### **Domanda 1**
- **Q1**: "Si implementi un sistema di comunicazione tra thread utilizzando una struttura `BidirectionalChannel<T: Send>`. Ogni thread può inviare e ricevere messaggi bidirezionali. Il canale deve garantire che:
  - Ogni messaggio venga consegnato esattamente una volta.
  - Non ci siano deadlock o perdita di messaggi."

**Metodi da implementare**:
```rust
pub struct BidirectionalChannel<T> { ... }

impl<T: Send> BidirectionalChannel<T> {
    pub fn new() -> (Arc<Self>, Arc<Self>);
    pub fn send(&self, msg: T);
    pub fn receive(&self) -> T;
}
```

---

#### **Domanda 2**
- **Q2**: "Si implementi una struttura dati `ConcurrentLazyCache<K, V>` che utilizza `Arc<RwLock<HashMap<K, V>>>` per fornire accesso concorrente a una cache lazy. Spiegare come prevenire le race condition e assicurare che ogni chiave venga valutata una sola volta."

**Metodi da implementare**:
```rust
pub struct ConcurrentLazyCache<K, V> { ... }

impl<K, V> ConcurrentLazyCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub fn new() -> Self;
    pub fn get_or_insert_with<F>(&self, key: K, value_fn: F) -> V
    where
        F: FnOnce() -> V;
}
```

---

#### **Domanda 3**
- **Q3**: "Realizzare una variante della `RankingBarrier` che supporti una chiamata `abort()`. Tale chiamata interrompe tutti i thread in attesa e impedisce ulteriori utilizzi della barriera. Garantire la thread-safety."

**Metodi da implementare**:
```rust
pub struct RankingBarrier { ... }

impl RankingBarrier {
    pub fn new(parties: usize) -> Self;
    pub fn wait(&self) -> Result<usize, &'static str>;
    pub fn abort(&self);
}
```

---

#### **Domanda 4**
- **Q4**: "Si costruisca una pipeline concorrente composta da più fasi in cui ogni fase è rappresentata da un `Looper`. Ogni fase deve:
  - Ricevere un messaggio dalla fase precedente.
  - Eseguire una trasformazione sul messaggio.
  - Inviare il risultato alla fase successiva."

**Metodi da implementare**:
```rust
pub fn looper<F, T>(rx: Receiver<T>, tx: Sender<T>, transform: F)
where
    F: Fn(T) -> T + Send + 'static,
    T: Send + 'static;
```