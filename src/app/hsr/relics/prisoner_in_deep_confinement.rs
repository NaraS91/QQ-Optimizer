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
                Stat::Base(BaseStat::Atk),
                BuffScaling::Multiplicative,
                flat_value!(0.12),
            )],
            true,
        ),
        Modifier::new_with_config(
            (wearer.kind, Source::Relic4p),
            vec![ModifierData::new(
                ModifierTarget::Caster,
                Stat::Advanced(AdvancedStat::DefIgnore),
                BuffScaling::Additive,
                |_, _, config, _, _, _| config.unwrap().get_index() as f32 * 0.06,
            )],
            Some(ModifierConfig::new(ConfigType::Stacks(3))),
            true,
        ),
    ]
}
