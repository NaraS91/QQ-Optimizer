use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use super::hsr::{
    relics::{Relic, RelicPart},
    units::UnitKind,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct RelicsStore {
    relics: Vec<Option<Relic>>,
    reusable_ids: VecDeque<usize>,
    next_new_id: usize,
}

impl Default for RelicsStore {
    fn default() -> Self {
        RelicsStore::new()
    }
}

impl RelicsStore {
    pub fn new() -> Self {
        RelicsStore {
            relics: Vec::new(),
            reusable_ids: VecDeque::new(),
            next_new_id: 0,
        }
    }

    pub fn add(&mut self, mut relic: Relic) -> usize {
        if let Some(id) = self.reusable_ids.pop_back() {
            relic.id = id;
            self.relics[id] = Some(relic);
            id
        } else {
            relic.id = self.next_new_id;
            self.relics.push(Some(relic));
            self.next_new_id += 1;
            relic.id
        }
    }

    pub fn get_all_by_parts(&self) -> [Vec<Relic>; 6] {
        let (mut head, mut hands, mut body, mut feet, mut sphere, mut rope) = (
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        self.relics
            .iter()
            .filter_map(|op| *op)
            .for_each(|relic| match relic.part {
                RelicPart::Head => head.push(relic),
                RelicPart::Hands => hands.push(relic),
                RelicPart::Body => body.push(relic),
                RelicPart::Feet => feet.push(relic),
                RelicPart::Rope => rope.push(relic),
                RelicPart::Sphere => sphere.push(relic),
            });

        [head, hands, body, feet, sphere, rope]
    }

    pub fn get_all_as_ref(&self) -> Vec<&Relic> {
        self.relics.iter().filter_map(|op| op.as_ref()).collect()
    }

    pub fn update(&mut self, relic: Relic) -> bool {
        let id = relic.id;
        if self.next_new_id < id {
            self.relics[id] = Some(relic);
            true
        } else {
            false
        }
    }

    pub fn get_relic(&self, id: usize) -> Option<Relic> {
        if id < self.next_new_id {
            self.relics[id].clone()
        } else {
            None
        }
    }

    pub fn unequip(&self, id: usize) {
        if self.exists(id) {
            self.relics[id].unwrap().equipped = None;
        }
    }

    pub fn equip(&self, id: usize, unit_kind: UnitKind) {
        if self.exists(id) {
            self.relics[id].unwrap().equipped = Some(unit_kind);
        }
    }

    fn exists(&self, id: usize) -> bool {
        id < self.next_new_id && matches!(self.relics[id], Some(_))
    }
}
