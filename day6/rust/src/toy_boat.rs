pub struct ToyBoat {
    acceleration: u32,
}

impl ToyBoat {
    pub fn new(acceleration: Option<u32>) -> ToyBoat {
        ToyBoat {
            acceleration: acceleration.unwrap_or(1),
        }
    }
}

impl ToyBoat {
    fn calculate_distance_traveled(&self, charge_time: u32, race_time: u32) -> u32 {
        let velocity = charge_time * self.acceleration;
        velocity * (race_time - charge_time)
    }

    pub fn calculate_distances_traveled(&self, race_time: u32) -> Vec<u32> {
        // let velocity = charge_time * self.acceleration;
        // velocity * (race_time - charge_time)
        (0..race_time + 1)
            .collect::<Vec<u32>>()
            .iter()
            .enumerate()
            .map(|(charge_time, _)| self.calculate_distance_traveled(charge_time as u32, race_time))
            .collect()
    }
}

#[cfg(test)]
mod test_toy_boat {
    use super::*;

    #[test]
    fn calculates_distance_traveled() {
        let toy_boat = ToyBoat::new(None);

        assert_eq!(toy_boat.calculate_distance_traveled(5, 7), 10);
    }

    #[test]
    fn calculates_distances_traveled() {
        let toy_boat = ToyBoat::new(None);

        assert_eq!(
            toy_boat.calculate_distances_traveled(7),
            vec![0, 6, 10, 12, 12, 10, 6, 0]
        );
    }
}
