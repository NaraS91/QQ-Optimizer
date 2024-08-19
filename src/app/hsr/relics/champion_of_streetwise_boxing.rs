use crate::app::hsr::{
    basics::Element,
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
                Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Physical)),
                BuffScaling::Additive,
                flat_value!(0.1),
            )],
            true,
        ),
        Modifier::new_with_config(
            (wearer.kind, Source::Relic4p),
            vec![ModifierData::new(
                ModifierTarget::Caster,
                Stat::Base(BaseStat::Atk),
                BuffScaling::Multiplicative,
                |_, _, config, _, _, _| 0.05 * config.unwrap().get_index() as f32,
            )],
            Some(ModifierConfig::new(ConfigType::Stacks(5))),
            true,
        ),
    ]
}
