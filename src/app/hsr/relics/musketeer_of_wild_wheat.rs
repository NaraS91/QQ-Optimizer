use crate::app::hsr::{
    units::{
        AdvancedStat, BaseStat, BuffScaling, Modifier, ModifierData, ModifierTarget, Source, Stat,
        Unit,
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
        Modifier::new(
            (wearer.kind, Source::Relic4p),
            vec![
                ModifierData::new(
                    ModifierTarget::Caster,
                    Stat::Base(BaseStat::Spd),
                    BuffScaling::Multiplicative,
                    flat_value!(0.06),
                ),
                ModifierData::new(
                    ModifierTarget::Caster,
                    Stat::Advanced(AdvancedStat::TotalDmgBoost(
                        AdvancedStat::create_dmg_bonus_flag(1, 0, 0, 0, 0),
                    )),
                    BuffScaling::Multiplicative,
                    flat_value!(0.10),
                ),
            ],
            true,
        ),
    ]
}
