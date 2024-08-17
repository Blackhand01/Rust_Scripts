use std::sync::Arc;
use std::thread;

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
