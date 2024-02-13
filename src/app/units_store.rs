use std::collections::VecDeque;
use enum_map::{enum_map, EnumMap};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use super::hsr::{relics::{Relic, RelicPart}, units::{Unit, UnitKind}};

#[derive(Serialize, Deserialize)]
pub struct UnitsStore {
    units: Box<EnumMap<UnitKind, Option<Unit>>>,
}

impl Default for UnitsStore {
    fn default() -> Self {
        UnitsStore::new()
    }
}

impl UnitsStore {
    pub fn new() -> Self {
        let mut units = enum_map! {
            _ => None
        };

        for kind in UnitKind::iter() {
            units[kind] = Some(Unit::new(kind));
        }

        UnitsStore {
            units: Box::new(units)
        }
    }

    pub fn get_unit(&self, kind: UnitKind) -> &Option<Unit> {
        &self.units[kind]
    }

    pub fn get_all_kinds(&self) -> Vec<UnitKind> {
        self.units.iter()
            .filter(|(_, op)| {op.is_some()})
            .map(|(uk, _)| { uk }).collect()
    }

    pub fn equip_relic(&mut self, unit_kind: UnitKind, relic_id: usize, relic_part: RelicPart) -> bool {
        if let Some(ref mut unit) = self.units[unit_kind] {
            unit.update_relics(relic_id, relic_part);
            true
        } else {
            false
        }
    }

    pub fn unequip_relic(&mut self, unit_kind: UnitKind, relic_part: RelicPart) -> bool {
        if let Some(ref mut unit) = self.units[unit_kind] {
            unit.relics[relic_part.get_index()] = None;
            true
        } else {
            false
        }
    }

    pub fn update_team_mate(&mut self, main_unit: UnitKind, new_team_mate: Option<UnitKind>, id: usize) -> bool{
        if let Some(ref mut unit) = self.units[main_unit] {
            unit.team[id] = new_team_mate;
            true
        } else {
            false
        }
    }
}