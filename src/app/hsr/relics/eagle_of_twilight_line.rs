use crate::app::hsr::{
    basics::Element,
    units::{
        AdvancedStat, BuffScaling, Modifier, ModifierData, ModifierTarget, Source, Stat, Unit,
    },
    utils::flat_value,
};

fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![Modifier::new(
        (wearer.kind, Source::Relic2p),
        vec![ModifierData::new(
            ModifierTarget::Caster,
            Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Wind)),
            BuffScaling::Additive,
            flat_value!(0.1),
        )],
        true,
    )]
}
