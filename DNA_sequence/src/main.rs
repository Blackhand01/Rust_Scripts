// find all subsequences of seq in s and return a vector of tuples containing the start position
// and the found subsequences as string slices
// ignore overlaps: if a subsequence is found, the search must continue from the next character
// missing lifetimes: the result string slices depend only from one input parameter, which one?

// suggestion: write a function find_sub(&str, &str) -> Option<(usize, &str)> that finds the first subsequence in a string, you can use it in all the following functions


// Trova la prima occorrenza della sottosequenza in una stringa
fn find_sub<'a>(dna_sequence_left: &'a str, subsequence_to_find: &'a str) -> Option<(usize, &'a str)> {
    if dna_sequence_left.is_empty() || subsequence_to_find.is_empty() {
        return None;
    }

    // Converte stringa in una collezione (vettore) di caratteri per fare il confronto crt per crt
    let subseq_chars: Vec<char> = subsequence_to_find.chars().collect();
    let mut subseq_idx = 0;

    for (current_pos, crt_to_compare) in dna_sequence_left.chars().enumerate() {
        if crt_to_compare == subseq_chars[subseq_idx] {
            subseq_idx += 1;
            if subseq_idx == subseq_chars.len() {
                let start_pos = current_pos + 1 - subseq_chars.len();
                return Some((start_pos, &dna_sequence_left[start_pos..=current_pos]));
            }
        }
    }
    None
}

// Trova tutte le occorrenze della sottosequenza in una stringa, senza sovrapposizioni
fn subsequences1<'a>(dna_sequence: &'a str, subsequence_to_find: &'a str) -> Vec<(usize, &'a str)> {
    let mut result: Vec<(usize, &'a str)> = Vec::new(); // Vettore per memorizzare i risultati
    let mut current_pos: usize = 0; // Posizione corrente nel dna_sequence

    // Ciclo per trovare tutte le sottosequenze
    while let Some((pos, found_subsequence)) = find_sub(&dna_sequence[current_pos..], subsequence_to_find) {
        current_pos += pos;
        result.push((current_pos, found_subsequence));
        current_pos +=  found_subsequence.len();
    }

    result 
}


pub fn demo1() {
    let dna_sequence = "AACGGTAACC".to_string();
    let subsequence_to_find = "A1-1,C2-4";

    for (offset, found_subsequence) in subsequences1(&dna_sequence, subsequence_to_find) {
        println!("Found subsequence at position {}: {}", offset, found_subsequence);
    }
}

// Now we want to find different subsequences at the same time, seq is a vector of string slices with many subsequence to search
// For each subsequence find all the matches and to the results (there may be overlaps, ignore them), but in this way you can reuse the previous solution
// The result will contain: the start position in s, the found subsequence as string slice and the mached subsequence in seq
// Now the string slices in the rsult depend from two input parameters, which ones?

fn subsequences2<'a>(dna_sequence: &'a str, subsequences_to_find: &'a [&'a str]) -> Vec<(usize, &'a str, &'a str)> {
    let mut result: Vec<(usize, &'a str, &'a str)> = Vec::new();

    // Uguale a prima, solo devo farlo per ogni sottosequenza in `subsequences_to_find`
    for &subseq in subsequences_to_find {
        let mut current_pos: usize = 0;

        // Utilizza `find_sub` per trovare tutte le occorrenze della sottosequenza
        while let Some((pos, found_subsequence)) = find_sub(&dna_sequence[current_pos..], subseq) {
            current_pos += pos;
            result.push((current_pos, found_subsequence, subseq));
            current_pos += found_subsequence.len();
        }
    }

    result
}


pub fn demo2() {
    let dna_sequence = "AACGGTAACC".to_string();
    let subsequences_to_find = ["A1-1,C2-4", "G1-1,T2-4"];

    for (offset, found_subsequence, subseq) in subsequences2(&dna_sequence, &subsequences_to_find) {
        println!("Found subsequence '{}' at position {}: '{}'", subseq, offset, found_subsequence);
    }
}

// Now we want to do some DNA editing! Therefore we receive a mutable string and we'd like to return a vector of mutable string slices
// Follow this steps:
// 1. adjust the lifetimes without any implementation yet: does it compile?
// 2. try to implement the function: does it compile?
// 3. if it doesn't compile, try to understand why from the compiler errors and draw all the necessary lifetimes
// 4. Spoiler: basically it's not possibile to return more then one mutable reference to the same data
// 5. Try this workaround: return a vector of indexes (first solution) and let the caller extract the mutable references
// 7. (later in the course you will learn about smart pointers, which can be used to solve this kind of problems in a more elegant way)



// Rust non permette di avere più di un riferimento mutabile alla stessa stringa (dna_sequence) alla volta. 
// Restituendo riferimenti mutabili a diverse parti della stringa, stai cercando di creare più di un riferimento mutabile contemporaneamente, il che viola le regole di Rust.

// Soluzione alternativa 1: Restituire gli Indici, il chiamante della funzione può poi usarli per ottenere i riferimenti mutabili
fn subsequences3<'a>(dna_sequence: &'a mut str, subsequence_to_find: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let mut current_pos = 0;

    while let Some(pos) = dna_sequence[current_pos..].find(subsequence_to_find) {
        current_pos += pos;
        positions.push(current_pos);
        current_pos += subsequence_to_find.len();
    }
    positions
}

pub fn demo3() {
    let mut dna_sequence = "AACGGTAACC".to_string();
    let subsequence_to_find = "A1-1,C2-4";

    for offset in subsequences3(&mut dna_sequence, subsequence_to_find) {
        let end_pos = offset + subsequence_to_find.len();
        let found_subsequence = &mut dna_sequence[offset..end_pos];
        println!("Found subsequence at position {}: {}", offset, found_subsequence);
    }
}


// DNA strings may be very long and we can get a lot of matches.
// Therefore we want to process a subsequence as soon as we find it, without storing it in a vector
// A solution is to pass a closure to the function, which will be called for each match
// do you need to put lifetime annotations in the closure? why?

// <F> significa che la funzione può accettare come argomento qualsiasi tipo che soddisfi il vincolo specificato in where
// FnMut è uno dei tre trait principali per le closure in Rust:

// Fn: Una closure che non muta il suo stato interno.
// FnMut: Una closure che può mutare il suo stato interno.
// FnOnce: Una closure che può essere chiamata solo una volta, poiché consuma se stessa (e quindi il suo stato) quando viene chiamata.
fn subsequence4<F>(dna_sequence: &str, subsequence_to_find: &str, mut process_match: F)
where 
    F: FnMut(usize, &str),
{
    let mut current_pos = 0;

    while let Some(pos) = dna_sequence[current_pos..].find(subsequence_to_find) {
        let start = current_pos + pos;
        let end = start + subsequence_to_find.len();
        process_match(start, &dna_sequence[start..end]);
        current_pos = end;
    }
}


pub fn demo4() {
    let a = "AACGGTAACC".to_string();
    let seq = "A1-1,C2-4";
    // la closure è come un array function 
    subsequence4(&a, seq, 
        |pos, sub| { println!("Found subsequence at position {}: {}", pos, sub); }
    );
}

// Now let's define a struct SimpleDNAIter (add the required lifetimes), memorizing a DNA sequence and the subsequence to search
// Then we add a next() method to the struct, which will return the next subsequence found in the DNA sequence after each call
// The result of next() is a tuple, but it's wrapped in an Option, because a call to next() may find no more subsequences in the DNA sequence
// In order to implement it, you may add any other attribute to the struct (remember: the struct is stateful and after each call to next() you must start from the last position found)
// The struct may be used as shown in the demo_SimpleDNAIter() function
// This approach is similar to the previous one, but it's more flexible and it can be used in more complex scenarios. For example you may interrupt it
// at any time and resume it later

struct SimpleDNAIter<'a> {
    dna_sequence: &'a str,
    subsequence_to_find: &'a str,
    current_pos: usize,
}

impl<'a> SimpleDNAIter<'a> {
    pub fn new(s: &'a str, seq: &'a str) -> Self {
        SimpleDNAIter { 
            dna_sequence: s, 
            subsequence_to_find: seq, 
            current_pos : 0
        }
    }

    // Metodo next per trovare la prossima sottosequenza
    pub fn next(&mut self) -> Option<(usize, &'a str)> {
        if let Some(pos) = self.dna_sequence[self.current_pos..].find(self.subsequence_to_find) {
            let start = self.current_pos + pos;
            let end = start + self.subsequence_to_find.len();
            self.current_pos = end;
            Some((start, &self.dna_sequence[start..end]))
        }
        else {
            None
        }
    }
}


fn demo_SimpleDNAIter() {
    let mut dna_iter = SimpleDNAIter::new("ACGTACGTACGTACGT", "AC");

    while let Some((pos, subseq)) = dna_iter.next() {
        println!("Found subsequence at position {}: {}", pos, subseq);
        // Possiamo interrompere il ciclo se abbiamo trovato ciò che cercavamo
    }
}

// finally we want to implement a real iterator, so that it can be used in a for loop and it may be combined we all the most common iterator methods
// The struct DNAIter is already defined, you have to implement the Iterator trait for it and add lifetimes
struct DNAIter<'a> {
    s: &'a str,
    seq: &'a str,
    current_pos: usize,  
}

impl<'a> DNAIter<'a> {
    pub fn new(s: &'a str, seq: &'a str) -> DNAIter<'a> {
        DNAIter {
            s: s,
            seq: seq,
            current_pos: 0,  
        }
    }
}


impl<'a> Iterator for DNAIter<'a> {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pos) = self.s[self.current_pos..].find(self.seq) {
            let start = self.current_pos + pos;
            let end = start + self.seq.len();
            self.current_pos = end; // Aggiorna la posizione corrente per la prossima chiamata a `next`
            Some((start, &self.s[start..end]))
        } else {
            None // Nessun'altra sottosequenza trovata
        }
    }
}


fn demo_dna_iter() {
    let dna_iter = DNAIter::new("ACGTACGTAAACCCGTACGT", "A1-3,C1-2");

    // now you can combine it with all the iterator modifiers!!!
    dna_iter
        .filter(|(pos, sub)| sub.len() >= 5)
        .for_each(|(pos, sub)| {
            println!(
                "Found subsequence at least long 5 at position {}: {}",
                pos, sub
            )
        });
}

// now let's return an iterator without defining a struct, just using a closure
// the std lib of rust support you with the std::from_fn() function
// we supply a skeleton implementation, you have to fill the closure
fn subsequence5_iter<'a>(s: &'a str, seq: &'a str) -> impl Iterator<Item = (usize, &'a str)> {
    let mut pos = 0;
    std::iter::from_fn(move || {
        if let Some(found_pos) = s[pos..].find(seq) {
            let start = pos + found_pos;
            let end = start + seq.len();
            pos = end; // Aggiorna la posizione corrente
            Some((start, &s[start..end]))
        } else {
            None // Nessun'altra sottosequenza trovata
        }
    })
}


fn demo_dna_iter2() {
    subsequence5_iter("ACGTACGTAAACCGTACGT", "ACGT")
        .filter(|(_pos, sub)| sub.len() >= 5)
        .for_each(|(pos, sub)| {
            println!(
                "Found subsequence at least long 5 at position {}: {}",
                pos, sub
            )
        });
}


fn main() {
    // demo1();
    // demo2();
    // demo3();
    // demo4();
    // demo_SimpleDNAIter();
    // demo_dna_iter();
    demo_dna_iter2();
}
