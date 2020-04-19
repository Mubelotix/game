pub enum UnitType {
    Archer,
    Knight,
    Scout,
    Barbarian,
}

impl UnitType {
    pub fn get_texture_idx(&self) -> usize {
        match self {
            UnitType::Archer => 13,
            UnitType::Knight => 14,
            UnitType::Scout => 15,
            UnitType::Barbarian => 16,
        }
    }
}

pub struct Unit {
    pub unit_type: UnitType,
}

impl Unit {
    pub fn new(unit_type: UnitType) -> Unit {
        Unit {
            unit_type,
        }
    }
}