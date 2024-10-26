use std::{fs::File, io::{Read, Seek, Write}};

use serde::{Deserialize, Serialize, self};

use crate::Crdt;

pub struct NonVolitileCrdt<Inner: Crdt + Serialize + for<'d> Deserialize<'d>> {
    inner: Inner,
    file: std::fs::File,
}

impl<T: Crdt + Serialize + for<'d> Deserialize<'d>> Crdt for NonVolitileCrdt<T> {
    type Clock = T::Clock;
    type Operation = T::Operation;

    fn next_clock(&self) -> Self::Clock {
        self.inner.next_clock()
    }
    fn apply(&mut self, op: &Self::Operation, op_clock: &Self::Clock) {
        self.inner.apply(op, op_clock);
    }
    fn update_clock(&mut self, other: Self::Clock) {
        self.inner.update_clock(other);
    }
    fn local_op(&mut self, op: &Self::Operation) {
        self.inner.local_op(op);
        self.file.seek(std::io::SeekFrom::Start(0)).unwrap();
        let data = ron::to_string(&self.inner).unwrap();
        self.file.write_all(data.as_bytes()).unwrap();
        self.file.set_len(data.as_bytes().len() as u64).unwrap();
        self.file.flush().unwrap();
    }
}

impl<T: Crdt + Serialize + for<'d> Deserialize<'d> + Default> NonVolitileCrdt<T> {
    pub fn new(mut file: File) -> Self {
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        let inner = ron::from_str(&buf).unwrap_or_default();

        Self {
            inner,
            file
        }
    }
    pub fn inner(&self) -> &T {
        &self.inner
    }
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}
