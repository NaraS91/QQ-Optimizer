use crate::app::hsr::{
    units::{
        BaseStat, BuffScaling, Modifier,
        ModifierData, ModifierTarget, Source, Stat, Unit,
    },
    utils::value_with_lc_s,
};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![Modifier::new(
        (wearer.kind, Source::LightCone),
        vec![
            ModifierData::new(
                ModifierTarget::Caster,
                Stat::Base(BaseStat::Atk),
                BuffScaling::Multiplicative,
                value_with_lc_s!(|s: usize| SKILL_PARAMS[s].0),
            ),
            ModifierData::new(
                ModifierTarget::Team,
                Stat::Base(BaseStat::Spd),
                BuffScaling::Additive,
                value_with_lc_s!(|s: usize| SKILL_PARAMS[s].1),
            ),
        ],
        true,
    )]
}

const SKILL_PARAMS: [(f32, f32, f32, f32); 5] = [
    (0.2400, 12.0000, 3.0000, 3.0000),
    (0.2800, 14.0000, 3.5000, 3.0000),
    (0.3200, 16.0000, 4.0000, 3.0000),
    (0.3600, 18.0000, 4.5000, 3.0000),
    (0.4000, 20.0000, 5.0000, 3.0000),
];
