use crate::app::hsr::{
    units::{
        AdvancedStat, BaseStat, BuffScaling, ConfigType, Modifier, ModifierConfig, ModifierData,
        ModifierTarget, Source, Stat, Unit,
    },
    utils::flat_value,
};

fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![
        Modifier::new(
            (wearer.kind, Source::Relic2p),
            vec![ModifierData::new(
                ModifierTarget::Caster,
                Stat::Base(BaseStat::Hp),
                BuffScaling::Multiplicative,
                flat_value!(0.12),
            )],
            true,
        ),
        Modifier::new_with_config(
            (wearer.kind, Source::Relic4p),
            vec![ModifierData::new(
                ModifierTarget::Caster,
                Stat::Advanced(AdvancedStat::CritRate),
                BuffScaling::Additive,
                |_, _, config, _, _, _| config.unwrap().get_index() as f32 * 0.08,
            )],
            Some(ModifierConfig::new(ConfigType::Stacks(2))),
            true,
        ),
    ]
}
