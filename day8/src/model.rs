#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Location {
    pub id: String,

    pub left_id: String,
    pub right_id: String,
}

impl Location {
    pub fn new(id: String, left_id: String, right_id: String) -> Location {
        Location {
            id,
            left_id,
            right_id,
        }
    }
}
