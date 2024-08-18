use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};

pub struct Waiter {
    receiver: Arc<Mutex<Receiver<()>>>,
    senders: Vec<Sender<()>>,
}

impl Waiter {
    pub fn wait(&self) {
        // Manda segnali a tutti gli altri thread
        for sender in &self.senders {
            sender.send(()).unwrap();
        }

        // Aspetta di ricevere segnali dagli altri thread
        for _ in 0..self.senders.len() {
            self.receiver.lock().unwrap().recv().unwrap();
        }
    }
}

pub struct CyclicBarrier {
    waiters: Vec<Waiter>,  // Vettore di Waiter
}

impl CyclicBarrier {
    pub fn new(n: usize) -> Arc<Self> {
        let mut waiters = Vec::with_capacity(n);

        // Creiamo i Waiter per ciascun thread
        for i in 0..n {
            let mut senders = Vec::with_capacity(n - 1);
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));

            for j in 0..n {
                if i != j {
                    let (sender, _) = mpsc::channel();
                    senders.push(sender);
                }
            }

            waiters.push(Waiter { receiver, senders });
        }

        Arc::new(CyclicBarrier { waiters })
    }

    // Restituisce un Waiter in base all'indice
    pub fn get_waiter(&self, index: usize) -> Waiter {
        self.waiters[index].clone()
    }
}

impl Clone for Waiter {
    fn clone(&self) -> Self {
        Waiter {
            receiver: Arc::clone(&self.receiver),
            senders: self.senders.clone(),
        }
    }
}

fn main() {
    // Creiamo un CyclicBarrier per 3 thread
    let barrier = CyclicBarrier::new(3);
    let mut v_threads = Vec::new();

    // Creiamo 3 thread
    for i in 0..3 {
        // Ogni thread riceve un Waiter unico
        let barrier = barrier.clone();
        v_threads.push(std::thread::spawn(move || {
            let waiter = barrier.get_waiter(i);

            for j in 0..10 {
                // Ogni thread aspetta che tutti gli altri thread raggiungano il barrier
                waiter.wait();
                println!("after barrier {} {}", i, j);
            }
        }));
    }

    // Aspettiamo che tutti i thread finiscano
    for t in v_threads {
        t.join().unwrap();
    }
}
