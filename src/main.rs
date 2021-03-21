extern crate rand;

mod user_input;
mod board;
mod ship;

use rand::Rng;
use user_input::get_x;
use user_input::get_y;

fn main() {
    // declares cpu board struct then initializes it
    let mut comp_board = board::board_factory("comp board".to_string());
    comp_board.init();                              
    // same as above but for player board
    let mut player_board = board::board_factory("player board".to_string());
    player_board.init();

    // prints cpu board without ships visible and user board with ship shown
    println!("-----------ENEMY BOARD----------");
    player_board.print_board(false);
    println!("------------YOUR BOARD----------");
    player_board.print_board(true);

    // begins game, all turns run inside function using a loop
    game_cycle(comp_board, player_board);

    println!("Thanks for playing!");
}


fn game_cycle(mut cboard: board::Board, mut pboard: board::Board) {
    loop {
        // get coordinates from user
        println!("What is your x coordinate? (0-9)");
        let x: i8 = get_x();

        println!("What is your y coordinate? (A-J)");
        let y: i8 = get_y();

        //check if move has already been playes
        if !cboard.is_valid(y, x) {
            println!("You've already gone here");
            continue
        }

        // checks if move is a hit and, updates board with hit char, else updates with miss char
        if cboard.is_hit(y, x) {
            println!("Congrats, you got a hit!");
            pboard.enemy_spaces[y as usize][x as usize] = board::HIT;
            cboard.self_spaces[y as usize][x as usize] = board::HIT;
        } else {
            println!("Sorry, you got missed!");
            pboard.enemy_spaces[y as usize][x as usize] = board::MISS;
            cboard.self_spaces[y as usize][x as usize] = board::MISS;
        }

        // after player move, if win prints final boards and exits function
        if pboard.is_win() {
            println!("-----------ENEMY BOARD----------");
            pboard.print_board(false);
            println!("------------YOUR BOARD----------");
            pboard.print_board(true);

            println!("You Won!");
            return
        }
        // begin cpu turn 
        let mut rng = rand::thread_rng();
        let mut x;
        let mut y;
        // generates random x, y loops until an unplayed move is found 
        loop {
            x = rng.gen_range(0..10) as i8;
            y = rng.gen_range(0..10) as i8;
            if pboard.is_valid(y, x) { break }
        }

        // same check as above on ln 47
        if pboard.is_hit(y, x) {
            println!("The enemy has hit us!");
            cboard.enemy_spaces[y as usize][x as usize] = board::HIT;
            pboard.self_spaces[y as usize][x as usize] = board::HIT;
        } else {
            println!("The enemy missed!");
            cboard.enemy_spaces[y as usize][x as usize] = board::MISS;
            pboard.self_spaces[y as usize][x as usize] = board::MISS;
        }
        
        // both turns have taken place so print boards
        println!("-----------ENEMY BOARD----------");
        pboard.print_board(false);
        println!("------------YOUR BOARD----------");
        pboard.print_board(true);

        // check if cpu won and exit function else start loop over aka end of turn
        if cboard.is_win() {
            println!("You lost!");
            return
        } else { continue }
        


    }
}

