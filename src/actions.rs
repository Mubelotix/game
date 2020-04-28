use crate::{idx::*, map::*, units::*, previsualisation::*, *};

#[derive(PartialEq, Clone)]
pub enum Attack {
    StickKnock,
    VolleyOfArrows,
    OffensiveSwordFight,
    DefensiveSwordFight,
    Heal,
}

impl Attack {
    pub fn apply(&self, position: HexIndex, map: &mut Map, units: &mut Units) {
        if let Some(unit) = units.get_mut(&position) {
            unit.life.lose_life(1);
        }
    }

    pub fn get_description(&self) -> &'static str {
        match self {
            Attack::StickKnock => "Hit an adjacent unit (1 damage) and push it away.",
            Attack::VolleyOfArrows => "Shoot arrows in one direction. The first ennemy on that direction will be damaged (1 damage) and pushed away.",
            Attack::OffensiveSwordFight => "Attack adjacent unit using sword (2 damage) and pull it (1 damage for both units).",
            Attack::DefensiveSwordFight => "Attack adjacent unit using sword (2 damage) and push it away.",
            Attack::Heal => "Restore 1 LP. The healed unit will be restored at least to the third of the max LPs.",
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            Attack::StickKnock => "Stick Knock",
            Attack::VolleyOfArrows => "Volley of Arrows",
            Attack::OffensiveSwordFight => "Offensive Sword Fight",
            Attack::DefensiveSwordFight => "Defensive Sword Fight",
            Attack::Heal => "Heal",
        }
    }

    pub fn get_icon_idx(&self) -> usize {
        unimplemented!();
    }

    pub fn can_be_used_by_unit(&self, unit: &UnitType) -> bool {
        match self {
            Attack::StickKnock => true,
            Attack::Heal => true,
            Attack::VolleyOfArrows => match unit {
                UnitType::Archer => true,
                _ => false,
            },
            Attack::OffensiveSwordFight | Attack::DefensiveSwordFight => match unit {
                UnitType::Knight => true,
                _ => false,
            }
        }
    }

    #[allow(clippy::cognitive_complexity)]
    pub fn get_potential_targets(&self, _map: &Map, _units: &Units, position: &HexIndex) -> Vec<HexIndex> {
        match self {
            Attack::Heal => {
                let mut targets = Vec::new();
                if let Some(index) = position.get_top_left_neighbour() {
                    targets.push(index);
                }
                if let Some(index) = position.get_top_right_neighbour() {
                    targets.push(index);
                }
                if let Some(index) = position.get_right_neighbour() {
                    targets.push(index);
                }
                if let Some(index) = position.get_bottom_right_neighbour() {
                    targets.push(index);
                }
                if let Some(index) = position.get_bottom_left_neighbour() {
                    targets.push(index);
                }
                if let Some(index) = position.get_left_neighbour() {
                    targets.push(index);
                }
                targets.push(*position);
                targets
            }
            Attack::DefensiveSwordFight | Attack::OffensiveSwordFight | Attack::StickKnock => {
                let mut targets = Vec::new();
                if let Some(index) = position.get_top_left_neighbour() {
                    targets.push(index);
                }
                if let Some(index) = position.get_top_right_neighbour() {
                    targets.push(index);
                }
                if let Some(index) = position.get_right_neighbour() {
                    targets.push(index);
                }
                if let Some(index) = position.get_bottom_right_neighbour() {
                    targets.push(index);
                }
                if let Some(index) = position.get_bottom_left_neighbour() {
                    targets.push(index);
                }
                if let Some(index) = position.get_left_neighbour() {
                    targets.push(index);
                }
                targets
            }
            Attack::VolleyOfArrows => {
                let mut targets = Vec::new();
                if let Some(index) = position.get_top_left_neighbour() {
                    targets.push(index);
                    while let Some(index) = targets[targets.len() - 1].get_top_left_neighbour() {
                        targets.push(index);
                    }
                }
                if let Some(index) = position.get_top_right_neighbour() {
                    targets.push(index);
                    while let Some(index) = targets[targets.len() - 1].get_top_right_neighbour() {
                        targets.push(index);
                    }
                }
                if let Some(index) = position.get_right_neighbour() {
                    targets.push(index);
                    while let Some(index) = targets[targets.len() - 1].get_right_neighbour() {
                        targets.push(index);
                    }
                }
                if let Some(index) = position.get_bottom_right_neighbour() {
                    targets.push(index);
                    while let Some(index) = targets[targets.len() - 1].get_bottom_right_neighbour() {
                        targets.push(index);
                    }
                }
                if let Some(index) = position.get_bottom_left_neighbour() {
                    targets.push(index);
                    while let Some(index) = targets[targets.len() - 1].get_bottom_left_neighbour() {
                        targets.push(index);
                    }
                }
                if let Some(index) = position.get_left_neighbour() {
                    targets.push(index);
                    while let Some(index) = targets[targets.len() - 1].get_left_neighbour() {
                        targets.push(index);
                    }
                }
                targets
            }
        }        
    }

    pub fn get_consequences(&self, map: &Map, units: &[Option<Unit>; 61], position: &HexIndex, target: &HexIndex) -> Vec<(HexIndex, PrevisualisationItem)> {
        match self {
            Attack::StickKnock => {
                if Some(*target) == position.get_neighbour(&Direction::TopLeft) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::TopLeft))]
                }
                if Some(*target) == position.get_neighbour(&Direction::TopRight) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::TopRight))]
                }
                if Some(*target) == position.get_neighbour(&Direction::Right) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::Right))]
                }
                if Some(*target) == position.get_neighbour(&Direction::BottomRight) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::BottomRight))]
                }
                if Some(*target) == position.get_neighbour(&Direction::BottomLeft) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::BottomLeft))]
                }
                if Some(*target) == position.get_neighbour(&Direction::Left) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::Left))]
                }
                vec![]
            },
            Attack::DefensiveSwordFight => {
                if Some(*target) == position.get_neighbour(&Direction::TopLeft) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::TopLeft))]
                }
                if Some(*target) == position.get_neighbour(&Direction::TopRight) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::TopRight))]
                }
                if Some(*target) == position.get_neighbour(&Direction::Right) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::Right))]
                }
                if Some(*target) == position.get_neighbour(&Direction::BottomRight) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::BottomRight))]
                }
                if Some(*target) == position.get_neighbour(&Direction::BottomLeft) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::BottomLeft))]
                }
                if Some(*target) == position.get_neighbour(&Direction::Left) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::Left))]
                }
                vec![]
            },
            Attack::OffensiveSwordFight => {
                if Some(*target) == position.get_neighbour(&Direction::TopLeft) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::BottomRight))]
                }
                if Some(*target) == position.get_neighbour(&Direction::TopRight) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::BottomLeft))]
                }
                if Some(*target) == position.get_neighbour(&Direction::Right) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::Left))]
                }
                if Some(*target) == position.get_neighbour(&Direction::BottomRight) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::TopLeft))]
                }
                if Some(*target) == position.get_neighbour(&Direction::BottomLeft) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::TopRight))]
                }
                if Some(*target) == position.get_neighbour(&Direction::Left) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::Right))]
                }
                vec![]
            },
            Attack::Heal => {
                vec![]
            },
            Attack::VolleyOfArrows => {
                unimplemented!();
            }
        }
        
        
    }
}