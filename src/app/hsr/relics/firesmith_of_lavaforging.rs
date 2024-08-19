use crate::app::hsr::{
    basics::Element,
    units::{
        AdvancedStat, BuffScaling, ConfigType, Modifier, ModifierConfig, ModifierData,
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
                Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Fire)),
                BuffScaling::Additive,
                flat_value!(0.1),
            )],
            true,
        ),
        Modifier::new_with_config(
            (wearer.kind, Source::Relic4p),
            vec![
                ModifierData::new(
                    ModifierTarget::Caster,
                    Stat::Advanced(AdvancedStat::TotalDmgBoost(
                        AdvancedStat::create_dmg_bonus_flag(0, 0, 1, 0, 0),
                    )),
                    BuffScaling::Additive,
                    flat_value!(0.12),
                ),
                ModifierData::new(
                    ModifierTarget::Caster,
                    Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Fire)),
                    BuffScaling::Additive,
                    |_, _, config, _, _, _| if config.unwrap().get_bool() { 0.12 } else { 0. },
                ),
            ],
            Some(ModifierConfig::new(ConfigType::Flag)),
            true,
        ),
    ]
}
