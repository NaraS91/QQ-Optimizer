use crate::app::hsr::{
    units::{
        AdvancedStat, BonusDMGFlag, BuffScaling, Modifier, ModifierData, ModifierTarget, Source,
        Stat, Unit,
    },
    utils::value_with_lc_s,
};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![Modifier::new(
        (wearer.kind, Source::LightCone),
        vec![
            ModifierData::new(
                ModifierTarget::Caster,
                Stat::Advanced(AdvancedStat::EnergyRegenerationRate),
                BuffScaling::Additive,
                value_with_lc_s!(|s: usize| SKILL_PARAMS[s].0),
            ),
            ModifierData::new(
                ModifierTarget::Ally,
                Stat::Advanced(AdvancedStat::TotalDmgBoost(BonusDMGFlag::MAX)),
                BuffScaling::Additive,
                value_with_lc_s!(|s: usize| SKILL_PARAMS[s].1),
            ),
        ],
        true,
    )]
}

const SKILL_PARAMS: [(f32, f32, f32); 5] = [
    (0.1000, 0.3000, 1.0000),
    (0.1200, 0.3500, 1.0000),
    (0.1400, 0.4000, 1.0000),
    (0.1600, 0.4500, 1.0000),
    (0.1800, 0.5000, 1.0000),
];
