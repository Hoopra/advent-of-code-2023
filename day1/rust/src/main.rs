mod part1;
mod part2;

use part1::solve_part_1;
use part2::solve_part_2;

fn main() {
    let file_path = "../input.txt";

    solve_part_1(file_path);
    solve_part_2(file_path);
}
