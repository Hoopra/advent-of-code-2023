use super::Position;

pub type PipeGrid = Vec<Vec<Option<Pipe>>>;

pub struct SurroundingTiles<'a> {
    pub left: &'a Option<Pipe>,
    pub right: &'a Option<Pipe>,
    pub up: &'a Option<Pipe>,
    pub down: &'a Option<Pipe>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Pipe {
    pub position: Position,
    pub symbol: char,
}

impl Pipe {
    pub fn new(position: Position, symbol: char) -> Pipe {
        Pipe { position, symbol }
    }

    pub fn from_connections(
        position: Position,
        left: bool,
        right: bool,
        up: bool,
        down: bool,
    ) -> Pipe {
        Pipe {
            position,
            symbol: pipe_type_from_connections(left, right, up, down),
        }
    }
}

impl Pipe {
    pub fn connects_left(&self) -> bool {
        match self.symbol {
            '-' | '7' | 'J' => true,
            _ => false,
        }
    }

    pub fn connects_right(&self) -> bool {
        match self.symbol {
            '-' | 'L' | 'F' => true,
            _ => false,
        }
    }

    pub fn connects_up(&self) -> bool {
        match self.symbol {
            '|' | 'L' | 'J' => true,
            _ => false,
        }
    }

    pub fn connects_down(&self) -> bool {
        match self.symbol {
            '|' | '7' | 'F' => true,
            _ => false,
        }
    }

    pub fn get_connectors(&self) -> Vec<Position> {
        let mut connectors: Vec<Position> = vec![];
        let (x, y) = self.position;

        if self.connects_left() {
            connectors.push((x - 1, y));
        }

        if self.connects_right() {
            connectors.push((x + 1, y));
        }

        if self.connects_up() {
            connectors.push((x, y - 1));
        }

        if self.connects_down() {
            connectors.push((x, y + 1));
        }

        connectors
    }
}

fn pipe_type_from_connections(left: bool, right: bool, up: bool, down: bool) -> char {
    if left && right {
        return '-';
    }

    if up && down {
        return '|';
    }

    if left && up {
        return 'J';
    }

    if left && down {
        return '7';
    }

    if right && up {
        return 'L';
    }

    'F'
}
