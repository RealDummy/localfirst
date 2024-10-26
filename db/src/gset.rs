use std::{cmp::max, collections::HashSet, hash::Hash};

use serde::{Deserialize, Serialize};

use crate::crdt::Crdt;

type Clock = i32;

#[derive(Debug, Serialize, Default, Deserialize)]
pub struct GrowSet<Atom: Hash + Eq> {
    clock: Clock,
    set: HashSet<Atom>
}

impl<T: Hash + Eq> GrowSet<T> {
    pub fn new() -> Self {
        Self {
            clock: 0,
            set: HashSet::new()
        }
    }
    pub fn get(&self, item: T) -> bool {
        self.set.contains(&item)
    }
    pub fn get_all(&self) -> &HashSet<T> {
        return &self.set;
    }
}

impl<T: Hash + Eq + Clone> crate::crdt::Crdt for GrowSet<T> {
    type Clock = Clock;
    type Operation = T;
    fn update_clock(&mut self, other: Self::Clock) {
        self.clock = self.clock.max(other);
    }
    fn next_clock(&self) -> Self::Clock {
        self.clock + 1
    }
    fn apply(&mut self, op: &Self::Operation, op_clock: &Self::Clock) {
        self.set.insert(op.clone());
    }
}
