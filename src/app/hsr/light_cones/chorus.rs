use crate::app::hsr::{
    units::{BaseStat, BuffScaling, Modifier, ModifierData, ModifierTarget, Source, Stat, Unit},
    utils::value_with_lc_s,
};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![Modifier::new(
        (wearer.kind, Source::LightCone),
        vec![ModifierData::new(
            ModifierTarget::Team,
            Stat::Base(BaseStat::Atk),
            BuffScaling::Additive,
            value_with_lc_s!(|s: usize| SKILL_PARAMS[s]),
        )],
        true,
    )]
}

const SKILL_PARAMS: [f32; 5] = [0.0800, 0.0900, 0.1000, 0.1100, 0.1200];
