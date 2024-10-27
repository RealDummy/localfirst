use std::{cmp::max, collections::HashSet, hash::Hash, path::Iter};

use serde::{Deserialize, Serialize};

use crate::crdt::Crdt;

type Clock = VectorClock<u16>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VectorClock<T: Eq> {
    nodes: Vec<T>,
    clock: Vec<u64>,
}

impl<T: Eq + Clone> VectorClock<T> {
    pub fn new(me: T) -> Self {
        Self {
            nodes: vec![me],
            clock: vec![0],
        }
    }
    fn node_clocks(&self) -> impl Iterator<Item = (&T, &u64)> {
        self.nodes.iter().zip(&self.clock)
    }
    fn clock_of_mut(&mut self, node: &T) -> Option<&mut u64> {
        self.nodes.iter()
            .zip(self.clock.iter_mut())
            .find(|(n, _)| *n==node)
            .map(|(_, c)| c)
    }
    pub fn clock_of(&self, node: &T) -> Option<u64> {
        self.nodes.iter()
            .zip(self.clock.iter())
            .find(|(n, _)| *n==node)
            .map(|(_, c)| c)
            .cloned()
    }
    fn add_node(&mut self, from: &T, clock: u64) {
        self.nodes.push(from.clone());
        self.clock.push(clock);
    }
    pub fn recv(&mut self, from: &T) {
        if let Some(c) = self.clock_of_mut(from) {
            *c += 1;
        }
        self.add_node(from, 1);

    }
    pub fn update(&mut self, other: &Self) {
        for (n, c) in other.node_clocks() {
            match self.clock_of_mut(n) {
                Some(current) => {
                    *current = *c.max(current);
                }
                None => {
                    self.add_node(n, *c);
                }
            }
        }
    }
    pub fn behind_nodes<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = (&'a T, &'a u64)> {
        other.node_clocks().filter( |(on, &oc)| 
            oc > self.clock_of(on).unwrap_or(0)
        )
    }
}

impl<T: Eq + Clone> PartialEq for VectorClock<T> {
    fn eq(&self, other: &Self) -> bool {
        self.behind_nodes(other).count() == 0 && other.behind_nodes(self).count() == 0
    }
}

impl<T: Eq + Clone> Eq for VectorClock<T> {}

impl <T: Eq + Clone> PartialOrd for  VectorClock<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut gt = None;
        if self.behind_nodes(other).count() > 0 {
            gt = Some(other);
        } 
        if other.behind_nodes(self).count() > 0 {
            match gt {
                Some(_) => {return None;}
                None => {gt = Some(self);}
            };
        }
        match gt {
            Some(n) if n == self => Some(std::cmp::Ordering::Greater),
            Some(_) => Some(std::cmp::Ordering::Greater),
            None => panic!("shouldnt be here")
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrowSet<Atom: Hash + Eq> {
    name: u16,
    clock: Clock,
    set: HashSet<Atom>
}

impl<T: Hash + Eq> GrowSet<T> {
    pub fn new(me: u16) -> Self {
        Self {
            name: me,
            clock: Clock::new(me),
            set: HashSet::new(),
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
    fn update_clock(&mut self, other: &Self::Clock) {
        self.clock.update(other);
    }
    fn local_op(&mut self, op: &Self::Operation) {
        self.clock.recv(&self.name);
        self.set.insert(op.clone());
    }
    fn recv_op(&mut self, op: &Self::Operation, op_clock: &Self::Clock) {
        self.clock.update(op_clock);
        self.set.insert(op.clone());
    }
}
