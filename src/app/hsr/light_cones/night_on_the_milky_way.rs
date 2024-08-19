use crate::app::hsr::{
    units::{
        AdvancedStat, BaseStat, BonusDMGFlag, BuffScaling, ConfigType, Modifier, ModifierConfig,
        ModifierData, ModifierTarget, Source, Stat, Unit,
    },
    utils::{flat_value, lc_superimposition},
};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![Modifier::new_with_config(
        (wearer.kind, Source::LightCone),
        vec![
            ModifierData::new(
                ModifierTarget::Caster,
                Stat::Base(BaseStat::Atk),
                BuffScaling::Multiplicative,
                |_, unit, config, _, lc_store, _| {
                    config.unwrap().get_index() as f32
                        * SKILL_PARAMS[lc_superimposition!(unit, lc_store) as usize].1
                },
            ),
            ModifierData::new(
                ModifierTarget::Caster,
                Stat::Advanced(AdvancedStat::TotalDmgBoost(BonusDMGFlag::MAX)),
                BuffScaling::Additive,
                |_, unit, config, _, lc_store, _| {
                    config.unwrap().get_index() as f32
                        * SKILL_PARAMS[lc_superimposition!(unit, lc_store) as usize].0
                },
            ),
        ],
        Some(ModifierConfig::new(ConfigType::StacksWithFlag(5))),
        true,
    )]
}

const SKILL_PARAMS: [(f32, f32); 5] = [
    (0.3000, 0.0900),
    (0.3500, 0.1050),
    (0.4000, 0.1200),
    (0.4500, 0.1350),
    (0.5000, 0.1500),
];
