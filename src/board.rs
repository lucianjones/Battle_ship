extern crate rand;

use std::char;
use rand::Rng;
use super::ship;

// set hit, miss, water chars, determine size of board
pub const MISS: char = 'o';
pub const HIT: char = 'x';
pub const WATER: char = '~';
pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 10;

// define struct board, contains selfspcs (your board with ships) and enemyspcs (enemy board that
// only shows your moves), both are 2d array with height * [width], and a [] of ships
pub struct Board {
    pub name: String,
    pub self_spaces: [[char; WIDTH]; HEIGHT],
    pub enemy_spaces: [[char; WIDTH]; HEIGHT],
    pub ships: [ship::Ship; 5],
} 

impl Board {
    // initializes board
    pub fn init(&mut self) {
        let mut skip: bool;
        let mut dir: bool;
        let mut bound: i8;
        let mut start: i8;
        let mut rng = rand::thread_rng();

        // iter through ship arr in board
        for ship in self.ships.iter_mut() {
            loop {
                //generate rand start point on board
                let w = rng.gen_range(0..WIDTH - 1);
                let h = rng.gen_range(0..HEIGHT - 1);

                // check if start point is already occupied
                if self.self_spaces[h as usize][w as usize] != WATER {
                    continue
                }

                // create rand bool
                dir = rng.gen();

                // use random bool to choose if vert or hor
                if dir {
                    bound = HEIGHT as i8;
                    start = h as i8;
                } else {
                    bound = WIDTH as i8;
                    start = w as i8;
                }

                // checks if ship will go out of bounds
                if start + ship.n_spaces >= bound {
                    continue
                }

                // checks spaces ship would occupy to make sure they are free
                skip = false;
                for i in start..ship.n_spaces + start {
                    let val = if dir {
                        self.self_spaces[i as usize][start as usize]
                    } else {
                        self.self_spaces[start as usize][i as usize]
                    };

                    if val != WATER {
                        skip = true;
                        break
                    }
                }

                if skip { continue }

                // once all checks passed, change grid vals to ships vals
                for i in start..ship.n_spaces + start {
                    if dir {
                        self.self_spaces[i as usize][start as usize] = ship.get_label();
                    } else {
                        self.self_spaces[start as usize][i as usize] = ship.get_label();
                    }
                }

                break
            }
        }
    }

    // checks that move hasnt already been used 
    pub fn is_valid(&self, row: i8, col: i8) -> bool {
        self.self_spaces[row as usize][col as usize] != HIT &&
        self.self_spaces[row as usize][col as usize] != MISS
    }
    
    // checks if move is a hit
    pub fn is_hit(&self, row: i8, col: i8) -> bool {
        self.self_spaces[row as usize][col as usize] != WATER
    }


    // checks if win has occured, boats take up 17 spcs total so if 17 hit occur -> win
    pub fn is_win(&self) -> bool {
        let mut hits = 0;
        for i in &self.enemy_spaces {
            for j in i {
                if j == &'x' {
                    hits += 1;
                }
            }
        }

        hits == 17
    }

    // function that prints board by iterating through each row array an printing each val as well
    // as y coordinate, the iterates over x len and prints x coordinates
    pub fn print_board(&self, is_self: bool) {
        let spaces = if is_self { self.self_spaces } else { self.enemy_spaces };
        let mut i = 65;

        for row in &spaces {
            print!(" {0} ", unsafe { char::from_u32_unchecked(i) }); // converts number to char, no check as all nums going in are valid
            i += 1;
            for j in row {
                print!(" {0} ", j);
            }
            println!()
        }
        
        print!("  ");
        for j in 0..WIDTH {
            print!(" {0} ", j);
        }
        println!();
    }
}

// sets up board struct: fills arrays with water, creates the ships that will be placed on board
pub fn board_factory(name: String) -> Board {
    return Board {
        name: name.to_string(),
        self_spaces: [[WATER; WIDTH]; HEIGHT],
        enemy_spaces: [[WATER; WIDTH]; HEIGHT],
        ships: [
            ship::ship_factory("aircraft".to_string(), 5),
            ship::ship_factory("battleship".to_string(), 4),
            ship::ship_factory("destroyer".to_string(), 3),
            ship::ship_factory("submarine".to_string(), 3),
            ship::ship_factory("patrol".to_string(), 2),
        ]
    };
}
                
