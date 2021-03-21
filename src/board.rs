extern crate rand;

use rand::Rng;
use super::ship;

pub const MISS: char = 'o';
pub const HIT: char = 'x';
pub const WATER: char = '~';
pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 10;


pub struct Board {
    pub name: String,
    pub self_spaces: [[char; WIDTH]; HEIGHT],
    pub enemy_spaces: [[char; WIDTH]; HEIGHT],
    pub ships: [ship::Ship; 5],
} 

impl Board {
    pub fn init(&mut self) {
        let mut skip: bool;
        let mut dir: bool;
        let mut bound: i8;
        let mut start: i8;
        let mut rng = rand::thread_rng();

        for ship in self.ships.iter_mut() {
            loop {
                let w = rng.gen_range(0..WIDTH - 1);
                let h = rng.gen_range(0..HEIGHT - 1);

                if self.self_spaces[h as usize][w as usize] != WATER {
                    continue
                }

                dir = rng.gen();

                if dir {
                    bound = HEIGHT as i8;
                    start = h as i8;
                } else {
                    bound = WIDTH as i8;
                    start = w as i8;
                }

                if start + ship.n_spaces >= bound {
                    continue
                }

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

    pub fn is_valid(&self, row: i8, col: i8) -> bool {
        self.self_spaces[row as usize][col as usize] != HIT ||
        self.self_spaces[row as usize][col as usize] != MISS
    }
    
    pub fn is_hit(&self, row: i8, col: i8) -> bool {
        self.self_spaces[row as usize][col as usize] != WATER
    }


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

    pub fn is_human(&self) -> bool {
        self.name == "Human"
    }

    pub fn print_board(&self, is_self: bool) {
        let spaces = if is_self { self.self_spaces } else { self.enemy_spaces };
        let mut i = 0;

        for row in &spaces {
            print!(" {0} ", i);
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

impl PartialEq for Board {
    fn eq(&self, other: &Board) -> bool {
        self.name == other.name
    }
}

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
                
