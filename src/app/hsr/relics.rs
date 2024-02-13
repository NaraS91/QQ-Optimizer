use enum_map::Enum;
use serde::{Deserialize, Serialize};

use super::units::UnitKind;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Relic {
    pub id: usize,
    pub level: u8,
    pub part: RelicPart,
    pub set: RelicSet,
    pub main_stat: RelicStat,
    pub sub1: RelicStat,
    pub sub2: RelicStat,
    pub sub3: RelicStat,
    pub sub4: Option<RelicStat>,
    pub equipped: Option<UnitKind>
}

#[derive(Clone, Copy, Serialize, Deserialize, Enum)]
pub enum RelicSet {
    Cavern(CavernSet),
    Planar(PlanarSet)
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum RelicPart {
    Head,
    Hands,
    Body,
    Feet,
    Sphere,
    Rope
}

impl RelicPart {
    pub fn get_index(&self) -> usize {
        match self {
            Self::Head => 0,
            Self::Hands => 1,
            Self::Body => 2,
            Self::Feet => 3,
            Self::Sphere => 4,
            Self::Rope => 5
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Enum)]
enum CavernSet {
    Passerby,
    Musketeer,
    Knight,
    Hunter,
    Champion,
    Guard,
    Firesmith,
    Genius,
    Band,
    Eagle,
    Thief,
    Wastelander,
    Longevous,
    Messenger,
    Ashblazing,
    Prisoner
}

#[derive(Clone, Copy, Serialize, Deserialize, Enum)]
enum PlanarSet {
    SpaceSealing,
    Fleet,
    PanCosminc,
    Belobog,
    Celestial,
    InertSalsatto,
    Talia,
    Sprightly,
    RutilantArena,
    BrokenKell,
    Firmament,
    Penacony
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum RelicStat {
    Atk(u32),
    AktP(f32),
    Def(u32),
    DefP(f32),
    HpP(f32),
    Hp(u32),
    Err(f32),
    Cr(f32),
    Cd(f32),
    BE(f32),
    Spd(f32),
    EHR(f32),
    ERes(f32)
}