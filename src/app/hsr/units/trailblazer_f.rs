use crate::app::hsr::utils::flat_value;

use super::{
    AdvancedStat, BaseStat, BonusDMGFlag, BuffScaling, Modifier, ModifierData, ModifierOrDOT,
    ModifierTarget, Source, Stat, Unit,
};

pub fn modifiers(unit: &Unit) -> Vec<ModifierOrDOT> {
    let mut result = vec![ModifierOrDOT::Modifier(Modifier::new(
        (unit.kind, Source::Trace(2)),
        vec![ModifierData::new(
            ModifierTarget::Team,
            Stat::Advanced(AdvancedStat::TotalDmgReceived(BonusDMGFlag::MAX)),
            BuffScaling::Additive,
            flat_value!(-0.15),
        )],
        true,
    ))];

    if unit.unique_data.eidolon >= 6 {
        result.push(ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Eidolon(6)),
            vec![ModifierData::new(
                ModifierTarget::Caster,
                Stat::Base(BaseStat::Def),
                BuffScaling::Multiplicative,
                flat_value!(0.1),
            )],
            true,
        )))
    }

    result
}

const SKILL_PARAMS: [(f32, f32, f32); 15] = [
    (0.4000, 1.0000, 1.0000),
    (0.4100, 1.0000, 1.0000),
    (0.4200, 1.0000, 1.0000),
    (0.4300, 1.0000, 1.0000),
    (0.4400, 1.0000, 1.0000),
    (0.4500, 1.0000, 1.0000),
    (0.4625, 1.0000, 1.0000),
    (0.4750, 1.0000, 1.0000),
    (0.4875, 1.0000, 1.0000),
    (0.5000, 1.0000, 1.0000),
    (0.5100, 1.0000, 1.0000),
    (0.5200, 1.0000, 1.0000),
    (0.5300, 1.0000, 1.0000),
    (0.5400, 1.0000, 1.0000),
    (0.5500, 1.0000, 1.0000),
];

const ULT_PARAMS: [(f32, f32); 15] = [
    (0.5000, 0.7500),
    (0.5500, 0.8250),
    (0.6000, 0.9000),
    (0.6500, 0.9750),
    (0.7000, 1.0500),
    (0.7500, 1.1250),
    (0.8125, 1.2188),
    (0.8750, 1.3125),
    (0.9375, 1.4062),
    (1.0000, 1.5000),
    (1.0500, 1.5750),
    (1.1000, 1.6500),
    (1.1500, 1.7250),
    (1.2000, 1.8000),
    (1.2500, 1.8750),
];

const TALENT_PARAMS: [(f32, f32, f32, f32); 15] = [
    (0.0400, 2.0000, 8.0000, 20.0000),
    (0.0425, 2.0000, 8.0000, 32.0000),
    (0.0450, 2.0000, 8.0000, 41.0000),
    (0.0475, 2.0000, 8.0000, 50.0000),
    (0.0500, 2.0000, 8.0000, 56.0000),
    (0.0520, 2.0000, 8.0000, 62.0000),
    (0.0540, 2.0000, 8.0000, 66.5000),
    (0.0560, 2.0000, 8.0000, 71.0000),
    (0.0580, 2.0000, 8.0000, 75.5000),
    (0.0600, 2.0000, 8.0000, 80.0000),
    (0.0620, 2.0000, 8.0000, 84.5000),
    (0.0640, 2.0000, 8.0000, 89.0000),
    (0.0660, 2.0000, 8.0000, 93.5000),
    (0.0680, 2.0000, 8.0000, 98.0000),
    (0.0700, 2.0000, 8.0000, 102.5000),
];

const TECH_PARAMS: [(f32, f32, f32); 1] = [(0.3000, 384.0000, 1.0000)];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];
