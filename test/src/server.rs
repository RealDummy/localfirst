use std::{fs::File, net::{IpAddr, Ipv4Addr, SocketAddr}, sync::{Arc, Mutex}};


use hyper::server::conn::http1::Builder;

use crate::{crdt::Crdt, gset::GrowSet, store::NonVolitileCrdt};

type Messages = NonVolitileCrdt<GrowSet<String>>;



pub fn broadcast(peers: Vec<u16>, msg: Arc<Mutex<Messages>>) {
    
}

async fn gossip_conn(msg: Arc<Mutex<Messages>>, socket: tokio::net::TcpStream, addr: SocketAddr) {
    let msg = msg.lock().unwrap();
    let clock = msg.get_clock();
}

pub async fn gossip_listen(port: u16, msg: Arc<Mutex<Messages>>) {
    let ipaddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let sockaddr = SocketAddr::new(ipaddr, port);
    let listener = tokio::net::TcpListener::bind(sockaddr).await.unwrap();
    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        let msg2: Arc<Mutex<NonVolitileCrdt<GrowSet<String>>>> = msg.clone();
        tokio::spawn(async move {
            gossip_conn(msg2, socket, addr).await
        });
    }
}

pub fn gossip(port: u16, peers: Vec<u16>) -> Arc<Mutex<Messages>> {
    let backing_store = File::options().create(true).read(true).write(true).open(&format!("{port}")).unwrap();
    let crdt = NonVolitileCrdt::from_file(backing_store).unwrap_or_else(|| {
        let backing_store = File::options().create(true).read(true).write(true).open(&format!("{port}")).unwrap();
        NonVolitileCrdt::new(backing_store, GrowSet::new(port))
    });
    let msg = Arc::new(Mutex::new(crdt));
    let svc = hyper::service::service_fn(f)
    let builder = Builder::new()
        .serve_connection(io, service)
    msg.clone()

}