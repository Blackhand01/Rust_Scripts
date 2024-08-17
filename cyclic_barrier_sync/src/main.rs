use std::{sync::{Arc, Condvar, Mutex}, thread};

// La struttura `BarrierState` rappresenta lo stato interno della barriera.
// Contiene:
// - `waiting`: Numero di thread che stanno aspettando di raggiungere la barriera.
// - `exiting`: Flag che indica se la barriera è "aperta" e i thread possono passare.
struct BarrierState {
    waiting: usize,
    exiting: bool,
} 

// La struttura `CyclicBarrier` rappresenta la barriera ciclica.
// Contiene:
// - `size`: Numero di thread che devono raggiungere la barriera prima che si apra.
// - `generation`: Permette di identificare se i thread sono ancora nella stessa "onda" di sincronizzazione, evita thread troppo veloci
// - `cond`: Una condizione variabile che permette ai thread di aspettare e notificarsi l'un l'altro.
pub struct CyclicBarrier {

    size: usize,
    generation: Mutex<BarrierState>,
    cond: Condvar,

}

impl CyclicBarrier {
    // Funzione `new` che crea una nuova barriera ciclica con un numero specificato di thread (`n`).
    pub fn new(n: usize) -> CyclicBarrier {
        CyclicBarrier {
            size: n,
            generation: Mutex::new(BarrierState { waiting: 0, exiting: false }),
            cond: Condvar::new(),
        }
    }

    // La funzione `wait` viene chiamata dai thread per aspettare alla barriera.
    // Quando tutti i thread chiamano `wait`, la barriera si apre e i thread possono proseguire.
    pub fn wait(&self) {
        // Blocca l'accesso esclusivo allo stato della barriera.
        let mut generation = self.generation.lock().unwrap();

        // I thread non possono procedere se la barriera è aperta (`exiting` è true).
        // Usando `wait_while`, il thread viene sospeso sulla condizione variabile fino a quando
        // la barriera non è chiusa (`exiting` è false).
        generation = self.cond.wait_while(generation, |generation| generation.exiting).unwrap();

        // Incrementa il numero di thread in attesa alla barriera.
        generation.waiting += 1;

        // Se tutti i thread richiesti hanno raggiunto la barriera...
        if generation.waiting == self.size {
            // Imposta la barriera come aperta (`exiting` è true).
            generation.exiting = true;
            // Notifica a tutti i thread in attesa che la barriera è aperta e possono proseguire.
            self.cond.notify_all();
        } else {
            // Se non tutti i thread sono arrivati, il thread attuale aspetta finché la barriera non si apre.
            generation = self.cond.wait_while(generation, |generation| !generation.exiting).unwrap();
        }

        // Una volta che il thread ha attraversato la barriera, decrementa il contatore `waiting`.
        generation.waiting -= 1;

        // Se è l'ultimo thread a passare attraverso la barriera, la barriera viene resettata.
        // Imposta `exiting` su false e notifica a tutti i thread che la barriera è chiusa di nuovo.
        if generation.waiting == 0 {
            generation.exiting = false;
            self.cond.notify_all();
        }
    }
}

fn main() {
    // Il CyclicBarrier è inizializzato con il numero 3
    // => 3 thread devono raggiungere il barrier prima 
    // che tutti possano continuare.
    let abarrrier = Arc::new(CyclicBarrier::new(3));

    let mut v_threads = Vec::new();

     // Avvia 3 thread
    for t in 0..3 {
        // Clona l'Arc contenente il CyclicBarrier per condividerlo tra i thread
        let cbarrier = abarrrier.clone();

        v_threads.push(thread::spawn(move || {
            for num in 0..10 {
                // Ogni thread aspetta che tutti i thread raggiungano il barrier
                cbarrier.wait();
                println!("after barrier {} {}", t, num);
            }
        }));
    }
    
    for t in v_threads {
        t.join().unwrap();
    }
}
