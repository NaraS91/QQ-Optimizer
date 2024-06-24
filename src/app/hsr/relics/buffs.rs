use crate::app::hsr::{
    basics::Element,
    units::{AdvancedStat, BaseStat, BuffScaling, ModifierData, ModifierTarget, Stat},
};

use super::{CavernSet, PlanarSet, RelicSet};

pub fn get_cavern_set_effects(cavern_set: CavernSet, full_set: bool) -> Vec<ModifierData> {
    match cavern_set {
        CavernSet::Genius_Of_Brilliant_Stars => genius_of_brilliant_stars(full_set),
        _ => genius_of_brilliant_stars(full_set),
    }
}

pub fn get_planar_set_effects(planar_set: PlanarSet) -> Vec<ModifierData> {
    match planar_set {
        _ => Vec::new(),
    }
}

fn genius_of_brilliant_stars(full_set: bool) -> Vec<ModifierData> {
    let mut result = vec![ModifierData::new(
        ModifierTarget::Caster,
        Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Quantum)),
        BuffScaling::Additive,
        |_, _, _, _, _, _| 0.1,
    )];

    if full_set {
        result.push(ModifierData::new(
            ModifierTarget::Enemy,
            Stat::Base(BaseStat::Def),
            BuffScaling::Multiplicative,
            |target, _, _, _, _, _| {
                if target.is_weak_to(Element::Quantum) {
                    -0.2
                } else {
                    -0.1
                }
            },
        ))
    }

    result
}
