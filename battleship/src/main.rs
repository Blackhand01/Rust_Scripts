/**
 * Battleship game
 * The game is played on a 20x20 board
 * The player has to place 4 boats on the board
 * The game ends when all the boats are sunk
 * A boat can be represented as a vertical or horizontal line
 * 
 * Author: Stefano Roy Bisignano
 */

const BSIZE: usize = 20;

// Data structures
pub struct Board {  
    boats: [u8; 4],                    
    data: [[u8; BSIZE]; BSIZE],        

}

pub enum Boat {         // A boat can be represented as a vertical or horizontal line
    Vertical(usize),
    Horizontal(usize),
}

pub struct Pos{
    x: usize,
    y: usize,
}

pub enum Error {
    Overlap,      // Boat overlaps
    OutOfBounds,  // Boat out of bounds
    BoatCount,    // Number of boats exceeded
}


// Board structure implementation
impl Board {

    pub fn new(boats: &[u8]) -> Board {
        let mut new_board = Board {
            boats: [0; 4],
            data: [[0; BSIZE]; BSIZE],
        };

        let len = std::cmp::min(boats.len(), 4);
        new_board.boats[..len].copy_from_slice(&boats[..len]);

        new_board
    }   

    pub fn add_boat(&mut self, boat: Boat, pos: Pos) -> Result<(), Error> {
        // being self in the function is needed to create a new board with let mut 
        let new_board = self;

        // Switch case to check the type of boat (vertical or horizontal)
        match boat {
            
            // Vertical boat case
            Boat::Vertical(length) => {
                // add vertical boat with a given length at position pos  
                new_board.add_vertical_boat(length, pos)?;
            }

            // Horizontal boat case
            Boat::Horizontal(length) => {
                // add horizontal boat with a given length at position pos
                new_board.add_horizontal_boat(length, pos)?;
            }
        }

        Ok(())
    }

    /**
     * Helper functions:
     * 
     * is_occupied checks if a cell is occupied or not
     * place_boat places a boat on the board
     * add_vertical_boat adds a vertical boat to the board
     * add_horizontal_boat adds a horizontal boat to the board
     */

    fn is_occupied(&self, pos: Pos) -> bool {
        self.data[pos.x][pos.y] != 0
    }

    fn place_boat(&mut self, pos: Pos) -> () {
        self.data[pos.x][pos.y] = 1;
    }
    
    fn add_vertical_boat(&mut self, length: usize, pos: Pos) -> Result<(), Error> {
        
        if pos.x + length > BSIZE {
            return Err(Error::OutOfBounds);
        }

        for i in 0..length {
            if self.is_occupied(Pos {x: pos.x + i, y: pos.y }) {
                return Err(Error::Overlap);
            }
        }
    
        for i in 0..length {
            self.place_boat(Pos { x: pos.x + i, y: pos.y });
        }
    
        Ok(())
    }
    

    fn add_horizontal_boat(&mut self, length: usize, pos: Pos) -> Result<(), Error> {

        if pos.y + length > BSIZE {
            return Err(Error::OutOfBounds);
        }
    
        for i in 0..length {
            if self.is_occupied(Pos { x: pos.x, y: pos.y + i }) {
                return Err(Error::Overlap);
            }
        }
    
        for i in 0..length {
            self.place_boat(Pos { x: pos.x, y: pos.y + i });
        }

        Ok(())
    }


    // from string to board
    pub fn from(s: String) -> Board {
        let mut board = Board::new(&[]);
        
        for (i, line) in s.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == '1' {
                    board.data[i][j] = 1;
                }
            }
        }

        board
    }

    // from board to string
    pub fn to_string(&self) -> String {
        let mut s = String::new();

        for row in self.data.iter() {
            for cell in row.iter()   {
                s += &cell.to_string();
            }
            s += "\n";
        }

        s
    }
    
}





fn main() {
    // Boats to add to the board
    let boats = [4, 3, 2, 1];
    // Create a board
    let mut board = Board::new(&boats);
    // Initial positions of the boats
    let positions: Vec<(Boat, Pos)> = vec![
        (Boat::Vertical(4), Pos { x: 0, y: 0 }),
        (Boat::Horizontal(3), Pos { x: 18, y: 18 }),
    ];

    // Add the boats to the board
    for (boat, pos) in positions {
        let result= board.add_boat(boat, pos);

        match result {
            Ok(()) => {
                println!("{}", board.to_string());
            },
            Err(e) => match e {
                Error::Overlap => println!("Error: boat overlap."),
                Error::OutOfBounds => println!("Error: boat out of bounds."),
                Error::BoatCount => println!("Error: too many boats."),
            },
        }
    }

}