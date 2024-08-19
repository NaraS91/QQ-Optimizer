use crate::app::hsr::{
    units::{
        AdvancedStat, BuffScaling, Modifier, ModifierData, ModifierTarget, Source, Stat, Unit,
    },
    utils::flat_value,
};

fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![Modifier::new(
        (wearer.kind, Source::Relic2p),
        vec![
            ModifierData::new(
                ModifierTarget::Caster,
                Stat::Advanced(AdvancedStat::TotalDmgBoost(
                    AdvancedStat::create_dmg_bonus_flag(1, 0, 1, 0, 0),
                )),
                BuffScaling::Additive,
                |unit, _, config, team, lc_store, relics_store| {
                    let cr = unit.get_effective_advanced_stat(
                        AdvancedStat::CritRate,
                        team,
                        lc_store,
                        relics_store,
                    );
                    if cr >= 0.7 {
                        0.2
                    } else {
                        0.
                    }
                },
            ),
            ModifierData::new(
                ModifierTarget::Caster,
                Stat::Advanced(AdvancedStat::CritRate),
                BuffScaling::Additive,
                flat_value!(0.08),
            ),
        ],
        false,
    )]
}
