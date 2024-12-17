use super::{AdvancedStat, BaseStat, EnergyRegen, SkillData, Stat, ToughnessReduction, UnitKind};
use crate::app::hsr::basics::{Element, Path};
use enum_map::Enum;

pub struct UnitStats {
    pub rarity: u8,
    pub base: [[f32; BaseStat::LENGTH]; 7],
    pub growth: [f32; BaseStat::LENGTH],
}

pub const CHARACTER_INFO: [(
    UnitKind,
    Path,
    UnitStats,
    [(Stat, f32); 10],
    [Option<SkillData>; 10],
); 0] = [];
