use std::str::FromStr;

use serde::{Deserialize, Serialize};

pub trait Crdt {
    type Clock: PartialOrd;
    type Operation: Eq;
    fn update_clock(&mut self, other: Self::Clock);
    fn next_clock(&self) -> Self::Clock;
    fn apply(&mut self, op: &Self::Operation, op_clock: &Self::Clock);

    fn recv_op(&mut self, op: &Self::Operation, op_clock: Self::Clock) {
        self.apply(op, &op_clock);
        self.update_clock(op_clock);
    }
    fn local_op(&mut self, op: &Self::Operation) {
        let c = self.next_clock();
        self.apply(op, &c);
        self.update_clock(c);
    }
}

#[derive(Debug, Deserialize)]
pub enum Message<Op: Eq> {
    Get(Op),
    Add(Op),
    GetAll,
    Restart
}

#[derive(Serialize)]
pub enum Response<Op: Eq> {
    Committed,
    Abort,
    Read(Op, bool),
    Error,
}