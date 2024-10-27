use std::str::FromStr;

use serde::{Deserialize, Serialize};

pub trait Crdt {
    type Clock: PartialOrd;
    type Operation: Eq;
    fn update_clock(&mut self, other: &Self::Clock);
    fn recv_op(&mut self, op: &Self::Operation, op_clock: &Self::Clock);
    fn local_op(&mut self, op: &Self::Operation);
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