use enum_map::{enum_map, EnumMap};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use super::{
    hsr::{
        relics::RelicPart,
        units::{ParsedUnitFile, UniqueData, Unit, UnitKind},
    },
    ASSETS_LOADER,
};

#[derive(Serialize, Deserialize)]
pub struct UnitsStore {
    unique_data: Box<EnumMap<UnitKind, Option<UniqueData>>>,
    parsed_data: Box<EnumMap<UnitKind, Option<ParsedUnitFile>>>,
}

impl Default for UnitsStore {
    fn default() -> Self {
        UnitsStore::new()
    }
}

impl UnitsStore {
    pub fn new() -> Self {
        let mut unique_data = enum_map! {
            _ => None
        };

        for kind in UnitKind::iter() {
            unique_data[kind] = Some(Default::default());
        }

        UnitsStore {
            unique_data: Box::new(unique_data),
            parsed_data: Box::new(enum_map! { _ => None}),
        }
    }

    pub fn new_empty() -> Self {
        UnitsStore {
            unique_data: Box::new(enum_map! { _ => None}),
            parsed_data: Box::new(enum_map! { _ => None}),
        }
    }

    pub fn add_unit(&mut self, kind: UnitKind, data: UniqueData) {
        self.unique_data[kind] = Some(data);
    }

    pub fn get_unit(&mut self, ctx: &egui::Context, kind: UnitKind) -> Option<Unit> {
        if self.parsed_data[kind].is_none() {
            let unit_data = ASSETS_LOADER.get_unit_base_data(ctx, kind);
            if let Some(data) = unit_data {
                self.parsed_data[kind] = Some(Unit::parse_character_info(data));
            } else {
                println!("hmmm");
                return None;
            }
        }

        let parsed_data = self.parsed_data[kind].as_ref().unwrap();
        self.unique_data[kind]
            .and_then(|unique_data| Some(Unit::from_data(kind, unique_data, parsed_data)))
    }

    pub fn get_all_possessed_unit_kinds(&self, ctx: &egui::Context) -> Vec<UnitKind> {
        self.unique_data
            .iter()
            .filter(|(_, op)| op.is_some())
            .map(|(uk, _)| uk)
            .collect()
    }

    pub fn get_unique_data(&self, kind: UnitKind) -> Option<UniqueData> {
        self.unique_data[kind]
    }

    pub fn get_all_kinds(&self) -> Vec<UnitKind> {
        self.unique_data
            .iter()
            .filter(|(_, op)| op.is_some())
            .map(|(uk, _)| uk)
            .collect()
    }

    pub fn equip_light_cone(&mut self, unit_kind: UnitKind, light_cone_id: usize) -> bool {
        if let Some(ref mut unit) = self.unique_data[unit_kind] {
            unit.light_cone = Some(light_cone_id);
            true
        } else {
            false
        }
    }

    pub fn unequip_light_cone(&mut self, unit_kind: UnitKind) -> Option<usize> {
        if let Some(ref mut unit) = self.unique_data[unit_kind] {
            let id = unit.light_cone;
            unit.light_cone = None;
            id
        } else {
            None
        }
    }

    pub fn equip_relic(
        &mut self,
        unit_kind: UnitKind,
        relic_id: usize,
        relic_part: RelicPart,
    ) -> bool {
        if let Some(ref mut unit) = self.unique_data[unit_kind] {
            unit.update_relics(relic_id, relic_part);
            true
        } else {
            false
        }
    }

    pub fn unequip_relic(&mut self, unit_kind: UnitKind, relic_part: RelicPart) -> bool {
        if let Some(ref mut unit) = self.unique_data[unit_kind] {
            unit.relics[relic_part.get_index()] = None;
            true
        } else {
            false
        }
    }

    pub fn update_team_mate(
        &mut self,
        main_unit: UnitKind,
        new_team_mate: Option<UnitKind>,
        id: usize,
    ) -> bool {
        if let Some(ref mut unit) = self.unique_data[main_unit] {
            unit.team[id] = new_team_mate;
            true
        } else {
            false
        }
    }
}
