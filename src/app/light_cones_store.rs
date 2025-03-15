use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use super::hsr::{
    light_cones::{LightCone, LightConeKind},
    units::UnitKind,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct LightConesStore {
    light_cones: Vec<Option<LightCone>>,
    reusable_ids: VecDeque<usize>,
    next_new_id: usize,
}

impl Default for LightConesStore {
    fn default() -> Self {
        LightConesStore::new()
    }
}

impl LightConesStore {
    pub fn new() -> Self {
        LightConesStore {
            light_cones: vec![Some(LightCone::new(
                LightConeKind::Today_Is_Another_Peaceful_Day,
                80,
                6,
                5,
                0,
            ))],
            reusable_ids: VecDeque::new(),
            next_new_id: 1,
        }
    }

    pub fn new_empty() -> Self {
        LightConesStore {
            light_cones: Vec::new(),
            reusable_ids: VecDeque::new(),
            next_new_id: 0,
        }
    }

    pub fn add(&mut self, light_cone: &mut LightCone) {
        if let Some(id) = self.reusable_ids.pop_back() {
            light_cone.id = id;
            self.light_cones[id] = Some(*light_cone);
        } else {
            self.light_cones.push(Some(*light_cone));
            light_cone.id = self.next_new_id;
            self.next_new_id += 1;
        }
    }

    pub fn update(&mut self, light_cone: LightCone) -> bool {
        let id = light_cone.id;
        if self.next_new_id < id {
            self.light_cones[id] = Some(light_cone);
            true
        } else {
            false
        }
    }

    pub fn get(&self, id: usize) -> Option<LightCone> {
        if id < self.next_new_id {
            self.light_cones[id].clone()
        } else {
            None
        }
    }

    pub fn get_all_as_ref(&self) -> Vec<&LightCone> {
        self.light_cones
            .iter()
            .filter_map(|op| op.as_ref())
            .collect()
    }

    pub fn unequip(&self, id: usize) {
        if self.exists(id) {
            self.light_cones[id].unwrap().equipped = None;
        }
    }

    pub fn equip(&self, id: usize, unit_kind: UnitKind) {
        if self.exists(id) {
            self.light_cones[id].unwrap().equipped = Some(unit_kind);
        }
    }

    fn exists(&self, id: usize) -> bool {
        id < self.next_new_id && matches!(self.light_cones[id], Some(_))
    }
}
