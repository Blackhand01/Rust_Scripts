
### APPELLO DD/MM/YYYY

#### Domanda 1
- **Q1**: "Si implementi un sistema di comunicazione tra thread utilizzando una struttura `BidirectionalChannel<T: Send>`. Ogni thread può inviare e ricevere messaggi bidirezionali. Il canale deve garantire che:
  - Ogni messaggio venga consegnato esattamente una volta.
  - Non ci siano deadlock o perdita di messaggi."
- **Risposta**:
```rust

```

---

#### Domanda 2
- **Q2**: "Si implementi una struttura dati `ConcurrentLazyCache<K, V>` che utilizza `Arc<RwLock<HashMap<K, V>>>` per fornire accesso concorrente a una cache lazy. Spiegare come prevenire le race condition e assicurare che ogni chiave venga valutata una sola volta."
- **Risposta**:
```rust

```

---

#### Domanda 3
- **Q3**: "Realizzare una variante della `RankingBarrier` che supporti una chiamata `abort()`. Tale chiamata interrompe tutti i thread in attesa e impedisce ulteriori utilizzi della barriera. Garantire la thread-safety."
- **Risposta**:
```rust

```

---

#### Domanda 4
- **Q4**: "Si costruisca una pipeline concorrente composta da più fasi in cui ogni fase è rappresentata da un `Looper`. Ogni fase deve:
  - Ricevere un messaggio dalla fase precedente.
  - Eseguire una trasformazione sul messaggio.
  - Inviare il risultato alla fase successiva.
  Implementare e dimostrare la pipeline con un caso pratico."
- **Risposta**:
```rust
```
