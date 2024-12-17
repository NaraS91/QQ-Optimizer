use std::collections::HashMap;

use super::{
    hsr::{
        basics::Element,
        light_cones::{LightCone, LightConeKind},
        relics::{CavernSet, PlanarSet, Relic, RelicPart, RelicSet, RelicStat},
        units::{UniqueData, UnitKind},
    },
    light_cones_store::LightConesStore,
    relics_store::RelicsStore,
    units_store::UnitsStore,
};
use json::{self, JsonValue};
use log::debug;

pub fn import(
    file_content: &str,
    relics_store: &mut RelicsStore,
    light_cones_store: &mut LightConesStore,
    units_store: &mut UnitsStore,
) {
    let data = json::parse(&file_content).ok().unwrap();
    import_units(&data["characters"], units_store);
    import_light_cones(&data["light_cones"], light_cones_store, units_store);
    import_relics(&data["relics"], relics_store, units_store);
}

fn import_units(units: &JsonValue, units_store: &mut UnitsStore) {
    for unit_data in units.members() {
        let kind = UnitKind::from_str(unit_data["key"].as_str().unwrap());
        if let Some(kind) = kind {
            units_store.add_unit(
                kind,
                UniqueData::new(
                    unit_data["level"].as_u32().unwrap(),
                    unit_data["ascension"].as_u32().unwrap(),
                    unit_data["skills"]["ult"].as_u8().unwrap(),
                    unit_data["skills"]["skill"].as_u8().unwrap(),
                    unit_data["skills"]["basic"].as_u8().unwrap(),
                    unit_data["skills"]["talent"].as_u8().unwrap(),
                    unit_data["eidolon"].as_u8().unwrap(),
                ),
            )
        } else {
            println!("???? {}", unit_data["key"].as_str().unwrap());
        }
    }
}

fn import_light_cones(
    light_cones: &JsonValue,
    light_cones_store: &mut LightConesStore,
    units_store: &mut UnitsStore,
) {
    for light_cone in light_cones.members() {
        let kind = LightConeKind::from_str(light_cone["key"].as_str().unwrap());

        if kind.is_none() {
            continue;
        }

        let mut lc = LightCone::new(
            kind.expect("checked above"),
            light_cone["level"].as_u16().unwrap(),
            light_cone["ascension"].as_u16().unwrap(),
            light_cone["superimposition"].as_u8().unwrap(),
            0,
        );

        light_cones_store.add(&mut lc);
        if let Some(unit_kind) = UnitKind::from_str(light_cone["location"].as_str().unwrap()) {
            if units_store.get_unique_data(unit_kind).is_some() {
                units_store.equip_light_cone(unit_kind, lc.id);
                lc.equipped = Some(unit_kind);
            }
        }
    }
}

fn import_relics(relics: &JsonValue, relics_store: &mut RelicsStore, units_store: &mut UnitsStore) {
    let sets = HashMap::from(STRING_TO_RELIC_NAME);
    for relic_data in relics.members() {
        let set = sets[relic_data["set"].as_str().expect("relic has a set")];
        let part = match relic_data["slot"].as_str().unwrap() {
            "Hands" => RelicPart::Hands,
            "Body" => RelicPart::Body,
            "Head" => RelicPart::Head,
            "Feet" => RelicPart::Feet,
            "Planar Sphere" => RelicPart::Sphere,
            "Link Rope" => RelicPart::Rope,
            _ => continue,
        };
        let main_stat = match (relic_data["mainstat"].as_str().unwrap(), part) {
            (_, RelicPart::Hands) => RelicStat::Atk,
            (_, RelicPart::Head) => RelicStat::Hp,
            ("ATK", _) => RelicStat::AtkP,
            ("DEF", _) => RelicStat::DefP,
            ("CRIT DMG", _) => RelicStat::Cd,
            ("CRIT Rate", _) => RelicStat::Cr,
            ("Break Effect", _) => RelicStat::BE,
            ("HP", _) => RelicStat::HpP,
            ("SPD", _) => RelicStat::Spd,
            ("Energy Regeneration Rate", _) => RelicStat::Err,
            ("Outgoing Healing Boost", _) => RelicStat::OH,
            ("Ice DMG Boost", _) => RelicStat::ElementalDmg(Element::Ice),
            ("Quantum DMG Boost", _) => RelicStat::ElementalDmg(Element::Quantum),
            ("Physical DMG Boost", _) => RelicStat::ElementalDmg(Element::Physical),
            ("Fire DMG Boost", _) => RelicStat::ElementalDmg(Element::Fire),
            ("Wind DMG Boost", _) => RelicStat::ElementalDmg(Element::Wind),
            ("Lightning DMG Boost", _) => RelicStat::ElementalDmg(Element::Lightning),
            ("Imaginary DMG Boost", _) => RelicStat::ElementalDmg(Element::Imaginary),
            ("Effect Hit Rate", _) => RelicStat::EHR,
            ("Effect RES", _) => RelicStat::ERes,
            (stat, _) => {
                debug!("wrong main stat: {}", stat);
                continue;
            }
        };

        let mut sub_stats = [None; 4];

        for (i, sub) in relic_data["substats"].members().enumerate() {
            let stat = match sub["key"].as_str().unwrap() {
                "ATK" => RelicStat::Atk,
                "DEF" => RelicStat::Def,
                "HP" => RelicStat::Hp,
                "SPD" => RelicStat::Spd,
                "CRIT Rate_" => RelicStat::Cr,
                "CRIT DMG_" => RelicStat::Cd,
                "DEF_" => RelicStat::DefP,
                "ATK_" => RelicStat::AtkP,
                "HP_" => RelicStat::HpP,
                "Break Effect_" => RelicStat::BE,
                "Effect RES_" => RelicStat::ERes,
                "Effect Hit Rate_" => RelicStat::EHR,
                some_stat => {
                    debug!("wrong sub stat: {}", some_stat);
                    continue;
                }
            };
            sub_stats[i] = if matches!(
                stat,
                RelicStat::Atk | RelicStat::Def | RelicStat::Hp | RelicStat::Spd
            ) {
                Some((stat, sub["value"].as_f32().unwrap()))
            } else {
                Some((stat, sub["value"].as_f32().unwrap() / 100.))
            }
        }

        //todo: add location/equipped unit

        let mut equipped = UnitKind::from_str(relic_data["location"].as_str().unwrap());

        if let Some(unit_kind) = equipped {
            if units_store.get_unique_data(unit_kind).is_none() {
                equipped = None;
            }
        }

        let id = relics_store.add(Relic::new(
            relic_data["level"].as_u8().unwrap(),
            part,
            set,
            main_stat,
            sub_stats,
            equipped,
        ));

        if let Some(unit_kind) = equipped {
            units_store.equip_relic(unit_kind, id, part);
        }
    }
}

const STRING_TO_RELIC_NAME: [(&str, RelicSet); 19] = [
    (
        "Messenger Traversing Hackerspace",
        RelicSet::Cavern(CavernSet::Messenger_Traversing_Hackerspace),
    ),
    (
        "Hunter of Glacial Forest",
        RelicSet::Cavern(CavernSet::Hunter_Of_Glacial_Forest),
    ),
    (
        "Inert Salsotto",
        RelicSet::Planar(PlanarSet::Inert_Salsotto),
    ),
    (
        "Genius of Brilliant Stars",
        RelicSet::Cavern(CavernSet::Genius_Of_Brilliant_Stars),
    ),
    (
        "Fleet of the Ageless",
        RelicSet::Planar(PlanarSet::Fleet_Of_The_Ageless),
    ),
    (
        "Longevous Disciple",
        RelicSet::Cavern(CavernSet::Longevous_Disciple),
    ),
    (
        "Guard of Wuthering Snow",
        RelicSet::Cavern(CavernSet::Guard_Of_Wuthering_Snow),
    ),
    (
        "Rutilant Arena",
        RelicSet::Planar(PlanarSet::Rutilant_Arena),
    ),
    (
        "Space Sealing Station",
        RelicSet::Planar(PlanarSet::Space_Sealing_Station),
    ),
    (
        "Musketeer of Wild Wheat",
        RelicSet::Cavern(CavernSet::Musketeer_Of_Wild_Wheat),
    ),
    ("Broken Keel", RelicSet::Planar(PlanarSet::Broken_Keel)),
    (
        "Belobog of the Architects",
        RelicSet::Planar(PlanarSet::Belobog_Of_The_Architects),
    ),
    (
        "Passerby of Wandering Cloud",
        RelicSet::Cavern(CavernSet::Passerby_Of_Wandering_Cloud),
    ),
    (
        "Thief of Shooting Meteor",
        RelicSet::Cavern(CavernSet::Thief_Of_Shooting_Meteor),
    ),
    (
        "Sprightly Vonwacq",
        RelicSet::Planar(PlanarSet::Sprightly_Vonwacq),
    ),
    (
        "Firesmith of Lava-Forging",
        RelicSet::Cavern(CavernSet::Firesmith_Of_Lavaforging),
    ),
    (
        "Firmament Frontline: Glamoth",
        RelicSet::Planar(PlanarSet::Firmament_Frontline_Glamoth),
    ),
    (
        "Penacony, Land of the Dreams",
        RelicSet::Planar(PlanarSet::Penacony_Land_Of_The_Dreams),
    ),
    (
        "Pan-Cosmic Commercial Enterprise",
        RelicSet::Planar(PlanarSet::Pancosmic_Commercial_Enterprise),
    ),
];
