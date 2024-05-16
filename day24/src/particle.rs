pub type Vector3D = (f32, f32, f32);

#[derive(Debug)]
pub struct Particle {
    pub velocity: Vector3D,
    pub position: Vector3D,
}

impl Particle {
    pub fn from_text(text: &str) -> Self {
        let components: Vec<&str> = text.split(" @ ").collect();

        Self {
            position: vector_3d_from_text(components.get(0).unwrap()),
            velocity: vector_3d_from_text(components.get(1).unwrap()),
        }
    }
}

fn vector_3d_from_text(text: &str) -> Vector3D {
    let components: Vec<String> = text.split(", ").map(|value| value.to_string()).collect();
    (
        components.get(0).unwrap().parse().unwrap(),
        components.get(1).unwrap().parse().unwrap(),
        components.get(2).unwrap().parse().unwrap(),
    )
}

pub fn parse_particles(text: &str) -> Vec<Particle> {
    text.lines().map(|line| Particle::from_text(line)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_particle_from_text() {
        //
        let text = "19, 13, 30 @ -2, 1, -2";
        let particle = Particle::from_text(text);

        assert_eq!(particle.position, (19.0, 13.0, 30.0));
        assert_eq!(particle.velocity, (-2.0, 1.0, -2.0));
    }
}
