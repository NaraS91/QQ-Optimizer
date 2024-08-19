use crate::app::hsr::utils::flat_value;

use super::{
    AdvancedStat, BaseStat, BonusDMGFlag, BuffScaling, Modifier, ModifierData, ModifierOrDOT,
    ModifierTarget, Source, Stat, Unit,
};
pub fn modifiers(unit: &Unit) -> Vec<ModifierOrDOT> {
    vec![
        ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Skill),
            vec![ModifierData::new(
                ModifierTarget::Enemy,
                Stat::Base(BaseStat::Spd),
                BuffScaling::Multiplicative,
                flat_value!(-0.1),
            )],
            true,
        )),
        ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Ultimate),
            vec![ModifierData::new(
                ModifierTarget::Enemies,
                Stat::Base(BaseStat::Spd),
                BuffScaling::Multiplicative,
                flat_value!(-0.1),
            )],
            true,
        )),
        ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Trace(1)),
            vec![ModifierData::new(
                ModifierTarget::Enemies,
                Stat::Advanced(AdvancedStat::TotalDmgReceived(BonusDMGFlag::MAX)),
                BuffScaling::Multiplicative,
                flat_value!(0.12),
            )],
            true,
        )),
    ]
}

const SKILL_PARAMS: [(f32, f32, f32, f32); 15] = [
    (0.3600, 0.6500, 0.1000, 2.0000),
    (0.3960, 0.6600, 0.1000, 2.0000),
    (0.4320, 0.6700, 0.1000, 2.0000),
    (0.4680, 0.6800, 0.1000, 2.0000),
    (0.5040, 0.6900, 0.1000, 2.0000),
    (0.5400, 0.7000, 0.1000, 2.0000),
    (0.5850, 0.7125, 0.1000, 2.0000),
    (0.6300, 0.7250, 0.1000, 2.0000),
    (0.6750, 0.7375, 0.1000, 2.0000),
    (0.7200, 0.7500, 0.1000, 2.0000),
    (0.7560, 0.7600, 0.1000, 2.0000),
    (0.7920, 0.7700, 0.1000, 2.0000),
    (0.8280, 0.7800, 0.1000, 2.0000),
    (0.8640, 0.7900, 0.1000, 2.0000),
    (0.9000, 0.8000, 0.1000, 2.0000),
];

const ULT_PARAMS: [(f32, f32, f32, f32); 15] = [
    (0.9000, 0.3200, 1.0000, 0.1000),
    (0.9600, 0.3280, 1.0000, 0.1000),
    (1.0200, 0.3360, 1.0000, 0.1000),
    (1.0800, 0.3440, 1.0000, 0.1000),
    (1.1400, 0.3520, 1.0000, 0.1000),
    (1.2000, 0.3600, 1.0000, 0.1000),
    (1.2750, 0.3700, 1.0000, 0.1000),
    (1.3500, 0.3800, 1.0000, 0.1000),
    (1.4250, 0.3900, 1.0000, 0.1000),
    (1.5000, 0.4000, 1.0000, 0.1000),
    (1.5600, 0.4080, 1.0000, 0.1000),
    (1.6200, 0.4160, 1.0000, 0.1000),
    (1.6800, 0.4240, 1.0000, 0.1000),
    (1.7400, 0.4320, 1.0000, 0.1000),
    (1.8000, 0.4400, 1.0000, 0.1000),
];

const TALENT_PARAMS: [f32; 15] = [
    0.3000, 0.3300, 0.3600, 0.3900, 0.4200, 0.4500, 0.4875, 0.5250, 0.5625, 0.6000, 0.6300, 0.6600,
    0.6900, 0.7200, 0.7500,
];

const TECH_PARAMS: [(f32, f32, f32, f32, f32); 1] = [(1.0000, 0.2000, 0.1000, 15.0000, 0.5000)];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];
