use crate::app::hsr::{
    units::{
        BaseStat, BuffScaling, ConfigType, Modifier, ModifierConfig, ModifierData, ModifierTarget,
        Source, Stat, Unit,
    },
    utils::flat_value,
};

fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![
        Modifier::new(
            (wearer.kind, Source::Relic2p),
            vec![ModifierData::new(
                ModifierTarget::Caster,
                Stat::Base(BaseStat::Spd),
                BuffScaling::Multiplicative,
                flat_value!(0.06),
            )],
            true,
        ),
        Modifier::new_with_config(
            (wearer.kind, Source::Relic4p),
            vec![ModifierData::new(
                ModifierTarget::Allies,
                Stat::Base(BaseStat::Spd),
                BuffScaling::Multiplicative,
                |_, _, config, _, _, _| if config.unwrap().get_bool() { 0.12 } else { 0. },
            )],
            Some(ModifierConfig::new(ConfigType::Flag)),
            true,
        ),
    ]
}
