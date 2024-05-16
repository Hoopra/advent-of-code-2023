use crate::particle::Particle;

type Line2D = (f32, f32);

// (x, y, t1, t2)
pub fn find_particle_intersection_2d(p1: &Particle, p2: &Particle) -> (f32, f32, f32, f32) {
    let l1 = find_line_equation_2d(p1);
    let l2 = find_line_equation_2d(p2);

    let (x, y) = find_line_intersection_2d(l1, l2);

    let t1 = (x - p1.position.0) / p1.velocity.0;
    let t2 = (x - p2.position.0) / p2.velocity.0;

    (x, y, t1, t2)
}

fn find_line_equation_2d(particle: &Particle) -> Line2D {
    let Particle { velocity, position } = particle;

    let (x, y, _) = position;
    let (vx, vy, _) = velocity;

    let (x1, y1) = (x, y);

    let slope = vy / vx;
    let y_intersect = -(slope * x1 - y1);

    (slope, y_intersect)
}

fn find_line_intersection_2d(l1: Line2D, l2: Line2D) -> (f32, f32) {
    let (a1, b1) = l1;
    let (a2, b2) = l2;

    let intersect_x = (b2 - b1) / (a1 - a2);
    let intersect_y = a1 * (intersect_x) + b1;

    (intersect_x, intersect_y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_line_equation_2d() {
        let particle = Particle::from_text("19, 13, 30 @ -2, 1, -2");

        let (a, b) = find_line_equation_2d(&particle);

        assert_eq!(a, -0.5);
        assert_eq!(b, 22.5);
    }

    #[test]
    fn finds_particle_intersection_2d() {
        let particle1 = Particle::from_text("19, 13, 30 @ -2, 1, -2");
        let particle2 = Particle::from_text("18, 19, 22 @ -1, -1, -2");

        let (x, y, t1, t2) = find_particle_intersection_2d(&particle1, &particle2);

        assert_eq!((x, y), (14.333333, 15.333334));
        assert!(t1 > 0.0);
        assert!(t2 > 0.0);

        let particle1 = Particle::from_text("19, 13, 30 @ -2, 1, -2");
        let particle2 = Particle::from_text("20, 19, 15 @ 1, -5, -3");

        let (x, y, t1, t2) = find_particle_intersection_2d(&particle1, &particle2);

        assert_eq!((x, y), (21.444445, 11.777778));
        assert!(t1 < 0.0);
        assert!(t2 > 0.0);
    }
}
