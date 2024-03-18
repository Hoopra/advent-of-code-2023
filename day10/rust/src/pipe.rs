pub type PipeGrid = Vec<Vec<Option<Pipe>>>;

pub type Position = (usize, usize);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PipePart {
    Unknown,
    Inside,
    Outside,
    Connection,
}

impl PipePart {
    pub fn default(connection: bool) -> PipePart {
        match connection {
            true => PipePart::Connection,
            _ => PipePart::Unknown,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pipe {
    pub left: PipePart,
    pub right: PipePart,
    pub up: PipePart,
    pub down: PipePart,
}

pub struct SurroundingTiles<'a> {
    pub left: &'a Option<Pipe>,
    pub right: &'a Option<Pipe>,
    pub up: &'a Option<Pipe>,
    pub down: &'a Option<Pipe>,
}

impl Pipe {
    pub fn new(left: PipePart, right: PipePart, up: PipePart, down: PipePart) -> Pipe {
        Pipe {
            left,
            right,
            up,
            down,
        }
    }

    pub fn from_connections(left: bool, right: bool, up: bool, down: bool) -> Pipe {
        Pipe {
            left: PipePart::default(left),
            right: PipePart::default(right),
            up: PipePart::default(up),
            down: PipePart::default(down),
        }
    }
}
