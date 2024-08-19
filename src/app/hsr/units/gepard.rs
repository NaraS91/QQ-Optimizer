use crate::app::hsr::utils::flat_value;

use super::{
    AdvancedStat, BaseStat, BuffScaling, Modifier, ModifierData, ModifierOrDOT, ModifierTarget,
    Source, Stat, Unit, UnitKind,
};

pub fn modifiers(unit: &Unit) -> Vec<ModifierOrDOT> {
    let mut result: Vec<ModifierOrDOT> = Vec::new();

    if unit.unique_data.eidolon >= 2 {
        result.push(ModifierOrDOT::Modifier(Modifier::new(
            (UnitKind::Gepard, Source::Eidolon(2)),
            vec![ModifierData::new(
                ModifierTarget::Enemy,
                Stat::Base(BaseStat::Spd),
                BuffScaling::Multiplicative,
                flat_value!(-0.2),
            )],
            true,
        )))
    }

    if unit.unique_data.eidolon >= 4 {
        result.push(ModifierOrDOT::Modifier(Modifier::new(
            (UnitKind::Gepard, Source::Eidolon(4)),
            vec![ModifierData::new(
                ModifierTarget::Team,
                Stat::Advanced(AdvancedStat::EffectRes),
                BuffScaling::Additive,
                flat_value!(0.2),
            )],
            true,
        )))
    }

    result
}

const SKILL_PARAMS: [(f32, f32, f32, f32); 15] = [
    (1.0000, 0.6500, 1.0000, 0.3000),
    (1.1000, 0.6500, 1.0000, 0.3300),
    (1.2000, 0.6500, 1.0000, 0.3600),
    (1.3000, 0.6500, 1.0000, 0.3900),
    (1.4000, 0.6500, 1.0000, 0.4200),
    (1.5000, 0.6500, 1.0000, 0.4500),
    (1.6250, 0.6500, 1.0000, 0.4875),
    (1.7500, 0.6500, 1.0000, 0.5250),
    (1.8750, 0.6500, 1.0000, 0.5625),
    (2.0000, 0.6500, 1.0000, 0.6000),
    (2.1000, 0.6500, 1.0000, 0.6300),
    (2.2000, 0.6500, 1.0000, 0.6600),
    (2.3000, 0.6500, 1.0000, 0.6900),
    (2.4000, 0.6500, 1.0000, 0.7200),
    (2.5000, 0.6500, 1.0000, 0.7500),
];

const ULT_PARAMS: [(f32, f32, f32); 15] = [
    (0.3000, 3.0000, 150.0000),
    (0.3187, 3.0000, 240.0000),
    (0.3375, 3.0000, 307.5000),
    (0.3563, 3.0000, 375.0000),
    (0.3750, 3.0000, 420.0000),
    (0.3900, 3.0000, 465.0000),
    (0.4050, 3.0000, 498.7500),
    (0.4200, 3.0000, 532.5000),
    (0.4350, 3.0000, 566.2500),
    (0.4500, 3.0000, 600.0000),
    (0.4650, 3.0000, 633.7500),
    (0.4800, 3.0000, 667.5000),
    (0.4950, 3.0000, 701.2500),
    (0.5100, 3.0000, 735.0000),
    (0.5250, 3.0000, 768.7500),
];

const TALENT_PARAMS: [f32; 15] = [
    0.2500, 0.2750, 0.3000, 0.3250, 0.3500, 0.3750, 0.4062, 0.4375, 0.4688, 0.5000, 0.5250, 0.5500,
    0.5750, 0.6000, 0.6250,
];

const TECH_PARAMS: [(f32, f32, f32); 1] = [(0.2400, 2.0000, 150.0000)];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];
