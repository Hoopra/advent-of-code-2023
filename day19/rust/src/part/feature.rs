#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PartFeature {
    ExtremelyCool,
    Musical,
    Aerodynamic,
    Shiny,
}

impl PartFeature {
    pub fn from_symbol(symbol: char) -> PartFeature {
        match symbol {
            'a' => PartFeature::Aerodynamic,
            'x' => PartFeature::ExtremelyCool,
            's' => PartFeature::Shiny,
            'm' | _ => PartFeature::Musical,
        }
    }
}
