const BOSS_DEF: f32 = 1150.;
const BOSS_RES: f32 = 0.;

// returns (avg_dmg, non_crit dmg, crit_dmg)
pub fn standard_attack(
    base_power: f32,
    unit_level: f32,
    atk: f32,
    crit_rate: f32,
    crit_dmg: f32,
    elemental_boost: f32,
    vulnarability: f32,
    res_pen: f32,
    is_broken: bool,
) -> (f32, f32, f32) {
    let base_dmg = base_power
        * atk
        * (1. + elemental_boost)
        * (1. - (BOSS_DEF / (BOSS_DEF + 200. + 10. * unit_level)))
        * (1. - (BOSS_RES - res_pen))
        * (1. + vulnarability)
        * (if is_broken { 0.9 } else { 1.0 });

    (
        base_dmg * (1. - crit_rate).max(0.) + base_dmg * (1. + crit_dmg) * crit_rate.min(1.),
        base_dmg,
        base_dmg * (1. + crit_dmg),
    )
}
