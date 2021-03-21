mod user_input;
mod board;
mod ship;

use user_input::get_x;
use user_input::get_y;

fn main() {
    let mut comp_board = board::board_factory("comp board".to_string());
    comp_board.init();
    let mut player_board = board::board_factory("player board".to_string());
    player_board.init();
    player_board.print_board(false);
    game_cycle(comp_board, player_board);
}


fn game_cycle(mut cboard: board::Board, mut pboard: board::Board) {
    loop {
        println!("What is your x coordinate? (0-9)");
        let x: i8 = get_x();

        println!("What is your y coordinate? (0-9)");
        let y: i8 = get_y();

        if !cboard.is_valid(y, x) {
            println!("You've already gone here");
            continue
        }

        if cboard.is_hit(y, x) {
            println!("Congrats, you got a hit!");
            pboard.enemy_spaces[y as usize][x as usize] = board::HIT;
            cboard.self_spaces[y as usize][x as usize] = board::HIT;
        } else {
            println!("Sorry, you got a missed!");
            pboard.enemy_spaces[y as usize][x as usize] = board::MISS;
            cboard.self_spaces[y as usize][x as usize] = board::MISS;
        }

        pboard.print_board(false);
        if pboard.is_win() {
            println!("You Won!");
            return
        } else { continue }
    }
}

