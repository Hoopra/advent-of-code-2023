use super::Feature;

pub struct TerrainCollection {
    features: Vec<Feature>,
}

impl TerrainCollection {
    pub fn new(features: Vec<Feature>) -> TerrainCollection {
        TerrainCollection { features }
    }
}

impl TerrainCollection {
    pub fn get_features(&self) -> &Vec<Feature> {
        &self.features
    }
}

impl PartialEq for TerrainCollection {
    fn eq(&self, other: &Self) -> bool {
        self.get_features()
            .iter()
            .zip(other.get_features())
            .all(|(a, b)| a == b)
    }
}
