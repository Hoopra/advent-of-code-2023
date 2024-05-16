use crate::{line::find_particle_intersection_2d, particle::Particle};

type TestArea = ((f32, f32), (f32, f32));

pub fn find_particle_intersections(particles: &Vec<Particle>, bounding_area: TestArea) -> u32 {
    let mut intersections = 0;
    let (bounds_x, bounds_y) = bounding_area;

    for a in 0..particles.len() {
        for b in a..particles.len() {
            let particle_a = particles.get(a).unwrap();
            let particle_b = particles.get(b).unwrap();

            let (x_intersect, y_intersect, t1, t2) =
                find_particle_intersection_2d(particle_a, particle_b);

            let x_within_bounds = is_within_bounds(x_intersect, bounds_x);
            let y_within_bounds = is_within_bounds(y_intersect, bounds_y);

            if x_within_bounds && y_within_bounds && t1 > 0.0 && t2 > 0.0 {
                intersections += 1;
            }
        }
    }

    intersections
}

fn is_within_bounds(value: f32, bounds: (f32, f32)) -> bool {
    let (min, max) = bounds;

    value >= min && value <= max
}

#[cfg(test)]
mod tests {
    use crate::particle::parse_particles;

    use super::*;

    #[test]
    fn finds_number_of_particle_intersections() {
        let text = "19, 13, 30 @ -2, 1, -2\n18, 19, 22 @ -1, -1, -2\n20, 25, 34 @ -2, -2, -4\n12, 31, 28 @ -1, -2, -1\n20, 19, 15 @ 1, -5, -3";

        let particles = parse_particles(text);

        let test_area = ((7.0, 27.0), (7.0, 27.0));
        let result = find_particle_intersections(&particles, test_area);

        assert_eq!(result, 2);
    }
}
