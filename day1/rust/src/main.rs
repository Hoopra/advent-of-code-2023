mod d1p1;
mod d1p2;

use d1p1::solve_day_1_part_1;
use d1p2::solve_day_1_part_2;

fn main() {
    let file_path = "../input_d1p1.txt";

    solve_day_1_part_1(file_path);
    solve_day_1_part_2(file_path);
}
