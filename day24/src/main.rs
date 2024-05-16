mod intersections;
mod line;
mod particle;

use intersections::find_particle_intersections;
use particle::parse_particles;
use std::fs::read_to_string;

fn main() {
    solve_part_1()
}

fn solve_part_1() {
    let text = read_to_string("input.txt").unwrap();

    let particles = parse_particles(&text);

    let test_area = (
        (200000000000000.0, 400000000000000.0),
        (200000000000000.0, 400000000000000.0),
    );

    let intersections = find_particle_intersections(&particles, test_area);
    assert_eq!(intersections, 21785);
}
