use std::sync::{Arc, Mutex, Condvar};

struct CyclicBarrier<T> {
    count: Mutex<usize>,        // Contatore per i thread che devono ancora arrivare
    threshold: usize,           // Numero di thread attesi
    condvar: Condvar,           // Condition variable per sincronizzare i thread
    generation: Mutex<usize>,   // Permetterà di identificare se i thread sono ancora 
                                // nella stessa "onda" di sincronizzazione, evita thread
                                // troppo veloci
    values: Mutex<Vec<T>>,      // Vettore per raccogliere i valori passati dai thread
}

impl<T: Send + 'static> CyclicBarrier<T> {
    fn new(threshold: usize) -> Arc<Self> {
         Arc::new(CyclicBarrier {
            count: Mutex::new(threshold),
            threshold,
            condvar: Condvar::new(),
            generation: Mutex::new(0),
            values: Mutex::new(Vec::with_capacity(threshold)), // Inizialmente vuoto, capacità uguale a threshold
        })
    }

    fn wait(&self, value: T)  -> Vec<T> {
        let mut count = self.count.lock().unwrap();
        let mut generation = self.generation.lock().unwrap();
        let mut values = self.values.lock().unwrap();

        *count -= 1;
        let current_generation = *generation;
        
        // Inseriamo il valore nel vettore
        values.push(value);

        // L'ultimo thread ha raggiunto la barriera, la "apriamo"
        if(*count == 0){
            *count = self.threshold;    // Reset del contatore per il prossimo ciclo
            *generation += 1;           // Incremento della generazione per segnalare un nuovo ciclo
            self.condvar.notify_all();  // Risveglia tutti i thread in attesa
        }
        else{
            // Aspettiamo che tutti i thread raggiungano la barriera
            while(*count != self.threshold && *generation == current_generation ) {
                // il thread corrente attende su una Condvar, rilasciando temporaneamente il controllo del Mutex associato a count
                count = self.condvar.wait(count).unwrap();
            }
            // Ritorniamo i valori raccolti, anche se siamo uno dei thread in attesa
            values.clone()
        }
    }
}



fn main() {
    // Il CyclicBarrier è inizializzato con il numero 3
    // => 3 thread devono raggiungere il barrier prima 
    // che tutti possano continuare.
    let abarrrier = Arc::new(cb::CyclicBarrier::new(3));

    let mut vt = Vec::new();

     // Avvia 3 thread
    for i in 0..3 {
        // Clona l'Arc contenente il CyclicBarrier per condividerlo tra i thread
        let cbarrier = abarrrier.clone();

        vt.push(thread::spawn(move || {
            for j in 0..10 {
                // Ogni thread aspetta che tutti i thread raggiungano il barrier
                cbarrier.wait();
                println!("after barrier {} {}", i, j);
            }
        }));
    }
    
    for t in vt {
        t.join().unwrap();
    }
}
