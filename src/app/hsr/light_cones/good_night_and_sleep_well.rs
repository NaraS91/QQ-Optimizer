use crate::app::hsr::{
    units::{
        AdvancedStat, BonusDMGFlag, BuffScaling, ConfigType, Modifier, ModifierConfig,
        ModifierData, ModifierTarget, Source, Stat, Unit,
    },
    utils::{lc_superimposition},
};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![Modifier::new_with_config(
        (wearer.kind, Source::LightCone),
        vec![ModifierData::new(
            ModifierTarget::Caster,
            Stat::Advanced(AdvancedStat::TotalDmgBoost(BonusDMGFlag::MAX)),
            BuffScaling::Additive,
            |_, unit, config, _, lc_store, _| {
                SKILL_PARAMS[lc_superimposition!(unit, lc_store) as usize].0
                    * config.unwrap().get_index() as f32
            },
        )],
        Some(ModifierConfig::new(ConfigType::Stacks(3))),
        true,
    )]
}

const SKILL_PARAMS: [(f32, f32); 5] = [
    (0.1200, 3.0000),
    (0.1500, 3.0000),
    (0.1800, 3.0000),
    (0.2100, 3.0000),
    (0.2400, 3.0000),
];
