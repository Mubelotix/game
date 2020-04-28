use crate::{idx::*, map::*, previsualisation::*, units::*, *};

#[derive(PartialEq, Clone, Debug)]
pub enum Attack {
    StickKnock,
    VolleyOfArrows,
    OffensiveSwordFight,
    DefensiveSwordFight,
    Heal,
}

impl Attack {
    pub fn apply(&self, position: &HexIndex, target: &HexIndex, _map: &mut Map, units: &mut Units) {
        match self {
            Attack::VolleyOfArrows => {
                let mut final_target = None;

                for direction in Direction::iter() {
                    let mut targets = Vec::new();
                    let mut right_direction = false;
                    if let Some(index) = position.get_neighbour(&direction) {
                        if &index == target {
                            right_direction = true;
                        }
                        targets.push(index);
                        if units.get(&index).is_none() {
                            while let Some(index) =
                                targets[targets.len() - 1].get_neighbour(&direction)
                            {
                                if &index == target {
                                    right_direction = true;
                                }
                                targets.push(index);
                                if units.get(&index).is_some() {
                                    break;
                                }
                            }
                            if right_direction {
                                final_target = Some(targets[targets.len() - 1]);
                            }
                        }
                    }
                }

                if let Some(target) = final_target {
                    if let Some(unit) = units.get_mut(&target) {
                        unit.life.lose_life(1);
                    }
                }
            }
            t => {
                log!("unknown action: {:?}", t);
                if let Some(unit) = units.get_mut(target) {
                    unit.life.lose_life(1);
                }
            }
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

    pub fn _get_name(&self) -> &'static str {
        match self {
            Attack::StickKnock => "Stick Knock",
            Attack::VolleyOfArrows => "Volley of Arrows",
            Attack::OffensiveSwordFight => "Offensive Sword Fight",
            Attack::DefensiveSwordFight => "Defensive Sword Fight",
            Attack::Heal => "Heal",
        }
    }

    pub fn _get_icon_idx(&self) -> usize {
        unimplemented!();
    }

    pub fn _can_be_used_by_unit(&self, unit: &UnitType) -> bool {
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
            },
        }
    }

    #[allow(clippy::cognitive_complexity)]
    pub fn get_potential_targets(
        &self,
        _map: &Map,
        units: &Units,
        position: &HexIndex,
    ) -> Vec<HexIndex> {
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

                for direction in Direction::iter() {
                    if let Some(index) = position.get_neighbour(&direction) {
                        targets.push(index);
                        if units.get(&index).is_none() {
                            while let Some(index) =
                                targets[targets.len() - 1].get_neighbour(&direction)
                            {
                                targets.push(index);
                                if units.get(&index).is_some() {
                                    break;
                                }
                            }
                        }
                    }
                }

                targets
            }
        }
    }

    pub fn get_consequences(
        &self,
        _map: &Map,
        units: &[Option<Unit>; 61],
        position: &HexIndex,
        target: &HexIndex,
    ) -> Vec<(HexIndex, PrevisualisationItem)> {
        match self {
            Attack::StickKnock => {
                if Some(*target) == position.get_neighbour(&Direction::TopLeft) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::TopLeft))];
                }
                if Some(*target) == position.get_neighbour(&Direction::TopRight) {
                    return vec![(
                        *target,
                        PrevisualisationItem::PushArrow(Direction::TopRight),
                    )];
                }
                if Some(*target) == position.get_neighbour(&Direction::Right) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::Right))];
                }
                if Some(*target) == position.get_neighbour(&Direction::BottomRight) {
                    return vec![(
                        *target,
                        PrevisualisationItem::PushArrow(Direction::BottomRight),
                    )];
                }
                if Some(*target) == position.get_neighbour(&Direction::BottomLeft) {
                    return vec![(
                        *target,
                        PrevisualisationItem::PushArrow(Direction::BottomLeft),
                    )];
                }
                if Some(*target) == position.get_neighbour(&Direction::Left) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::Left))];
                }
                vec![]
            }
            Attack::DefensiveSwordFight => {
                if Some(*target) == position.get_neighbour(&Direction::TopLeft) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::TopLeft))];
                }
                if Some(*target) == position.get_neighbour(&Direction::TopRight) {
                    return vec![(
                        *target,
                        PrevisualisationItem::PushArrow(Direction::TopRight),
                    )];
                }
                if Some(*target) == position.get_neighbour(&Direction::Right) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::Right))];
                }
                if Some(*target) == position.get_neighbour(&Direction::BottomRight) {
                    return vec![(
                        *target,
                        PrevisualisationItem::PushArrow(Direction::BottomRight),
                    )];
                }
                if Some(*target) == position.get_neighbour(&Direction::BottomLeft) {
                    return vec![(
                        *target,
                        PrevisualisationItem::PushArrow(Direction::BottomLeft),
                    )];
                }
                if Some(*target) == position.get_neighbour(&Direction::Left) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::Left))];
                }
                vec![]
            }
            Attack::OffensiveSwordFight => {
                if Some(*target) == position.get_neighbour(&Direction::TopLeft) {
                    return vec![(
                        *target,
                        PrevisualisationItem::PushArrow(Direction::BottomRight),
                    )];
                }
                if Some(*target) == position.get_neighbour(&Direction::TopRight) {
                    return vec![(
                        *target,
                        PrevisualisationItem::PushArrow(Direction::BottomLeft),
                    )];
                }
                if Some(*target) == position.get_neighbour(&Direction::Right) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::Left))];
                }
                if Some(*target) == position.get_neighbour(&Direction::BottomRight) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::TopLeft))];
                }
                if Some(*target) == position.get_neighbour(&Direction::BottomLeft) {
                    return vec![(
                        *target,
                        PrevisualisationItem::PushArrow(Direction::TopRight),
                    )];
                }
                if Some(*target) == position.get_neighbour(&Direction::Left) {
                    return vec![(*target, PrevisualisationItem::PushArrow(Direction::Right))];
                }
                vec![]
            }
            Attack::Heal => vec![],
            Attack::VolleyOfArrows => {
                let mut final_target = None;
                let mut final_direction = None;

                for direction in Direction::iter() {
                    let mut targets = Vec::new();
                    let mut right_direction = false;
                    if let Some(index) = position.get_neighbour(&direction) {
                        if &index == target {
                            right_direction = true;
                        }
                        targets.push(index);
                        if units[index.get_index()].is_none() {
                            while let Some(index) =
                                targets[targets.len() - 1].get_neighbour(&direction)
                            {
                                if &index == target {
                                    right_direction = true;
                                }
                                targets.push(index);
                                if units[index.get_index()].is_some() {
                                    break;
                                }
                            }
                            if right_direction {
                                final_target = Some(targets[targets.len() - 1]);
                                final_direction = Some(direction);
                            }
                        }
                    }
                }
                vec![
                    (
                        *position,
                        PrevisualisationItem::LongDistanceShoot(final_target.unwrap()),
                    ),
                    (
                        final_target.unwrap(),
                        PrevisualisationItem::PushArrow(final_direction.unwrap()),
                    ),
                ]
            }
        }
    }
}
