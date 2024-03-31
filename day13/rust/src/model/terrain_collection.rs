use super::TerrainFeature;

pub struct TerrainCollection {
    features: Vec<TerrainFeature>,
}

impl TerrainCollection {
    pub fn new(features: Vec<TerrainFeature>) -> TerrainCollection {
        TerrainCollection { features }
    }
}

impl TerrainCollection {
    pub fn get_features(&self) -> &Vec<TerrainFeature> {
        &self.features
    }

    pub fn find_differences(&self, other: &Self) -> Vec<usize> {
        self.get_features()
            .iter()
            .enumerate()
            .zip(other.get_features())
            .filter_map(|((index, a), b)| match a != b {
                true => Some(index),
                false => None,
            })
            .collect()
    }
}

impl PartialEq for TerrainCollection {
    fn eq(&self, other: &Self) -> bool {
        self.find_differences(other).len() == 0
    }
}
