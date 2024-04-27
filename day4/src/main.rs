mod card_scores;
mod card_winnings;
mod scratch_card;

use card_scores::get_all_card_scores;
use card_winnings::get_total_cards_won;

fn main() {
    let file_path = "../input.txt";
    let score = get_all_card_scores(file_path);

    println!("total score: {score}");
    assert_eq!(score, 21105);

    let cards_won = get_total_cards_won(file_path);

    println!("total cards won: {cards_won}");
    assert_eq!(cards_won, 5329815);
}
