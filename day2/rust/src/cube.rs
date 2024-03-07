use std::cmp::max;

#[derive(Debug)]
pub struct CubeDraw {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

impl CubeDraw {
    pub fn new(red: u16, green: u16, blue: u16) -> CubeDraw {
        CubeDraw { red, green, blue }
    }

    pub fn default() -> CubeDraw {
        CubeDraw {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl CubeDraw {
    pub fn combine_with(&self, other: CubeDraw) -> CubeDraw {
        CubeDraw {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }

    pub fn find_highest(&self, other: &CubeDraw) -> CubeDraw {
        CubeDraw {
            red: max(self.red, other.red),
            green: max(self.green, other.green),
            blue: max(self.blue, other.blue),
        }
    }
}

#[derive(Debug)]
pub struct CubeGame {
    pub id: u16,
    pub draws: Vec<CubeDraw>,
}

impl CubeGame {
    pub fn new(id: u16, draws: Vec<CubeDraw>) -> CubeGame {
        CubeGame { id, draws }
    }

    pub fn find_maximum_draws(&self) -> CubeDraw {
        self.draws
            .iter()
            .fold(CubeDraw::default(), |previous, next| {
                previous.find_highest(next)
            })
    }
}

pub type GameState = CubeDraw;

impl GameState {
    pub fn is_game_draw_possible(&self, draw: &CubeDraw) -> bool {
        self.red >= draw.red && self.green >= draw.green && self.blue >= draw.blue
    }
}

#[cfg(test)]
mod test_find_digit {
    use super::*;

    #[test]
    fn is_game_draw_possible_works() {
        let state = GameState::new(12, 13, 14);

        let draw = CubeDraw::new(14, 3, 15);
        assert_eq!(state.is_game_draw_possible(&draw), false);

        let draw = CubeDraw::new(4, 2, 6);
        assert_eq!(state.is_game_draw_possible(&draw), true);
    }
}
