pub struct ToyBoat {
    acceleration: u64,
}

impl ToyBoat {
    pub fn new(acceleration: Option<u64>) -> ToyBoat {
        ToyBoat {
            acceleration: acceleration.unwrap_or(1),
        }
    }
}

impl ToyBoat {
    fn calculate_distance_traveled(&self, charge_time: u64, race_time: u64) -> u64 {
        let velocity = charge_time * self.acceleration;
        velocity * (race_time - charge_time)
    }

    pub fn calculate_distances_traveled(&self, race_time: u64) -> Vec<u64> {
        (0..race_time + 1)
            .collect::<Vec<u64>>()
            .iter()
            .enumerate()
            .map(|(charge_time, _)| self.calculate_distance_traveled(charge_time as u64, race_time))
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
