use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

// Definiamo la struttura ThreadPool
pub struct ThreadPool {
    workers: Vec<Worker>, // Una lista di worker
    sender: mpsc::Sender<Message>, // Un canale per inviare messaggi ai worker
}

// Rappresenta il thread che esegue i job
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>, // L'handle del thread
}

// Tipi di msg che possono essere inviati ai worker
enum Message {
    NewJob(Job), // Nuovo job da eseguire
    JobDone,     // Job eseguito
}

// Definiamo un tipo per il Job, che è una funzione dentro un Box
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {

    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // Canale mpsc per comunicazione tra workers
        let (sender, receiver) = mpsc::channel();

        // Condividiamo il receiver tra i worker (un receiver condiviso)
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        // Ritorniamo il ThreadPool con la lista di worker e il sender
        ThreadPool { workers, sender }
    }

      pub fn execute<F>(&self, f: F)
      where
          F: FnOnce() + Send + 'static,
      {
          // Incapsuliamo la funzione in un Box e la inviamo al canale
          let job = Box::new(f);
          self.sender.send(Message::NewJob(job)).unwrap();
      }
}

impl Drop for ThreadPool {

    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::JobDone).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}


impl Worker {

    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
    
        // Creiamo un thread che ascolta il canale e esegue i job
        let thread = thread::spawn(move || 
        
            loop {
                // Acquisiamo il lock sul receiver e attendiamo per un messaggio
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job();
                    }
                    Message::JobDone => {
                        println!("Worker {} finished job.", id);
                        break;
                    }
                }
            }); 

        Worker { id, thread: Some(thread) }
    }
}


fn main() {
    // Alloca i worker
    let threadpool = ThreadPool::new(4); // Creiamo un pool di 4 worker

    for i in 0..8 {
        // Aggiungiamo dei job al threadpool
        threadpool.execute(move || {
            println!("Executing task {}", i);
            thread::sleep(Duration::from_secs(2)); // Simuliamo un lavoro lungo
            println!("Task {} completed", i);
        });
    }

    // Manteniamo il main thread vivo per vedere l'esecuzione dei thread nel pool
    thread::sleep(Duration::from_secs(10));
    println!("Main thread done");
}
