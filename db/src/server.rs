use std::{fs::File, sync::{Arc, Mutex}};

use crate::{gset::GrowSet, store::NonVolitileCrdt};

type Messages = NonVolitileCrdt<GrowSet<String>>;

pub fn broadcast(peers: Vec<u16>, msg: Arc<Mutex<Messages>>) {
    
}

pub fn gossip_listen(port: u16, msg: Arc<Mutex<Messages>>) {

}

pub fn gossip(port: u16, peers: Vec<u16>) -> Arc<Mutex<Messages>> {
    let backing_store = File::options().create(true).read(true).write(true).open(&format!("{port}")).unwrap();
    let msg = Arc::new(Mutex::new(NonVolitileCrdt::new(backing_store)));

}