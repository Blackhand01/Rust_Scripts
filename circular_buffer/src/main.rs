ususe std::mem;
use std::ops::{Deref, DerefMut, Index, IndexMut};

//------------------------------------------------
// CircularBuffer
//------------------------------------------------
pub struct CircularBuffer<T> {
    data: Vec<T>,
    tail: usize,
    head: usize,
    len: usize
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Default> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buf = Vec::with_capacity(capacity);
        for _ in 0..capacity { buf.push(T::default()); }
        CircularBuffer{
            data: buf,
            tail: 0,
            head: 0,
            len: 0
        }
        
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.len == self.data.len() {
            return Err(Error::FullBuffer);
        } else {
            self.data[self.tail] = _element;
            self.tail = (self.tail + 1) % self.data.len();
            self.len += 1;
            return Ok(());
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.len == 0 {
            return Err(Error::EmptyBuffer);
        } else {
            let element = mem::take(&mut self.data[self.head]);
            self.head = (self.head + 1) % self.data.len();
            self.len -= 1;
            return Ok(element);
        }
    }

    pub fn clear(&mut self) {
        while self.len > 0 {
            self.read().unwrap();
        }
    }

    pub fn overwrite(&mut self, _element: T) {
        // if it's full, we need to read one element and discard it
        if self.len == self.data.len() {
            self.read().unwrap();
        } 
        self.write(_element).unwrap();
    }

    fn make_contiguous(&mut self) {
        
        // if it's empty, we can just reset the pointers
        if self.len == 0 {
            self.head = 0;
            self.tail = 0;
        } else {
            // otherwise we need to make it contiguos: just rotate it until head is zero
            while self.head != 0 {
                let element = self.read().unwrap();
                self.write(element).unwrap();
            }
        } 
    }

    fn real_index(&self, index: usize) -> usize {
        if index >= self.len {
            panic!("out of bounds");
        }
        (self.head + index) % self.data.len()
    }

}



impl<T: Default> Index<usize> for CircularBuffer<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {

        &self.data[self.real_index(index)]
    }
}

impl<T: Default> IndexMut<usize> for CircularBuffer<T> {
    
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        
        let idx = self.real_index(index);
        &mut self.data[idx]
    }
}

impl <T> Deref for CircularBuffer<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        if self.head > self.tail {
            panic!("not contiguous!!!")
        }
        &self.data[self.head..self.tail]
    }
}

pub trait TryDeref {
    type Target: ?Sized ;

    fn try_deref(&self) -> Result<&Self::Target, String>;
}

impl<T: Default> TryDeref for CircularBuffer<T> {
    type Target = [T];

    fn try_deref(&self) -> Result<&Self::Target, String> {
        if self.head > self.tail {
            return Err("not contiguous".to_string());
        }
        Ok(&self.data[self.head..self.tail])
    }
}

impl<T: Default> DerefMut for CircularBuffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.make_contiguous();
        if self.head > self.tail {
            panic!("not contiguous!!!")
        }
        &mut self.data[self.head..self.tail]
    }
}

//------------------------------------------------
// Sincronizzazione del CircularBuffer
//
// Il Mutex garantisce che solo un thread alla 
// volta possa accedere alla risorsa incapsulata 
// evitando race condition
//------------------------------------------------
use std::sync::Mutex;

pub struct SyncBuffer<T> {
    buf: Mutex<CircularBuffer<T>>,
}

impl<T: Default> SyncBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        SyncBuffer {    
            // Inizializziamo il buffer con una mutex contenente la chiamata alla funzione new di CircularBuffer
            buf: Mutex::new(CircularBuffer::new(capacity)),
        }
    }

    // Utilizza &self perchè è un borrow condivisibile, quindi il metodo
    // è chiamabile simultanemente ed è gestito in sincronizzazione.
    // Se avessi usato &mut selfs avrei reso il metodo esclusivo
    pub fn write(&self, element: T) -> Result<(), circbuf::Error<T>> {
        // Acquisisco il lock sul Mutex
        let mut buf_guard = self.buf.lock().unwrap(); 
        buf_guard.write(element)  // Scriviamo nel buffer usando il metodo originale di CircularBuffer
    }

    pub fn read(&self) -> Result<T, circbuf::Error<T>> {
        // Acquisisco il lock sul Mutex
        let mut buf_guard = self.buf.lock().unwrap();  
        buf_guard.read()  // Leggiamo dal buffer usando il metodo originale di CircularBuffer
    }
}

//------------------------------------------------
// Sincronizzazione del CircularBuffer + CondVar
//
// Le condition variable consentono di ridurre al
// minimo l'uso della CPU mentre i thread attendono
//------------------------------------------------
use std::sync::{Arc, Condvar, Mutex};

struct BlockingSyncBuf<T> {
    buf: Mutex<CircularBuffer<T>>,
    cv: Condvar,
}

impl<T: Default> BlockingSyncBuf<T> {
    pub fn new(capacity: usize) -> Self {
        BlockingSyncBuf {
            buf: Mutex::new(CircularBuffer::new(capacity)),
            cv: Condvar::new(),
        }
    }

    pub fn write_blocking(&self, mut element: T) {
        let mut buf_guard = self.buf.lock().unwrap();
        loop {
            match buf_guard.write(element) {
                Ok(_) => {
                    self.cv.notify_one();  // Notifica i thread in attesa che possono ora leggere
                    return;
                }
                Err(circbuf::Error::FullBuffer(el)) => {
                    element = el;  // Questo thread si salva l'elemento
                    buf_guard = self.cv.wait(buf_guard).unwrap();  // Questo thread attende che ci sia spazio disponibile
                }
                Err(_) => {
                    panic!("Unexpected error");
                }
            }
        }
    }

    pub fn read_blocking(&self) -> T {
        let mut buf_guard = self.buf.lock().unwrap();
        loop {
            match buf_guard.read() {
                Ok(element) => {
                    self.cv.notify_one();  // Notifica i thread in attesa che possono ora scrivere
                    return element;
                }
                Err(circbuf::Error::EmptyBuffer) => {
                    buf_guard = self.cv.wait(buf_guard).unwrap();  // Questo thread attende che ci siano dati disponibili
                }
                Err(_) => {
                    panic!("Unexpected error");
                }
            }
        }
    }
}

//------------------------------------------------
// Producer e Consumer
//------------------------------------------------
pub fn test_producer_consumer() {
    let buf = Arc::new(BlockingSyncBuf::new(10));  // Creiamo un buffer condiviso con capacità 10
    let buf1 = buf.clone();  // Cloniamo l'Arc per passarlo al consumer

    // Creiamo il thread del consumer
    let consumer = thread::spawn(move || {
        loop {
            let el = buf1.read_blocking();  // Il consumer legge dal buffer
            println!("read: {}", el);
            thread::sleep(std::time::Duration::from_secs(2));  // Il consumer legge ogni 2 secondi
        }
    });

    // Creiamo il thread del producer
    let producer = thread::spawn(move || {
        let mut count = 0;
        loop {
            count += 1;
            buf.write_blocking(count);  // Il producer scrive nel buffer
            println!("wrote: {}", count);
            thread::sleep(std::time::Duration::from_secs(1));  // Il producer scrive ogni 1 secondo
        }
    });

    // Aspettiamo che entrambi i thread terminino (in un'applicazione reale potremmo non volerli far terminare mai)
    producer.join().unwrap();
    consumer.join().unwrap();
}

