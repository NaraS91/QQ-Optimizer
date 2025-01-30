use crate::app::{light_cones_store::LightConesStore, relics_store::RelicsStore, ASSETS_LOADER};
use egui::WidgetText;
use enum_map::{enum_map, Enum};
use itertools::Itertools;
use log::debug;
use serde::{Deserialize, Serialize};
use std::{
    convert::identity,
    hash::Hash,
    io::{BufRead, Lines},
    str::FromStr,
};
use strum_macros::EnumIter;

use self::statistics::CHARACTER_INFO;

use super::{
    basics::{self, Element, Path},
    light_cones,
    relics::{Relic, RelicPart},
};

mod acheron;
mod argenti;
mod arlan;
mod asta;
mod aventurine;
mod bailu;
mod black_swan;
mod blade;
mod boothill;
mod bronya;
mod clara;
mod dan_heng;
mod dan_heng_il;
mod dr_ratio;
mod feixiao;
mod fu_xuan;
mod gallagher;
mod gepard;
mod guinaifen;
mod hanya;
mod herta;
mod himeko;
mod hook;
mod huohuo;
mod jade;
mod jiaoqiu;
mod jing_yuan;
mod jingliu;
mod kafka;
mod lingsha;
mod luka;
mod luocha;
mod lynx;
mod mar7th2;
mod march_7th;
mod misha;
mod moze;
mod natasha;
mod pela;
mod qingque;
mod robin;
mod ruan_mei;
mod sam;
mod sampo;
mod seele;
mod serval;
mod silver_wolf;
mod sparkle;
mod sushang;
mod tingyun;
mod topaz;
mod trailblazer_f;
mod trailblazer_i;
mod trailblazer_p;
mod welt;
mod xueyi;
mod yanqing;
mod yukong;
mod yunli;

mod statistics;

#[allow(non_camel_case_types)]
#[derive(Debug, Enum, EnumIter, PartialEq, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum UnitKind {
    Acheron,
    Argenti,
    Arlan,
    Asta,
    Aventurine,
    Bailu,
    Black_Swan,
    Blade,
    Boothill,
    Bronya,
    Clara,
    Dan_Heng,
    Dan_Heng_IL,
    Dr_Ratio,
    Feixiao,
    Fu_Xuan,
    Gallagher,
    Gepard,
    Guinaifen,
    Hanya,
    Herta,
    Himeko,
    Hook,
    Huohuo,
    Jade,
    Jiaoqiu,
    Jingliu,
    Jing_Yuan,
    Kafka,
    Lingsha,
    Luka,
    Luocha,
    Lynx,
    March_7th,
    Mar7th2,
    Misha,
    Moze,
    Natasha,
    Pela,
    Trailblazer_P,
    Trailblazer_F,
    Trailblazer_I,
    Qingque,
    Robin,
    Ruan_Mei,
    Sam,
    Sampo,
    Seele,
    Serval,
    Silver_Wolf,
    Sparkle,
    Sushang,
    Tingyun,
    Topaz,
    Welt,
    Xueyi,
    Yanqing,
    Yukong,
    Yunli,
}

impl ToString for UnitKind {
    fn to_string(&self) -> String {
        format!("{:?}", self).replace("_", " ")
    }
}

impl Hash for UnitKind {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

impl UnitKind {
    pub fn file_name(&self) -> String {
        match self {
            Self::Trailblazer_F => "Trailblazer_F_F".to_owned(),
            Self::Trailblazer_P => "Trailblazer_F_P".to_owned(),
            Self::Trailblazer_I => "Trailblazer_F_I".to_owned(),
            _ => self.to_string().replace(" ", "_"),
        }
    }

    pub fn from_str(str: &str) -> Option<UnitKind> {
        match &str.to_lowercase()[..] {
            "argenti" => Some(Self::Argenti),
            "arlan" => Some(Self::Arlan),
            "asta" => Some(Self::Asta),
            "bailu" => Some(Self::Bailu),
            "black swan" => Some(Self::Black_Swan),
            "blade" => Some(Self::Blade),
            "bronya" => Some(Self::Bronya),
            "clara" => Some(Self::Clara),
            "dan heng" => Some(Self::Dan_Heng),
            "dan heng il" => Some(Self::Dan_Heng_IL),
            "dr. ratio" => Some(Self::Dr_Ratio),
            "fu xuan" => Some(Self::Fu_Xuan),
            "gepard" => Some(Self::Gepard),
            "guinaifen" => Some(Self::Guinaifen),
            "hanya" => Some(Self::Hanya),
            "herta" => Some(Self::Herta),
            "himeko" => Some(Self::Himeko),
            "hook" => Some(Self::Hook),
            "huohuo" => Some(Self::Huohuo),
            "jingliu" => Some(Self::Jingliu),
            "jing yuan" => Some(Self::Jing_Yuan),
            "kafka" => Some(Self::Kafka),
            "luka" => Some(Self::Luka),
            "luocha" => Some(Self::Luocha),
            "lynx" => Some(Self::Lynx),
            "march 7th" => Some(Self::March_7th),
            "misha" => Some(Self::Misha),
            "natasha" => Some(Self::Natasha),
            "pela" => Some(Self::Pela),
            "trailblazer_P" => Some(Self::Trailblazer_P),
            "trailblazer_F" => Some(Self::Trailblazer_F),
            "trailblazerdestruction" => Some(Self::Trailblazer_P),
            "trailblazerpreservation" => Some(Self::Trailblazer_F),
            "trailblazerharmony" => Some(Self::Trailblazer_I),
            "qingque" => Some(Self::Qingque),
            "ruan mei" => Some(Self::Ruan_Mei),
            "sampo" => Some(Self::Sampo),
            "seele" => Some(Self::Seele),
            "serval" => Some(Self::Serval),
            "silver wolf" => Some(Self::Silver_Wolf),
            "sparkle" => Some(Self::Sparkle),
            "sushang" => Some(Self::Sushang),
            "tingyun" => Some(Self::Tingyun),
            "yingyun" => Some(Self::Tingyun),
            "yopaz" => Some(Self::Topaz),
            "welt" => Some(Self::Welt),
            "xueyi" => Some(Self::Xueyi),
            "yanqing" => Some(Self::Yanqing),
            "yukong" => Some(Self::Yukong),
            _ => None,
        }
    }

    pub fn get_modifiers(&self) -> Box<dyn Fn(&Unit) -> Vec<ModifierOrDOT>> {
        match self {
            Self::Acheron => Box::new(acheron::modifiers),
            Self::Argenti => Box::new(argenti::modifiers),
            Self::Arlan => Box::new(arlan::modifiers),
            Self::Asta => Box::new(asta::modifiers),
            Self::Aventurine => Box::new(aventurine::modifiers),
            Self::Bailu => Box::new(bailu::modifiers),
            Self::Black_Swan => Box::new(black_swan::modifiers),
            Self::Blade => Box::new(blade::modifiers),
            Self::Boothill => Box::new(boothill::modifiers),
            Self::Bronya => Box::new(bronya::modifiers),
            Self::Clara => Box::new(clara::modifiers),
            Self::Dan_Heng => Box::new(dan_heng::modifiers),
            Self::Dan_Heng_IL => Box::new(dan_heng_il::modifiers),
            Self::Dr_Ratio => Box::new(dr_ratio::modifiers),
            Self::Feixiao => Box::new(feixiao::modifiers),
            Self::Fu_Xuan => Box::new(fu_xuan::modifiers),
            Self::Gallagher => Box::new(gallagher::modifiers),
            Self::Gepard => Box::new(gepard::modifiers),
            Self::Guinaifen => Box::new(guinaifen::modifiers),
            Self::Hanya => Box::new(hanya::modifiers),
            Self::Herta => Box::new(herta::modifiers),
            Self::Himeko => Box::new(himeko::modifiers),
            Self::Hook => Box::new(hook::modifiers),
            Self::Huohuo => Box::new(huohuo::modifiers),
            Self::Jade => Box::new(jade::modifiers),
            Self::Jiaoqiu => Box::new(jiaoqiu::modifiers),
            Self::Jingliu => Box::new(jingliu::modifiers),
            Self::Jing_Yuan => Box::new(jing_yuan::modifiers),
            Self::Kafka => Box::new(kafka::modifiers),
            Self::Lingsha => Box::new(lingsha::modifiers),
            Self::Luka => Box::new(luka::modifiers),
            Self::Luocha => Box::new(luocha::modifiers),
            Self::Lynx => Box::new(lynx::modifiers),
            Self::March_7th => Box::new(march_7th::modifiers),
            Self::Mar7th2 => Box::new(mar7th2::modifiers),
            Self::Misha => Box::new(misha::modifiers),
            Self::Moze => Box::new(moze::modifiers),
            Self::Natasha => Box::new(natasha::modifiers),
            Self::Pela => Box::new(pela::modifiers),
            Self::Trailblazer_P => Box::new(trailblazer_p::modifiers),
            Self::Trailblazer_F => Box::new(trailblazer_f::modifiers),
            Self::Trailblazer_I => Box::new(trailblazer_i::modifiers),
            Self::Qingque => Box::new(qingque::modifiers),
            Self::Robin => Box::new(robin::modifiers),
            Self::Ruan_Mei => Box::new(ruan_mei::modifiers),
            Self::Sam => Box::new(sam::modifiers),
            Self::Sampo => Box::new(sampo::modifiers),
            Self::Seele => Box::new(seele::modifiers),
            Self::Serval => Box::new(serval::modifiers),
            Self::Silver_Wolf => Box::new(silver_wolf::modifiers),
            Self::Sparkle => Box::new(sparkle::modifiers),
            Self::Sushang => Box::new(sushang::modifiers),
            Self::Tingyun => Box::new(tingyun::modifiers),
            Self::Topaz => Box::new(topaz::modifiers),
            Self::Welt => Box::new(welt::modifiers),
            Self::Xueyi => Box::new(xueyi::modifiers),
            Self::Yanqing => Box::new(yanqing::modifiers),
            Self::Yukong => Box::new(yukong::modifiers),
            Self::Yunli => Box::new(yunli::modifiers),
        }
    }

    pub fn get_optimization_targets(&self) -> Option<Vec<OptimizationTarget>> {
        match self {
            UnitKind::Qingque => Some(qingque::optimization_targets()),
            _ => None,
        }
    }
}

#[derive(Enum, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum BaseStat {
    Hp,
    Atk,
    Def,
    Spd,
}

impl ToString for BaseStat {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_uppercase()
    }
}

pub type BonusDMGFlag = u8;

#[derive(Enum, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum AdvancedStat {
    CritRate,
    CritDamage,
    CritDamageCond(BonusDMGFlag),
    CritDamageReceived(BonusDMGFlag),
    BreakEffect,
    OutgoingHealingBoost,
    MaxEnergy,
    EnergyRegenerationRate,
    EffectHitRate,
    EffectRes,
    TotalDmgBoost(BonusDMGFlag),
    TotalDmgReceived(BonusDMGFlag),
    ElemDmgBoost(basics::Element),
    ElemDmgRes(basics::Element),
    ElemDmgReceived(basics::Element),
    TotalResPen,
    TotalDmgRes,
    DefIgnore,
    Vulnerability,
}

impl ToString for AdvancedStat {
    fn to_string(&self) -> String {
        match self.clone() {
            Self::CritRate => "Crit Rate".to_owned(),
            Self::CritDamage => "Crit DMG".to_owned(),
            Self::CritDamageCond(bonus_dmg_flag) => format!(
                "Crit DMG from {}",
                AdvancedStat::bonus_dmg_flag_to_str(bonus_dmg_flag)
            ),
            Self::CritDamageReceived(bonus_dmg_flag) => format!(
                "Crit DMG received from {}",
                AdvancedStat::bonus_dmg_flag_to_str(bonus_dmg_flag)
            ),
            Self::BreakEffect => "Break Effect".to_owned(),
            Self::OutgoingHealingBoost => "Outgoing Healing".to_owned(),
            Self::MaxEnergy => "Max Energy".to_owned(),
            Self::EnergyRegenerationRate => "Energy Regeneration Rate".to_owned(),
            Self::EffectHitRate => "Effect Hit Rate".to_owned(),
            Self::EffectRes => "Effect RES".to_owned(),
            Self::TotalDmgBoost(bonus_dmg_flag) => format!(
                "Total DMG boost from {}",
                AdvancedStat::bonus_dmg_flag_to_str(bonus_dmg_flag)
            ),
            Self::TotalDmgReceived(bonus_dmg_flag) => format!(
                "Total DMG received from {}",
                AdvancedStat::bonus_dmg_flag_to_str(bonus_dmg_flag)
            ),
            Self::ElemDmgBoost(element) => format!("{} DMG boost", element.to_str()),
            Self::ElemDmgRes(element) => format!("{} DMG RES", element.to_str()),
            Self::ElemDmgReceived(element) => format!("{} DMG received", element.to_str()),
            Self::TotalResPen => "RES penetration".to_owned(),
            Self::TotalDmgRes => "DMG RES".to_owned(),
            Self::DefIgnore => "DEF ignore".to_owned(),
            Self::Vulnerability => "Vulnerability".to_owned(),
        }
    }
}

impl AdvancedStat {
    // pass 1 if true, 0 if false!
    #[inline(always)]
    pub fn create_dmg_bonus_flag(
        is_basic_buffed: BonusDMGFlag,
        is_ult_buffed: BonusDMGFlag,
        is_skill_buffed: BonusDMGFlag,
        is_dot_buffed: BonusDMGFlag,
        is_fua_buffed: BonusDMGFlag,
    ) -> BonusDMGFlag {
        is_basic_buffed
            | is_ult_buffed << 1
            | is_skill_buffed << 2
            | is_dot_buffed << 3
            | is_fua_buffed << 4
    }

    #[inline(always)]
    pub fn is_basic_buff(flag: BonusDMGFlag) -> bool {
        flag & 1 > 0
    }

    #[inline(always)]
    pub fn is_ult_buff(flag: BonusDMGFlag) -> bool {
        flag & 1 > 0
    }

    #[inline(always)]
    pub fn is_skill_buff(flag: BonusDMGFlag) -> bool {
        flag & 1 > 0
    }

    #[inline(always)]
    pub fn is_dot_buff(flag: BonusDMGFlag) -> bool {
        flag & 1 > 0
    }

    #[inline(always)]
    pub fn is_fua_buff(flag: BonusDMGFlag) -> bool {
        flag & 1 > 0
    }

    fn bonus_dmg_flag_to_str(flag: BonusDMGFlag) -> String {
        let mut result = "".to_owned();
        let mut non_empty = false;
        let mut curr_pos = 0;
        let bit_count = flag.count_ones();
        if Self::is_basic_buff(flag) {
            curr_pos += 1;
            result += "Basic Attack";
            non_empty = true;
        }
        if Self::is_ult_buff(flag) {
            curr_pos += 1;
            if non_empty {
                if curr_pos < bit_count {
                    result += ", "
                } else {
                    result += " and ";
                }
            }
            result += "Ultimate";
            non_empty = true;
        }
        if Self::is_skill_buff(flag) {
            curr_pos += 1;
            if non_empty {
                if curr_pos < bit_count {
                    result += ", "
                } else {
                    result += " and ";
                }
            }
            result += "Skill";
            non_empty = true;
        }
        if Self::is_dot_buff(flag) {
            curr_pos += 1;
            if non_empty {
                if curr_pos < bit_count {
                    result += ", "
                } else {
                    result += " and ";
                }
            }
            result += "DOT";
            non_empty = true;
        }
        if Self::is_fua_buff(flag) {
            curr_pos += 1;
            if non_empty {
                if curr_pos < bit_count {
                    result += ", "
                } else {
                    result += " and ";
                }
            }
            result += "Follow-up Attack";
        };
        result
    }
}

#[derive(Clone)]
pub struct Unit {
    pub kind: UnitKind,
    pub path: Path,
    pub rarity: u8,
    pub element: Element,
    pub trait_buffs: [(Stat, f32); 10],
    pub skills: Vec<SkillData>,
    base_stats: [[f32; BaseStat::LENGTH]; 7],
    base_stats_growth: [f32; BaseStat::LENGTH],
    advanced_stats: enum_map::EnumMap<AdvancedStat, f32>,
    pub unique_data: UniqueData,
    pub buff_sources: Vec<(UnitKind, Source)>,
    pub dynamic_buffs: Vec<Modifier>,
    pub bonus_base_stats: [f32; BaseStat::LENGTH],
    pub bonus_advanced_stat: enum_map::EnumMap<AdvancedStat, f32>,
    weaknesses: [bool; Element::LENGTH],
    relic_opt_base: [f32; BaseStat::LENGTH],
    relic_opt_adv: enum_map::EnumMap<AdvancedStat, f32>,
    opt_base_stats: [f32; BaseStat::LENGTH],
    relic_set_modifiers: Vec<ModifierData>,
}

impl Hash for Unit {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SkillData {
    pub name: String,
    pub icon_path: String,
    pub description: String,
    pub skill_type: Option<SkillType>,
    pub kind: Option<SkillKind>,
    pub break_: ToughnessReduction,
    pub energy: EnergyRegen,
    pub ultimate_cost: u32,
    parameters: Vec<Vec<f32>>,
}

impl SkillData {
    pub fn new(
        name: &str,
        icon_path: &str,
        description: &str,
        skill_type: Option<SkillType>,
        kind: Option<SkillKind>,
        break_: ToughnessReduction,
        energy: EnergyRegen,
        ultimate_cost: u32,
        parameters: Vec<Vec<f32>>,
    ) -> Self {
        SkillData {
            name: name.to_owned(),
            icon_path: icon_path.to_owned(),
            description: description.to_owned(),
            skill_type,
            kind,
            break_,
            energy,
            ultimate_cost,
            parameters,
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum SkillType {
    SingleAttack,
    Blast,
    AoE,
    Bounce,
    Support,
    Enhance,
    Defense,
    Restore,
    Impair,
}

impl SkillType {
    pub fn try_from_str(s: &str) -> Option<Self> {
        match s {
            "Single_Attack" => Some(SkillType::SingleAttack),
            "Blast" => Some(SkillType::Blast),
            "AoE" => Some(SkillType::AoE),
            "Bounce" => Some(SkillType::Bounce),
            "Support" => Some(SkillType::Support),
            "Enhance" => Some(SkillType::Enhance),
            "Defense" => Some(SkillType::Defense),
            "Restore" => Some(SkillType::Restore),
            "Impair" => Some(SkillType::Impair),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Default)]
pub struct ToughnessReduction {
    pub standard: u32,
    pub hit: u32,
    pub subsequent: u32,
    pub adjacent: u32,
}

impl ToughnessReduction {
    pub const fn new(standard: u32, hit: u32, subsequent: u32, adjacent: u32) -> Self {
        ToughnessReduction {
            standard,
            hit,
            subsequent,
            adjacent,
        }
    }
}

impl FromStr for ToughnessReduction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut standard, mut hit, mut subsequent, mut adjacent) = (0, 0, 0, 0);
        s.split(" + ").for_each(|break_str| {
            if !break_str.contains("/") {
                standard = break_str.parse().unwrap();
            } else {
                if let Some((value, t)) = break_str.split("/").next_tuple() {
                    match t {
                        "hit" => hit = value.parse().unwrap(),
                        "subsequent" => subsequent = value.parse().unwrap(),
                        "adjacent" => adjacent = value.parse().unwrap(),
                        _ => panic!("oopsie"),
                    }
                } else {
                    panic!("ioioioio, unexpected ToughnessReduction string when parsing")
                }
            }
        });
        Ok(ToughnessReduction::new(standard, hit, subsequent, adjacent))
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Default)]
pub struct EnergyRegen {
    pub standard: u32,
    pub hit: u32,
    pub subsequent: u32,
    pub adjacent: u32,
}

impl EnergyRegen {
    pub const fn new(standard: u32, hit: u32, subsequent: u32, adjacent: u32) -> Self {
        EnergyRegen {
            standard,
            hit,
            subsequent,
            adjacent,
        }
    }
}

impl FromStr for EnergyRegen {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut standard, mut hit, mut subsequent, mut adjacent) = (0, 0, 0, 0);
        s.split(" + ").for_each(|break_str| {
            if !break_str.contains("/") {
                standard = break_str.parse().unwrap();
            } else {
                if let Some((value, t)) = break_str.split("/").next_tuple() {
                    match t {
                        "hit" => hit = value.parse().unwrap(),
                        "subsequent" => subsequent = value.parse().unwrap(),
                        "adjacent" => adjacent = value.parse().unwrap(),
                        _ => panic!("oopsie"),
                    }
                } else {
                    panic!("ioioioio, unexpected EnergyRegen string when parsing")
                }
            }
        });
        Ok(EnergyRegen::new(standard, hit, subsequent, adjacent))
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum SkillKind {
    BasicAttack,
    Skill,
    Ultimate,
    Technique,
    Talent,
    Trace(u8),
}

impl SkillKind {
    pub fn try_from_source(s: &Source) -> Option<Self> {
        match s {
            Source::Ultimate => Some(Self::Ultimate),
            Source::Skill => Some(Self::Skill),
            Source::Technique => Some(Self::Technique),
            Source::Talent => Some(Self::Talent),
            Source::Trace(i) => Some(Self::Trace(*i)),
            _ => None,
        }
    }
}

impl FromStr for SkillKind {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "basic atk" => Ok(SkillKind::BasicAttack),
            "skill" => Ok(SkillKind::Skill),
            "ultimate" => Ok(SkillKind::Ultimate),
            "technique" => Ok(SkillKind::Technique),
            "talent" => Ok(SkillKind::Talent),
            _ => Err(()),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy)]
pub struct UniqueData {
    pub level: u32,
    pub ascension: u32,
    pub relics: [Option<usize>; 6],
    pub light_cone: Option<usize>,
    pub ultimate_level: u8,
    pub skill_level: u8,
    pub attack_level: u8,
    pub talent_level: u8,
    pub team: [Option<UnitKind>; 3],
    pub eidolon: u8,
    #[serde(
        skip_serializing,
        default = "default_optimize_relics",
        skip_deserializing
    )]
    optimize_relics: [Option<Relic>; 6],
    optimize_state: bool,
}

fn default_optimize_relics() -> [Option<Relic>; 6] {
    [None; 6]
}

#[derive(Clone, Copy)]
pub enum ConfigType {
    Stacks(u32),
    StacksWithFlag(u32),
    Flag,
}

impl ConfigType {
    fn get_options(&self) -> ConfigUI {
        match self {
            Self::Stacks(x) => ConfigUI::NumberField(0, *x),
            Self::StacksWithFlag(x) => ConfigUI::TickBoxWithNumberField(0, *x),
            Self::Flag => ConfigUI::TickBox,
        }
    }
}

#[derive(Clone, Copy)]
enum ConfigValue {
    Number(usize),
    NumberWithBool(usize, bool),
    Bool(bool),
}

#[derive(Clone, Copy)]
pub struct ModifierConfig {
    config_type: ConfigType,
    config_value: ConfigValue,
}

enum ConfigUI {
    Dropdown(Vec<WidgetText>),
    NumberField(u32, u32),
    TickBox,
    TickBoxWithNumberField(u32, u32),
}

impl ModifierConfig {
    pub fn new(config_type: ConfigType) -> Self {
        //TODO: proper config values etc
        ModifierConfig {
            config_type,
            config_value: ConfigValue::Number(0),
        }
    }

    pub fn get_options(&self) -> ConfigUI {
        self.config_type.get_options()
    }
    pub fn get_config(&self) -> ConfigType {
        self.config_type
    }
    pub fn get_index(&self) -> usize {
        match self.config_value {
            ConfigValue::Number(x) => x,
            ConfigValue::NumberWithBool(x, _) => x,
            _ => panic!("ModifierConfig::get_index cmon bruh"),
        }
    }

    fn get_number_with_bool(&self) -> (usize, bool) {
        if let ConfigValue::NumberWithBool(x, b) = self.config_value {
            (x, b)
        } else {
            panic!(":(")
        }
    }

    pub fn get_bool(&self) -> bool {
        match self.config_value {
            ConfigValue::Bool(b) => b,
            ConfigValue::NumberWithBool(_, b) => b,
            _ => panic!("ModifierConfig::get_bool zzzzzz"),
        }
    }

    pub fn set_value(&mut self, value: ConfigValue) {
        self.config_value = value;
    }
}

#[derive(Clone)]
pub struct ModifierData {
    pub target: ModifierTarget,
    pub stat: Stat,
    pub scaling: BuffScaling,
    pub is_dimension: bool,
    value: fn(
        &Unit,
        &Unit,
        Option<ModifierConfig>,
        &[Option<&Unit>; 4],
        &LightConesStore,
        &RelicsStore,
    ) -> f32,
}

impl ModifierData {
    pub fn value(
        &self,
        target: &Unit,
        buffer: &Unit,
        config: Option<ModifierConfig>,
        team: &[Option<&Unit>; 4],
        lc_store: &LightConesStore,
        relic_store: &RelicsStore,
    ) -> f32 {
        (self.value)(target, buffer, config, team, lc_store, relic_store)
    }

    pub fn new(
        target: ModifierTarget,
        stat: Stat,
        scaling: BuffScaling,
        value: fn(
            &Unit,
            &Unit,
            Option<ModifierConfig>,
            &[Option<&Unit>; 4],
            &LightConesStore,
            &RelicsStore,
        ) -> f32,
    ) -> ModifierData {
        ModifierData {
            target,
            stat,
            scaling,
            value,
            is_dimension: false,
        }
    }

    pub fn new_dimension(
        target: ModifierTarget,
        stat: Stat,
        scaling: BuffScaling,
        value: fn(
            &Unit,
            &Unit,
            Option<ModifierConfig>,
            &[Option<&Unit>; 4],
            &LightConesStore,
            &RelicsStore,
        ) -> f32,
    ) -> ModifierData {
        ModifierData {
            target,
            stat,
            scaling,
            value,
            is_dimension: true,
        }
    }
}

#[derive(Clone)]
pub enum Source {
    Ultimate,
    Skill,
    Trace(u8),
    Technique,
    Eidolon(u8),
    LightCone,
    Talent,
    Relic2p,
    Relic4p,
}

pub enum ModifierOrDOT {
    Modifier(Modifier),
    DOT(DOTKind),
}

pub enum DOTKind {
    Standard(Element, f32),
    Arcana(u32),
}

#[derive(Clone)]
pub struct Modifier {
    pub source: (UnitKind, Source),
    pub data: Vec<ModifierData>,
    pub config: Option<ModifierConfig>,
    pub snapshot: bool,
}

impl Modifier {
    pub fn new(source: (UnitKind, Source), data: Vec<ModifierData>, snapshot: bool) -> Self {
        Self {
            source,
            data,
            config: None,
            snapshot,
        }
    }

    pub fn new_with_config(
        source: (UnitKind, Source),
        data: Vec<ModifierData>,
        config: Option<ModifierConfig>,
        snapshot: bool,
    ) -> Self {
        Self {
            source,
            data,
            config,
            snapshot,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ModifierTarget {
    Ally,
    Allies,
    Team,
    Caster,
    Enemy,
    Enemies,
}

#[derive(Enum, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Stat {
    Base(BaseStat),
    Advanced(AdvancedStat),
}

impl ToString for Stat {
    fn to_string(&self) -> String {
        match self {
            Stat::Base(baseStat) => baseStat.to_string(),
            Stat::Advanced(advanced_stat) => advanced_stat.to_string(),
        }
    }
}

impl FromStr for Stat {
    type Err = String;
    fn from_str(input: &str) -> Result<Stat, Self::Err> {
        match input.to_lowercase().as_str() {
            "atk" => Ok(Stat::Base(BaseStat::Atk)),
            "def" => Ok(Stat::Base(BaseStat::Def)),
            "hp" => Ok(Stat::Base(BaseStat::Spd)),
            "spd" => Ok(Stat::Base(BaseStat::Spd)),
            "critrate" => Ok(Stat::Advanced(AdvancedStat::CritRate)),
            "critdmg" => Ok(Stat::Advanced(AdvancedStat::CritDamage)),
            "break" => Ok(Stat::Advanced(AdvancedStat::BreakEffect)),
            "outgoinghealingboost" => Ok(Stat::Advanced(AdvancedStat::OutgoingHealingBoost)),
            "maxenergy" => Ok(Stat::Advanced(AdvancedStat::MaxEnergy)),
            "energyregenerationRate" => Ok(Stat::Advanced(AdvancedStat::EnergyRegenerationRate)),
            "effecthitrate" => Ok(Stat::Advanced(AdvancedStat::EffectHitRate)),
            "effectres" => Ok(Stat::Advanced(AdvancedStat::EffectRes)),
            "lightning" => Ok(Stat::Advanced(AdvancedStat::ElemDmgBoost(
                Element::Lightning,
            ))),
            "fire" => Ok(Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Fire))),
            "ice" => Ok(Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice))),
            "quantum" => Ok(Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Quantum))),
            "imaginary" => Ok(Stat::Advanced(AdvancedStat::ElemDmgBoost(
                Element::Imaginary,
            ))),
            "wind" => Ok(Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Wind))),
            "physical" => Ok(Stat::Advanced(AdvancedStat::ElemDmgBoost(
                Element::Physical,
            ))),
            "totalrespen" => Ok(Stat::Advanced(AdvancedStat::TotalResPen)),
            "totaldmgres" => Ok(Stat::Advanced(AdvancedStat::TotalDmgRes)),
            "defignore" => Ok(Stat::Advanced(AdvancedStat::DefIgnore)),
            "vulnerability" => Ok(Stat::Advanced(AdvancedStat::Vulnerability)),
            _ => Err(format!("oppsie, {} not found", input)),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum BuffScaling {
    Additive,
    Multiplicative,
}

pub struct OptimizationTarget {
    name: String,
    value: fn(&Unit, &[Option<&Unit>; 3], &RelicsStore, &LightConesStore) -> (f32, f32, f32),
}

impl OptimizationTarget {
    pub fn dmg(
        &self,
        unit: &Unit,
        team: &[Option<&Unit>; 3],
        relics_store: &RelicsStore,
        lc_store: &LightConesStore,
    ) -> (f32, f32, f32) {
        (self.value)(unit, team, relics_store, lc_store)
    }
}

impl Default for UniqueData {
    fn default() -> Self {
        UniqueData {
            level: 80,
            ascension: 6,
            relics: [None; 6],
            light_cone: Some(0),
            ultimate_level: 1,
            skill_level: 1,
            attack_level: 1,
            talent_level: 1,
            eidolon: 0,
            team: [None; 3],
            optimize_relics: [None; 6],
            optimize_state: false,
        }
    }
}

impl UniqueData {
    pub fn update_relics(&mut self, relic_id: usize, relic_part: RelicPart) {
        self.relics[relic_part.get_index()] = Some(relic_id)
    }

    pub fn update_opt_relic(&mut self, relic: &Relic) {
        self.optimize_relics[relic.part.get_index()] = Some(relic.clone());
    }

    pub fn equip_opt_set(&mut self, relics: &[Option<Relic>; 6]) {
        self.optimize_relics = *relics;
    }

    pub fn max_level(&self) -> u32 {
        self.ascension * 10 + 20
    }

    pub fn min_level(&self) -> u32 {
        if self.ascension == 0 {
            1
        } else {
            self.ascension * 10 + 10
        }
    }

    pub fn new(
        level: u32,
        ascension: u32,
        ultimate_level: u8,
        skill_level: u8,
        attack_level: u8,
        talent_level: u8,
        eidolon: u8,
    ) -> Self {
        UniqueData {
            level,
            ascension,
            relics: [None; 6],
            light_cone: None,
            ultimate_level,
            skill_level,
            attack_level,
            talent_level,
            eidolon,
            team: [None; 3],
            optimize_relics: [None; 6],
            optimize_state: false,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ParsedUnitFile {
    path: Path,
    element: Element,
    rarity: u8,
    base_stats: Vec<[f32; 4]>,
    base_stats_growth: [f32; 4],
    trait_buffs: Vec<(Stat, f32)>,
    skills: Vec<SkillData>,
}

impl Unit {
    pub fn from_data(
        unit_kind: UnitKind,
        unique_data: UniqueData,
        parsed_data: &ParsedUnitFile,
    ) -> Unit {
        Unit {
            kind: unit_kind,
            path: parsed_data.path,
            rarity: parsed_data.rarity,
            element: parsed_data.element,
            trait_buffs: parsed_data.trait_buffs.clone().try_into().unwrap(),
            base_stats: parsed_data.base_stats.clone().try_into().unwrap(),
            base_stats_growth: parsed_data.base_stats_growth,
            skills: parsed_data.skills.clone(),
            advanced_stats: enum_map! {_ => 0.0},
            unique_data,
            buff_sources: Vec::new(),
            dynamic_buffs: Vec::new(),
            bonus_base_stats: [0.; BaseStat::LENGTH],
            bonus_advanced_stat: enum_map::EnumMap::from_fn(|_| 0.),
            weaknesses: [false; Element::LENGTH],
            relic_opt_base: [0.; BaseStat::LENGTH],
            relic_opt_adv: enum_map! {_ => 0.},
            opt_base_stats: [0.; BaseStat::LENGTH],
            relic_set_modifiers: Vec::new(),
        }
    }

    pub fn parse_character_info(character_info: Vec<u8>) -> ParsedUnitFile {
        let mut iter: Lines<&[u8]> = character_info.lines();
        iter.next(); // ignore first line with name
        let path = Path::from_str(&iter.next().unwrap().unwrap()).unwrap();
        let element = Element::from_str(&iter.next().unwrap().unwrap()).unwrap();
        let rarity = u8::from_str_radix(&iter.next().unwrap().unwrap(), 10).unwrap();
        let mut base_stats: Vec<[f32; 4]> = Vec::new();
        for _ in 0..7 {
            base_stats.push([
                iter.next().unwrap().unwrap().parse().unwrap(),
                iter.next().unwrap().unwrap().parse().unwrap(),
                iter.next().unwrap().unwrap().parse().unwrap(),
                iter.next().unwrap().unwrap().parse().unwrap(),
            ]);
        }

        let base_stats_growth = [
            iter.next().unwrap().unwrap().parse().unwrap(),
            iter.next().unwrap().unwrap().parse().unwrap(),
            iter.next().unwrap().unwrap().parse().unwrap(),
            iter.next().unwrap().unwrap().parse().unwrap(),
        ];

        let mut trait_buffs: Vec<(Stat, f32)> = Vec::new();
        for _ in 0..10 {
            let line_unwrapped = iter.next().unwrap().unwrap();
            let mut words = line_unwrapped.split(' ');
            let stat = Stat::from_str(words.next().unwrap()).unwrap();
            trait_buffs.push((stat, words.next().unwrap().parse().unwrap()))
        }

        // if somehow there are less than 3 embedded skills we are doomed and have to update the converter
        let embedded_skills_num = iter.next().unwrap().unwrap().parse().unwrap();
        let mut embedded_skills: Vec<SkillData> = Vec::new();
        for i in 0..embedded_skills_num {
            embedded_skills.push(SkillData::new(
                &iter.next().unwrap().unwrap(),
                &iter.next().unwrap().unwrap(),
                &iter.next().unwrap().unwrap(),
                None,
                Some(SkillKind::Trace(i + 1)),
                Default::default(),
                Default::default(),
                0,
                {
                    iter.next();
                    iter.next()
                        .map(|line| {
                            let line_str = line.unwrap();
                            if line_str.is_empty() {
                                return Vec::new();
                            }

                            let mut result = Vec::new();
                            result.push(line_str.split(' ').map(|v| v.parse().unwrap()).collect());
                            result
                        })
                        .unwrap()
                },
            ));
        }

        let _ = iter.next();
        let skills_num = iter.next().unwrap().unwrap().parse().unwrap();
        let skills = (0..skills_num)
            .map(|_| {
                SkillData::new(
                    &iter.next().unwrap().unwrap(),
                    &iter.next().unwrap().unwrap(),
                    &iter.next().unwrap().unwrap(),
                    SkillType::try_from_str(&iter.next().unwrap().unwrap()),
                    SkillKind::from_str(&iter.next().unwrap().unwrap())
                        .map_or(None, |skill_kind| Some(skill_kind)),
                    ToughnessReduction::from_str(&iter.next().unwrap().unwrap()).unwrap(),
                    EnergyRegen::from_str(&iter.next().unwrap().unwrap()).unwrap(),
                    iter.next().unwrap().unwrap().parse().unwrap(),
                    Self::parse_params(&mut iter),
                )
            })
            .chain(embedded_skills)
            .collect();

        ParsedUnitFile {
            path,
            element,
            rarity,
            base_stats,
            base_stats_growth,
            trait_buffs,
            skills,
        }
    }

    fn parse_embedded_skill_params(lines: &mut Lines<&[u8]>) -> Vec<Vec<f32>> {
        let i: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let mut result: Vec<Vec<f32>> = Vec::new();
        result.push(
            lines
                .next()
                .map(|line| {
                    let line_str = line.unwrap();
                    if line_str.is_empty() {
                        return Vec::new();
                    }
                    line_str.split(' ').map(|v| v.parse().unwrap()).collect()
                })
                .unwrap(),
        );

        result
    }

    fn parse_params(lines: &mut Lines<&[u8]>) -> Vec<Vec<f32>> {
        let i: usize = lines
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .next()
            .unwrap()
            .parse()
            .unwrap();

        lines
            .take(i)
            .map(|line| {
                line.unwrap()
                    .split(' ')
                    .map(|v| v.parse().unwrap())
                    .collect()
            })
            .collect()
    }

    pub fn from_unique_data2(character: UnitKind, unique_data: UniqueData) -> Unit {
        if CHARACTER_INFO[character as usize].0 != character {
            panic!(
                "tried to retrieve statistics of {:?}, got {:?}",
                character, CHARACTER_INFO[character as usize].0
            );
        }

        let index = character as usize;
        Unit {
            kind: character,
            path: CHARACTER_INFO[index].1,
            rarity: CHARACTER_INFO[index].2.rarity,
            element: Element::Quantum, //TODO: get element data
            trait_buffs: CHARACTER_INFO[index].3,
            base_stats: CHARACTER_INFO[index].2.base,
            base_stats_growth: CHARACTER_INFO[index].2.growth,
            skills: CHARACTER_INFO[index]
                .4
                .clone()
                .into_iter()
                .filter_map(identity)
                .collect_vec(),
            advanced_stats: enum_map! {_ => 0.0},
            unique_data,
            buff_sources: Vec::new(),
            dynamic_buffs: Vec::new(),
            bonus_base_stats: [0.; BaseStat::LENGTH],
            bonus_advanced_stat: enum_map::EnumMap::from_fn(|_| 0.),
            weaknesses: [false; Element::LENGTH],
            relic_opt_base: [0.; BaseStat::LENGTH],
            relic_opt_adv: enum_map! {_ => 0.},
            opt_base_stats: [0.; BaseStat::LENGTH],
            relic_set_modifiers: Vec::new(),
        }
    }

    pub fn update_relics(&mut self, relic_id: usize, relic_part: RelicPart) {
        self.unique_data.update_relics(relic_id, relic_part)
    }

    pub fn is_weak_to(&self, elem: Element) -> bool {
        self.weaknesses[elem as usize]
    }

    pub fn reset_set_effects(&mut self) {
        self.relic_set_modifiers.clear();
    }

    pub fn add_set_effects<I>(&mut self, modifiers: I)
    where
        I: IntoIterator<Item = ModifierData>,
    {
        self.relic_set_modifiers.extend(modifiers)
    }

    pub fn update_opt_relic(&mut self, relic: &Relic) {
        if let Some(old_relic) = self.unique_data.optimize_relics[relic.part.get_index()] {
            for (stat, value, is_flat) in old_relic.opt_sub {
                self.reduce_relic_opt_stat(stat, value, is_flat);
            }

            self.reduce_relic_opt_stat(
                old_relic.opt_main.0,
                old_relic.opt_main.1,
                old_relic.opt_main.2,
            );
        }

        for (stat, value, is_flat) in relic.opt_sub {
            self.add_relic_opt_stat(stat, value, is_flat);
        }

        self.add_relic_opt_stat(relic.opt_main.0, relic.opt_main.1, relic.opt_main.2);

        self.unique_data.update_opt_relic(relic);
    }

    fn reduce_relic_opt_stat(&mut self, stat: Stat, value: f32, is_flat: bool) {
        match (stat, is_flat) {
            (Stat::Base(stat), true) => self.relic_opt_base[stat as usize] -= value,
            (Stat::Base(stat), false) => {
                self.relic_opt_base[stat as usize] -= value * self.opt_base_stats[stat as usize]
            }
            (Stat::Advanced(stat), _) => self.relic_opt_adv[stat] -= value,
        }
    }

    fn add_relic_opt_stat(&mut self, stat: Stat, value: f32, is_flat: bool) {
        match (stat, is_flat) {
            (Stat::Base(stat), true) => self.relic_opt_base[stat as usize] += value,
            (Stat::Base(stat), false) => {
                self.relic_opt_base[stat as usize] += value * self.opt_base_stats[stat as usize]
            }
            (Stat::Advanced(stat), _) => self.relic_opt_adv[stat] += value,
        }
    }

    pub fn equip_opt_set(&mut self, relics: &[Option<Relic>; 6]) {
        self.unique_data.equip_opt_set(relics);
    }

    pub fn max_level(&self) -> u32 {
        self.unique_data.max_level()
    }

    pub fn min_level(&self) -> u32 {
        self.unique_data.min_level()
    }

    pub fn start_optimization(&mut self, lc_store: &LightConesStore) {
        self.unique_data.optimize_state = true;
        self.opt_base_stats = self.base_stats(lc_store);
        self.relic_opt_base = [0.; BaseStat::LENGTH];
        self.relic_opt_adv.clear();
    }

    pub fn end_optimization(&mut self) {
        self.unique_data.optimize_state = false;
    }

    pub fn reset_buffs(&mut self) {
        self.buff_sources.clear();
        self.dynamic_buffs.clear();
        self.bonus_base_stats = [0.; BaseStat::LENGTH];
        self.bonus_advanced_stat.clear();
    }

    pub fn base_stats(&self, light_cones_store: &LightConesStore) -> [f32; BaseStat::LENGTH] {
        let mut base_stats = self.base_stats[self.unique_data.ascension as usize];
        for i in 0..BaseStat::LENGTH {
            base_stats[i] +=
                (self.unique_data.level - self.min_level()) as f32 * self.base_stats_growth[i];
        }

        if self.unique_data.light_cone.is_none() {
            return base_stats;
        }

        let lc = light_cones_store.get(self.unique_data.light_cone.expect("checked above"));

        if let Some(lc) = lc {
            let lc_stats = lc.stats();
            for i in 0..light_cones::BaseStat::LENGTH {
                base_stats[i] += lc_stats[i];
            }
            base_stats
        } else {
            base_stats
        }
    }

    pub fn buff(
        &mut self,
        team: &[Option<&Unit>; 3],
        modifier: Modifier,
        buffer: &Unit,
        light_cones_store: &LightConesStore,
        relics_store: &RelicsStore,
    ) {
        if modifier.snapshot {
            self.buff_sources.push(modifier.source);
            modifier.data.iter().for_each(|data| {
                let team: &[Option<&Unit>; 4] = &[Some(self), team[0], team[1], team[2]];
                match (data.stat, data.scaling) {
                    (Stat::Base(base_stat), BuffScaling::Additive) => {
                        self.bonus_base_stats[base_stat as usize] += data.value(
                            self,
                            buffer,
                            modifier.config,
                            team,
                            light_cones_store,
                            relics_store,
                        )
                    }
                    (Stat::Base(base_stat), BuffScaling::Multiplicative) => {
                        self.bonus_base_stats[base_stat as usize] += self
                            .base_stats(light_cones_store)[base_stat as usize]
                            * data.value(
                                self,
                                buffer,
                                modifier.config,
                                team,
                                light_cones_store,
                                relics_store,
                            )
                    }
                    (Stat::Advanced(advanced_stat), _) => {
                        self.bonus_advanced_stat[advanced_stat] += data.value(
                            self,
                            buffer,
                            modifier.config,
                            team,
                            light_cones_store,
                            relics_store,
                        )
                    }
                }
            })
        } else {
            self.dynamic_buffs.push(modifier);
        }
    }

    pub fn get_effective_stat(
        &self,
        stat: Stat,
        team: &[Option<&Unit>; 4],
        light_cones_store: &LightConesStore,
        relics_store: &RelicsStore,
    ) -> f32 {
        match stat {
            Stat::Base(stat) => {
                self.get_effective_base_stat(stat, team, light_cones_store, relics_store)
            }
            Stat::Advanced(stat) => {
                self.get_effective_advanced_stat(stat, team, light_cones_store, relics_store)
            }
        }
    }

    fn get_effective_base_stat_op(
        &self,
        stat: BaseStat,
        team: &[Option<&Unit>; 4],
        light_cones_store: &LightConesStore,
        relics_store: &RelicsStore,
    ) -> f32 {
        let base_stat = self.opt_base_stats[stat as usize];
        let mut effective_stat = base_stat + self.relic_opt_base[stat as usize];

        self.dynamic_buffs.iter().for_each(|modifier| {
            modifier
                .data
                .iter()
                .filter(|data| matches!(data.stat, Stat::Base(_)))
                .for_each(|data| {
                    let buffer = team
                        .iter()
                        .find_map(|unit_op| {
                            unit_op.and_then(|unit| {
                                if unit.kind == modifier.source.0 {
                                    Some(unit)
                                } else {
                                    None
                                }
                            })
                        })
                        .expect("buffer has to be in the team");

                    match data.scaling {
                        BuffScaling::Additive => {
                            effective_stat += data.value(
                                self,
                                buffer,
                                modifier.config,
                                team,
                                light_cones_store,
                                relics_store,
                            )
                        }
                        BuffScaling::Multiplicative => {
                            effective_stat += base_stat
                                * data.value(
                                    self,
                                    buffer,
                                    modifier.config,
                                    team,
                                    light_cones_store,
                                    relics_store,
                                )
                        }
                    }
                })
        });

        effective_stat + self.bonus_base_stats[stat as usize]
    }

    pub fn get_effective_base_stat(
        &self,
        stat: BaseStat,
        team: &[Option<&Unit>; 4],
        light_cones_store: &LightConesStore,
        relics_store: &RelicsStore,
    ) -> f32 {
        if self.unique_data.optimize_state {
            return self.get_effective_base_stat_op(stat, team, light_cones_store, relics_store);
        }

        let base_stat = self.base_stats(light_cones_store)[stat as usize];
        let mut effective_stat = base_stat;

        self.unique_data
            .relics
            .iter()
            .filter_map(|relic_id_o| {
                relic_id_o.and_then(|relic_id| relics_store.get_relic(relic_id))
            })
            .for_each(|relic| {
                let (main_stat, flat) = relic.main_stat.0.to_buff_stat();
                if matches!(main_stat, Stat::Base(_)) {
                    if flat {
                        effective_stat += relic.main_stat.1;
                    } else {
                        effective_stat += relic.main_stat.1 * base_stat;
                    }
                }

                relic
                    .sub
                    .iter()
                    .filter_map(|stat_o| {
                        stat_o.and_then(|stat| Some((stat.1, stat.0.to_buff_stat())))
                    })
                    .for_each(|(value, (stat, flat))| {
                        if matches!(stat, Stat::Base(_)) {
                            if flat {
                                effective_stat += value;
                            } else {
                                effective_stat += value * base_stat;
                            }
                        }
                    })
            });

        self.dynamic_buffs.iter().for_each(|modifier| {
            modifier
                .data
                .iter()
                .filter(|data| matches!(data.stat, Stat::Base(_)))
                .for_each(|data| {
                    let buffer = team
                        .iter()
                        .find_map(|unit_op| {
                            unit_op.and_then(|unit| {
                                if unit.kind == modifier.source.0 {
                                    Some(unit)
                                } else {
                                    None
                                }
                            })
                        })
                        .expect("buffer has to be in the team");

                    match data.scaling {
                        BuffScaling::Additive => {
                            effective_stat += data.value(
                                self,
                                buffer,
                                modifier.config,
                                team,
                                light_cones_store,
                                relics_store,
                            )
                        }
                        BuffScaling::Multiplicative => {
                            effective_stat += base_stat
                                * data.value(
                                    self,
                                    buffer,
                                    modifier.config,
                                    team,
                                    light_cones_store,
                                    relics_store,
                                )
                        }
                    }
                })
        });

        effective_stat + self.bonus_base_stats[stat as usize]
    }

    fn get_effective_advanced_stat_opt(
        &self,
        stat: AdvancedStat,
        team: &[Option<&Unit>; 4],
        light_cones_store: &LightConesStore,
        relics_store: &RelicsStore,
    ) -> f32 {
        let mut effective_stat = self.relic_opt_adv[stat];

        self.dynamic_buffs.iter().for_each(|modifier| {
            modifier
                .data
                .iter()
                .filter(|data| matches!(data.stat, Stat::Advanced(_)))
                .for_each(|data| {
                    let buffer = team
                        .iter()
                        .find_map(|unit_op| {
                            unit_op.and_then(|unit| {
                                if unit.kind == modifier.source.0 {
                                    Some(unit)
                                } else {
                                    None
                                }
                            })
                        })
                        .expect("buffer has to be in the team");

                    effective_stat += data.value(
                        self,
                        buffer,
                        modifier.config,
                        team,
                        light_cones_store,
                        relics_store,
                    )
                })
        });

        effective_stat + self.bonus_advanced_stat[stat]
    }

    pub fn get_effective_advanced_stat(
        &self,
        stat: AdvancedStat,
        team: &[Option<&Unit>; 4],
        light_cones_store: &LightConesStore,
        relics_store: &RelicsStore,
    ) -> f32 {
        if self.unique_data.optimize_state {
            return self.get_effective_advanced_stat_opt(
                stat,
                team,
                light_cones_store,
                relics_store,
            );
        }

        let mut effective_stat = 0.;

        self.unique_data
            .relics
            .iter()
            .filter_map(|relic_id_o| {
                relic_id_o.and_then(|relic_id| relics_store.get_relic(relic_id))
            })
            .for_each(|relic| {
                let (main_stat, _) = relic.main_stat.0.to_buff_stat();
                if matches!(main_stat, Stat::Advanced(_)) {
                    effective_stat += relic.main_stat.1;
                }

                relic
                    .sub
                    .iter()
                    .filter_map(|stat_o| {
                        stat_o.and_then(|stat| Some((stat.1, stat.0.to_buff_stat())))
                    })
                    .for_each(|(value, (stat, _))| {
                        if matches!(stat, Stat::Advanced(_)) {
                            effective_stat += value;
                        }
                    })
            });

        self.dynamic_buffs.iter().for_each(|modifier| {
            modifier
                .data
                .iter()
                .filter(|data| matches!(data.stat, Stat::Advanced(_)))
                .for_each(|data| {
                    let buffer = team
                        .iter()
                        .find_map(|unit_op| {
                            unit_op.and_then(|unit| {
                                if unit.kind == modifier.source.0 {
                                    Some(unit)
                                } else {
                                    None
                                }
                            })
                        })
                        .expect("buffer has to be in the team");

                    effective_stat += data.value(
                        self,
                        buffer,
                        modifier.config,
                        team,
                        light_cones_store,
                        relics_store,
                    )
                })
        });

        effective_stat + self.bonus_advanced_stat[stat]
    }
}
