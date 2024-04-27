use super::{MapFeature, Position, TravelDirection};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Beam {
    pub travel_direction: TravelDirection,
    pub position: Option<Position>,
}

impl Beam {
    pub fn default(position: Position) -> Beam {
        Beam {
            travel_direction: TravelDirection::E,
            position: Some(position),
        }
    }

    pub fn new(position: Position, travel_direction: TravelDirection) -> Beam {
        Beam {
            travel_direction,
            position: Some(position),
        }
    }
}

impl Beam {
    pub fn update_position(&mut self) -> Option<Position> {
        let position = self.next_position();

        self.position = position;
        position
    }

    pub fn update_direction(&mut self, feature: &MapFeature) {
        self.travel_direction = self.deflect(feature);
    }

    pub fn next_position(&self) -> Option<Position> {
        let (x, y) = self.position.unwrap();

        match self.travel_direction {
            TravelDirection::E => Some((x + 1, y)),
            TravelDirection::W => match x > 0 {
                true => Some((x - 1, y)),
                _ => None,
            },
            TravelDirection::S => Some((x, y + 1)),
            TravelDirection::N => match y > 0 {
                true => Some((x, y - 1)),
                _ => None,
            },
        }
    }

    pub fn deflect(&self, feature: &MapFeature) -> TravelDirection {
        match feature {
            MapFeature::MirrorSW => match self.travel_direction {
                TravelDirection::E => TravelDirection::N,
                TravelDirection::W => TravelDirection::S,
                TravelDirection::S => TravelDirection::W,
                TravelDirection::N => TravelDirection::E,
            },
            MapFeature::MirrorSE => match self.travel_direction {
                TravelDirection::E => TravelDirection::S,
                TravelDirection::W => TravelDirection::N,
                TravelDirection::S => TravelDirection::E,
                TravelDirection::N => TravelDirection::W,
            },
            _ => self.travel_direction,
        }
    }

    pub fn split(mut self, feature: &MapFeature) -> Vec<Beam> {
        let position = self.position.unwrap();

        match feature {
            MapFeature::SplitterNS => match self.travel_direction {
                TravelDirection::E | TravelDirection::W => {
                    let mut beam_n = Beam::new(position.clone(), TravelDirection::N);
                    let mut beam_s = Beam::new(position.clone(), TravelDirection::S);

                    beam_n.update_position();
                    beam_s.update_position();

                    vec![beam_n, beam_s]
                }
                _ => {
                    self.update_position();
                    vec![self]
                }
            },
            MapFeature::SplitterWE => match self.travel_direction {
                TravelDirection::N | TravelDirection::S => {
                    let mut beam_w = Beam::new(position.clone(), TravelDirection::W);
                    let mut beam_e = Beam::new(position.clone(), TravelDirection::E);

                    beam_w.update_position();
                    beam_e.update_position();

                    vec![beam_w, beam_e]
                }
                _ => {
                    self.update_position();
                    vec![self]
                }
            },
            _ => vec![self],
        }
    }

    pub fn get_key(&self) -> String {
        let (x, y) = self.position.unwrap();

        let direction_key = match self.travel_direction {
            TravelDirection::E => "E",
            TravelDirection::W => "W",
            TravelDirection::N => "N",
            TravelDirection::S => "S",
        };

        format!("({},{})|{}", x, y, direction_key)
    }
}
