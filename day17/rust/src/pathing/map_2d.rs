pub type Position = (usize, usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
    N,
    E,
    S,
    W,
}
