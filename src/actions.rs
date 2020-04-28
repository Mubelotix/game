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
    pub fn apply(
        consequences: Vec<(HexIndex, PrevisualisationItem)>,
        _map: &mut Map,
        units: &mut [Option<Unit>; 61],
    ) {
        for consequence in consequences {
            match consequence {
                (position, PrevisualisationItem::LifeChange(life)) => {
                    if life.is_dead() {
                        units[position.get_index()] = None;
                    } else {
                        units[position.get_index()].as_mut().unwrap().life = life;
                    }
                }
                (_position, PrevisualisationItem::LongDistanceShoot(_target)) => {
                    // TODO burn forests, destroy montains
                }
                (position, PrevisualisationItem::PushArrow(direction, cancelled)) => {
                    if !cancelled {
                        if let Some(new_position) = position.get_neighbour(&direction) {
                            if units[new_position.get_index()].is_none() {
                                if let Some(unit) = units[position.get_index()].take() {
                                    units[new_position.get_index()] = Some(unit);
                                }
                            }
                        }
                    }
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
                for direction in Direction::iter() {
                    if Some(*target) == position.get_neighbour(&direction) {
                        let mut consequences = Vec::new();
                        if let Some(life) = units[target.get_index()].as_ref().map(|u| &u.life) {
                            if let Some((other_pos, Some(other_life))) =
                                target.get_neighbour(&direction).map(|new_pos| {
                                    (
                                        new_pos,
                                        units[new_pos.get_index()].as_ref().map(|u| &u.life),
                                    )
                                })
                            {
                                consequences.push((
                                    *target,
                                    PrevisualisationItem::LifeChange(life.previsualise_loss(2)),
                                ));
                                consequences.push((
                                    other_pos,
                                    PrevisualisationItem::LifeChange(
                                        other_life.previsualise_loss(1),
                                    ),
                                ));
                                consequences.push((
                                    *target,
                                    PrevisualisationItem::PushArrow(direction, true),
                                ));
                            } else {
                                consequences.push((
                                    *target,
                                    PrevisualisationItem::LifeChange(life.previsualise_loss(1)),
                                ));
                                consequences.push((
                                    *target,
                                    PrevisualisationItem::PushArrow(direction, false),
                                ));
                            }
                        } else {
                            consequences
                                .push((*target, PrevisualisationItem::PushArrow(direction, false)));
                        }
                        return consequences;
                    }
                }
            }
            Attack::DefensiveSwordFight => {
                for direction in Direction::iter() {
                    if Some(*target) == position.get_neighbour(&direction) {
                        let mut consequences = Vec::new();
                        if let Some(life) = units[target.get_index()].as_ref().map(|u| &u.life) {
                            if let Some((other_pos, Some(other_life))) =
                                target.get_neighbour(&direction).map(|new_pos| {
                                    (
                                        new_pos,
                                        units[new_pos.get_index()].as_ref().map(|u| &u.life),
                                    )
                                })
                            {
                                consequences.push((
                                    *target,
                                    PrevisualisationItem::LifeChange(life.previsualise_loss(3)),
                                ));
                                consequences.push((
                                    other_pos,
                                    PrevisualisationItem::LifeChange(
                                        other_life.previsualise_loss(1),
                                    ),
                                ));
                                consequences.push((
                                    *target,
                                    PrevisualisationItem::PushArrow(direction, true),
                                ));
                            } else {
                                consequences.push((
                                    *target,
                                    PrevisualisationItem::LifeChange(life.previsualise_loss(2)),
                                ));
                                consequences.push((
                                    *target,
                                    PrevisualisationItem::PushArrow(direction, false),
                                ));
                            }
                        } else {
                            consequences
                                .push((*target, PrevisualisationItem::PushArrow(direction, false)));
                        }

                        return consequences;
                    }
                }
            }
            Attack::OffensiveSwordFight => {
                for direction in Direction::iter() {
                    if Some(*target) == position.get_neighbour(&direction) {
                        let mut consequences = Vec::new();
                        if let Some(life) = units[target.get_index()].as_ref().map(|u| &u.life) {
                            if let Some((other_pos, Some(other_life))) =
                                target.get_neighbour(&!direction.clone()).map(|new_pos| {
                                    (
                                        new_pos,
                                        units[new_pos.get_index()].as_ref().map(|u| &u.life),
                                    )
                                })
                            {
                                consequences.push((
                                    *target,
                                    PrevisualisationItem::LifeChange(life.previsualise_loss(3)),
                                ));
                                consequences.push((
                                    other_pos,
                                    PrevisualisationItem::LifeChange(
                                        other_life.previsualise_loss(1),
                                    ),
                                ));
                                consequences.push((
                                    *target,
                                    PrevisualisationItem::PushArrow(!direction, true),
                                ));
                            } else {
                                consequences.push((
                                    *target,
                                    PrevisualisationItem::LifeChange(life.previsualise_loss(2)),
                                ));
                                consequences.push((
                                    *target,
                                    PrevisualisationItem::PushArrow(!direction, false),
                                ));
                            }
                        } else {
                            consequences.push((
                                *target,
                                PrevisualisationItem::PushArrow(!direction, false),
                            ));
                        }

                        return consequences;
                    }
                }
            }
            Attack::Heal => {
                if let Some(life) = units[target.get_index()].as_ref().map(|u| &u.life) {
                    return vec![(
                        *target,
                        PrevisualisationItem::LifeChange(life.previsualise_loss(-1)),
                    )];
                }
            }
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
                        } else if right_direction {
                            final_target = Some(targets[targets.len() - 1]);
                            final_direction = Some(direction);
                        }
                    }
                }

                let mut consequences = Vec::new();
                if let Some(life) = units[final_target.unwrap().get_index()]
                    .as_ref()
                    .map(|u| &u.life)
                {
                    if let Some((other_pos, Some(other_life))) = target
                        .get_neighbour(&final_direction.clone().unwrap())
                        .map(|new_pos| {
                            (
                                new_pos,
                                units[new_pos.get_index()].as_ref().map(|u| &u.life),
                            )
                        })
                    {
                        consequences.push((
                            final_target.unwrap(),
                            PrevisualisationItem::LifeChange(life.previsualise_loss(3)),
                        ));
                        consequences.push((
                            other_pos,
                            PrevisualisationItem::LifeChange(other_life.previsualise_loss(1)),
                        ));
                        consequences.push((
                            final_target.unwrap(),
                            PrevisualisationItem::PushArrow(final_direction.unwrap(), true),
                        ));
                    } else {
                        consequences.push((
                            final_target.unwrap(),
                            PrevisualisationItem::LifeChange(life.previsualise_loss(2)),
                        ));
                        consequences.push((
                            final_target.unwrap(),
                            PrevisualisationItem::PushArrow(final_direction.unwrap(), false),
                        ));
                    }
                }

                consequences.push((
                    *position,
                    PrevisualisationItem::LongDistanceShoot(final_target.unwrap()),
                ));

                return consequences;
            }
        }
        Vec::new()
    }
}
