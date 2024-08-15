
use std::time::SystemTime;

struct File {
    name: String,
    modified: SystemTime,
    content: Vec<u8>,
}

struct Dir {
    name: String,
    modified: SystemTime,
    children: Vec<Node>,
}

// Define this enum in order to be able to store different types in the same vector
enum Node {
    File(File),
    Dir(Dir),
}

#[derive(Debug)] // Aggiungi Debug per FSError
enum FSError {
    NotFound,     // file or dir not found
    NotADir,      // when trying to ad children to a file
    Duplicate,    // duplicate name in dir
    DirNotEmpty,  // try to remove a dir with children
    GenericError, // generic error
}

// define lifetimes
struct MatchResult{
    query: String,        // Matched query string
    path: String,     // Matched path
    name: String,     // Name of the node
}


struct Filesystem {
    root: Node,
}


fn find_child_by_name<'a>(dir: &'a Dir, name: &str) -> Option<&'a Node> {
    dir.children.iter().find(|child| match child {
        Node::Dir(d) => d.name == name,
        Node::File(f) => f.name == name,
    })
}

fn find_child_by_name_mut<'a>(dir: &'a mut Dir, name: &str) -> Option<&'a mut Node> {
    dir.children.iter_mut().find(|child| match child {
        Node::Dir(d) => d.name == name,
        Node::File(f) => f.name == name,
    })
}


impl Filesystem {
    // create a new empty filesystem with a root dir
    // (name of the root dir is empty string: "")
    pub fn new() -> Self {
        Filesystem{
            root: Node::Dir(
                Dir { 
                    name: "".to_string(),
                    modified: SystemTime::now(), 
                    children: Vec::new(),
                }
            )
        }
    }


    // create a new directory in the filesystem under the given path
    // return a reference the created dir
    // possible errors: NotFound, path NotADir, Duplicate

    pub fn mkdir(&mut self, path: &str, name: &str) -> Result<&mut Dir, FSError> {
        // Trova il nodo padre usando il percorso fornito
        let parent_node = self.get_mut(path)?;

        // Verifica che il nodo padre sia una directory
        match parent_node {
            Node::Dir(parent_dir) => {
                // Controlla se esiste già una directory o un file con lo stesso nome
                if parent_dir.children.iter().any(|child| match child {
                    Node::Dir(d) => d.name == name,
                    Node::File(f) => f.name == name,
                }) {
                    return Err(FSError::Duplicate);
                }

                // Crea una nuova directory
                let new_dir = Dir {
                    name: name.to_string(),
                    modified: std::time::SystemTime::now(),
                    children: Vec::new(),
                };

                // Aggiungi la nuova directory come figlio del nodo padre
                parent_dir.children.push(Node::Dir(new_dir));

                // Trova l'ultima directory aggiunta e restituiscila
                if let Some(Node::Dir(dir)) = parent_dir.children.last_mut() {
                    Ok(dir)
                } else {
                    Err(FSError::GenericError) // Errore generico se qualcosa va storto
                }
            }
            _ => Err(FSError::NotADir), // Errore se il nodo padre non è una directory
        }
    }
    

    // possible errors: NotFound, path is NotADir, Duplicate
    pub fn create_file(&mut self, path: &str, name: &str) -> Result<&mut File, FSError> {
        // Trova il nodo padre usando il percorso fornito
        let parent_node = self.get_mut(path)?;

        // Verifica che il nodo padre sia una directory
        match parent_node {
            Node::Dir(parent_dir) => {
                // Controlla se esiste già una directory o un file con lo stesso nome
                if parent_dir.children.iter().any(|child| match child {
                    Node::Dir(d) => d.name == name,
                    Node::File(f) => f.name == name,
                }) {
                    return Err(FSError::Duplicate);
                }

                // Crea un nuovo file
                let new_file = File {
                    name: name.to_string(),
                    modified: std::time::SystemTime::now(),
                    content: Vec::new(),
                };

                // Aggiungi il nuovo file come figlio del nodo padre
                parent_dir.children.push(Node::File(new_file));

                // Trova l'ultimo file aggiunto e restituiscilo
                if let Some(Node::File(file)) = parent_dir.children.last_mut() {
                    Ok(file)
                } else {
                    Err(FSError::GenericError) // Errore generico se qualcosa va storto
                }
            }
            _ => Err(FSError::NotADir), // Errore se il nodo padre non è una directory
        }
    }

    // updated modification time of the file or the dir
    // possible errors: NotFound
    pub fn touch(&mut self, path: &str) -> Result<(), FSError> {
        // Trova il nodo usando il percorso fornito
        let node = self.get_mut(path)?;

        // Aggiorna il tempo di modifica
        match node {
            Node::Dir(dir) => {
                dir.modified = std::time::SystemTime::now();
                Ok(())
            }
            Node::File(file) => {
                file.modified = std::time::SystemTime::now();
                Ok(())
            }
        }
    }

    // remove a node from the filesystem and return it
    // if it's a dir, it must be empty
    // possible errors: NotFound, DirNotEmpty
    pub fn delete(&mut self, path: &str) -> Result<Node, FSError> {
        // Dividi il percorso per ottenere il percorso del nodo padre e il nome del nodo da eliminare
        if let Some((parent_path, node_name)) = path.rsplit_once('/') {
            // Trova il nodo padre
            let parent_node = self.get_mut(parent_path)?;

            match parent_node {
                Node::Dir(parent_dir) => {
                    // Trova l'indice del nodo da eliminare
                    if let Some(index) = parent_dir.children.iter().position(|child| match child {
                        Node::Dir(d) => d.name == node_name,
                        Node::File(f) => f.name == node_name,
                    }) {
                        // Controlla se il nodo è una directory vuota
                        if let Node::Dir(ref dir) = parent_dir.children[index] {
                            if !dir.children.is_empty() {
                                return Err(FSError::DirNotEmpty);
                            }
                        }

                        // Rimuovi e restituisci il nodo
                        Ok(parent_dir.children.remove(index))
                    } else {
                        Err(FSError::NotFound)
                    }
                }
                _ => Err(FSError::NotADir),
            }
        } else {
            Err(FSError::NotFound)
        }
    }


    // get a reference to a node in the filesystem, given the path
    pub fn get(&mut self, path: &str) -> Result<&Node, FSError> {
        let node_path: Vec<&str> = path.split("/").filter( |s| !s.is_empty()).collect();
        let mut current_node: &Node = &self.root;

        for node in node_path{
            current_node = match current_node{
                Node::Dir (dir) => {
                    if let Some(child) = find_child_by_name(dir , node){
                        child
                    }
                    else {
                        return Err(FSError::NotFound);
                    }
                }
                _ => return Err(FSError::NotADir),
            };
        }

        Ok(current_node)
    }

    // get a mutable reference to a node in the filesystem, given the path
    pub fn get_mut(&mut self, path: &str) -> Result<&mut Node, FSError> {
        let nodes_path: Vec<&str> = path.split("/").filter(|s| !s.is_empty()).collect();
        let mut current_node: &mut Node = &mut self.root;

        for node in nodes_path{
            current_node = match current_node{
                Node::Dir (dir) => {
                    if let Some(child) = find_child_by_name_mut(dir, node) {
                        child // lo assegno a current node, così il for ciclerà su di esso
                    }
                    else{
                        return Err(FSError::NotFound); // non ho trovato una directory con nome corrispondente
                    }
                }
                
                _ => return Err(FSError::NotADir), // non è una directory
            };
        }
        Ok(current_node) // ho trovato la directory cercata
    }


    // search for a list of paths in the filesystem
    // qs is a list query strings with constraints
    // the constraints must be matched in or (it's returned any node matching at least one constraint)
    // constraint format: "type:pattern"
    // constraints:
    // - "type:dir" -> match only directories
    // - "type:file" -> match only files
    // - "name:value" -> match only nodes with the given name
    // - "partname:value" -> match only nodes with the given string in the name
    pub fn find(&self, qs: &[&str]) -> Vec<MatchResult> {
        let mut results = Vec::new();

        self.walk(|path, node| {
            for &q in qs {
                if let Some((key, value)) = q.split_once(':') {
                    if self.matches_query(node, key, value) {
                        let (name, node_type) = match node {
                            Node::Dir(d) => (d.name.clone(), "dir".to_string()),
                            Node::File(f) => (f.name.clone(), "file".to_string()),
                        };
                        
                        results.push(MatchResult {
                            query: q.to_string(),
                            path: path.to_string(),
                            name,
                        });
                        break;
                    }
                }
            }
        });

        results
    }



    fn matches_query(&self, node: &Node, key: &str, value: &str) -> bool {
        match (key, value, node) {
            ("type",     "dir",     Node::Dir(_)) => true,
            ("type",     "file",    Node::File(_)) => true,
            ("name",        _,      Node::Dir(d)) if d.name == value => true,
            ("name",        _,      Node::File(f)) if f.name == value => true,
            ("partname",    _,      Node::Dir(d)) if d.name.contains(value) => true,
            ("partname",    _,      Node::File(f)) if f.name.contains(value) => true,
            _ => false,
        }
    }
    
    


    // walk the filesystem, starting from the root, and call the closure for each node with its path
    // the first parameter of the closure is the path of the node, second is the node itself
    pub fn walk<F>(&self, mut f: F)
    where 
        F: FnMut(&str, &Node),  // Cambiamo `Fn` in `FnMut`
    {
        self.walk_node("", &self.root, &mut f);
    }

    fn walk_node<F>(&self, path: &str, node: &Node, f: &mut F)
    where 
        F: FnMut(&str, &Node),
    {
        f(path, node);  // Chiama la closure per il nodo corrente

        match node{
            Node::Dir(dir) => {
                for child in &dir.children { // Per ogni figlio nella directory
                    let child_path = format!("{}/{}", path, match child {   // Crea il percorso del figlio
                        Node::Dir(d) => &d.name,
                        Node::File(f) => &f.name,
                    });
                    self.walk_node(&child_path, child, f);
                }
            },
            _ => {} // Non fare nulla se non è una directory
        }
    }
}


fn main() {
    let mut fs = Filesystem::new();

    // Crea una struttura di directory, 10 dir ciascuna con una child dir e un file
    for i in 0..10 {
        fs.mkdir("/", format!("dir{}", i).as_str()).unwrap();
        fs.mkdir(format!("/dir{}", i).as_str(), "child1").unwrap();
        fs.create_file(format!("/dir{}", i).as_str(), "file1").unwrap();
    }

    println!("find /child2");

    // Usa get_mut per ottenere un riferimento mutabile
    if let Ok(res) = fs.get_mut("/dir2/child1") {
        if let Node::Dir(d) = res {
            d.name = "dir2 found".to_string();
        }
    } else {
        println!("not found");
    }

    // Cerca tutti i match
    let matches = fs.find(&["name:child1", "type:file"]);
    for m in matches {
        println!("Matched path: {}", m.path);
        // Poiché non possiamo accedere direttamente ai nodi, stampiamo solo il percorso
    }

    // Modifica il filesystem utilizzando i percorsi che non fanno parte dei risultati di MatchResult
    let paths = ["/dir1/child1", "/dir2/child1", "/dir3/child1"];
    for p in &paths {
        if let Ok(node) = fs.get_mut(p) {
            match node {
                Node::File(f) => {
                    println!("Inspecting file: {}", f.name);
                    // inspect content
                }
                Node::Dir(d) => {
                    println!("Inspecting directory: {}", d.name);
                    // inspect children
                }
            }
        }
    }

    // Cammina nel filesystem
    fs.walk(|path, node| {
        match node {
            Node::File(_) => {
                println!("file: {}", path);
            }
            Node::Dir(_) => {
                println!("dir: {}", path);
            }
        }
    });
}
