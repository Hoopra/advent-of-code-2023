mod multiple_games;
mod single_game;
mod toy_boat;

use multiple_games::parse_multiple_games;
use single_game::parse_single_game;

fn main() {
    parse_multiple_games();
    parse_single_game();
}
