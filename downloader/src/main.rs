use std::process::{Command, Stdio};  // Importiamo i moduli necessari per eseguire comandi di sistema
use std::io::{Read, BufReader};      // Importiamo i moduli per gestire input/output e lettura bufferizzata
use std::time::Duration;             // Importiamo il modulo per gestire la durata del tempo
use std::thread;                     // Importiamo il modulo per gestire i thread
use std::sync::{Arc, Mutex};         // Per sincronizzare l'accesso ai dati condivisi tra thread

// Definiamo un enum per rappresentare i possibili errori
enum DownloadError {
    Timeout,
    ProcessLaunchFailed(String),
    ReadFailed(String),
}

// Definiamo la struttura Downloader
struct Downloader {
    url: String,     // URL da scaricare
    timeout: u64,    // Timeout in secondi
}

// Implementiamo la funzione new per creare una nuova istanza di Downloader
impl Downloader {

    // Funzione di creazione di un nuovo Downloader
    fn new(url: &str, timeout: u64) -> Downloader {
        Downloader {
            url: url.to_string(),
            timeout,          
        }
    }
    
    // Processo principale crea un nuovo processo figlio che esegue il comando "curl" con l'URL specificato
    fn start(&self) -> Result<Vec<u8>, DownloadError> {
        // Avvia un processo figlio con il comando curl
        let mut child = match Command::new("curl")
            .arg(&self.url)                    
            .stdout(Stdio::piped())             // Redirige l'output su una pipe
            .spawn() {
                Ok(child) => child,              // Restituisce il processo figlio
                Err(e) => return Err(DownloadError::ProcessLaunchFailed(e.to_string())),
        };

        // Sincronizzazione del flag che controlla se il processo è stato completato
        let completed = Arc::new(Mutex::new(false));
        let completed_clone = Arc::clone(&completed);

        // Crea un thread per gestire il timeout e terminare il processo se necessario
        let timeout = self.timeout;
        let child_id = child.id(); 

        let timeout_thread = thread::spawn(move || {
            thread::sleep(Duration::from_secs(timeout));
            let completed = completed_clone.lock().unwrap();
            // Se il processo figlio è stato completato, non è necessario forzare la terminazione
            if !*completed {
                let _ = Command::new("kill")
                    .arg("-9")
                    .arg(child_id.to_string())
                    .status();
            }
        });

        // Legge l'output dal processo figlio
        let mut stdout = BufReader::new(child.stdout.take().unwrap());
        let mut data = Vec::new();              

        // Legge i dati dalla pipe fino a EOF
        match stdout.read_to_end(&mut data) {
            Ok(_) => {
                // Imposta il flag di completamento prima di attendere il thread di timeout
                let mut completed = completed.lock().unwrap();
                *completed = true;
                let _ = timeout_thread.join();
                Ok(data)
            }
            Err(e) => Err(DownloadError::ReadFailed(e.to_string())),
        }
    }
}

fn main() {
    // Creiamo un Downloader per scaricare una pagina web
    let url = "https://www.google.com";  // Puoi sostituire questa URL con un'altra per testare
    let timeout = 2;  // Timeout di 10 secondi

    let downloader = Downloader::new(url, timeout);

    // Avviamo il download
    match downloader.start() {
        Ok(data) => {
            println!("Download completed successfully! Data size: {} bytes", data.len());
            // Se vuoi stampare i dati, puoi farlo (ma attenzione, se sono molti potrebbero riempire la console)
            // println!("Data: {:?}", String::from_utf8_lossy(&data));
        }
        Err(e) => match e {
            DownloadError::Timeout => {
                println!("Download failed: Operation timed out.");
            }
            DownloadError::ProcessLaunchFailed(msg) => {
                println!("Download failed: Failed to start the process. Error: {}", msg);
            }
            DownloadError::ReadFailed(msg) => {
                println!("Download failed: Failed to read data from process. Error: {}", msg);
            }
        },
    }
}
