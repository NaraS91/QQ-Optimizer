use crate::app::hsr::{
    units::{
        AdvancedStat, BonusDMGFlag, BuffScaling, ConfigType, Modifier, ModifierConfig,
        ModifierData, ModifierTarget, Source, Stat, Unit,
    },
    utils::flat_value,
};

fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![
        Modifier::new(
            (wearer.kind, Source::Relic2p),
            vec![ModifierData::new(
                ModifierTarget::Caster,
                Stat::Advanced(AdvancedStat::TotalDmgBoost(BonusDMGFlag::MAX)),
                BuffScaling::Additive,
                flat_value!(0.12),
            )],
            true,
        ),
        Modifier::new_with_config(
            (wearer.kind, Source::Relic4p),
            vec![
                ModifierData::new(
                    ModifierTarget::Caster,
                    Stat::Advanced(AdvancedStat::CritDamage),
                    BuffScaling::Additive,
                    |_, _, config, _, _, _| {
                        [0., 0., 0.08, 0.12][config.unwrap().get_index()]
                            * if config.unwrap().get_bool() { 2. } else { 1. }
                    },
                ),
                ModifierData::new(
                    ModifierTarget::Caster,
                    Stat::Advanced(AdvancedStat::CritRate),
                    BuffScaling::Additive,
                    flat_value!(0.04),
                ),
            ],
            Some(ModifierConfig::new(ConfigType::StacksWithFlag(3))),
            true,
        ),
    ]
}
