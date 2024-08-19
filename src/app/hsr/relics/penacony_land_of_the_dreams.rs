use crate::app::hsr::{
    units::{
        AdvancedStat, BonusDMGFlag, BuffScaling, Modifier, ModifierData, ModifierTarget, Source,
        Stat, Unit,
    },
    utils::flat_value,
};

fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![Modifier::new(
        (wearer.kind, Source::Relic2p),
        vec![
            ModifierData::new(
                ModifierTarget::Allies,
                Stat::Advanced(AdvancedStat::TotalDmgBoost(BonusDMGFlag::MAX)),
                BuffScaling::Multiplicative,
                |target, buffer, _, _, _, _| {
                    if target.main_element == buffer.main_element {
                        0.1
                    } else {
                        0.
                    }
                },
            ),
            ModifierData::new(
                ModifierTarget::Caster,
                Stat::Advanced(AdvancedStat::EnergyRegenerationRate),
                BuffScaling::Additive,
                flat_value!(0.05),
            ),
        ],
        true,
    )]
}
