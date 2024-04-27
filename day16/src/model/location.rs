pub type Position = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TravelDirection {
    N,
    E,
    S,
    W,
}

#[derive(Debug)]
pub enum MapFeature {
    Empty,
    SplitterWE,
    SplitterNS,
    MirrorSW,
    MirrorSE,
}
