use std::{collections::HashSet, fs::{self, DirEntry}, os::unix::net::SocketAddr, str::FromStr};

use rand::random;
use serde::{Deserialize, Serialize};

use crate::{crdt::{Crdt, Message}, gset::{self, GrowSet}, store::NonVolitileCrdt};


pub struct Tester {
    nodes: Vec<String>,
    input: Vec<Vec<String>>,
}

impl Tester {
    pub fn new(dir: &str) -> Self {
        let dir = std::fs::read_dir(dir).expect("dir must exist");
        let mut input = Vec::new();
        let mut nodes = Vec::new();

        for file in dir {
            let node_name = file.unwrap().path();
            nodes.push(node_name.file_name().unwrap().to_str().unwrap().to_owned());
            let mut cmd = Vec::new();
            let text = std::fs::read_to_string(node_name).unwrap();
            for line in text.split("\n") {
                cmd.push(line.to_owned());
            }
            input.push(cmd);
        }
        Self {
            input,
            nodes,
        }
    }
    pub fn test_random(&mut self) -> HashSet<i32> {
        let f = std::fs::File::options().create(true).read(true).write(true).open("test_random").unwrap();

        let mut gset = NonVolitileCrdt::<GrowSet<i32>>::new(f);
        let mut nexts: Vec<_> = self.input.iter().map(|v| v.iter()).collect();
        loop {
            let r: usize = random::<usize>() % nexts.len();
            let next_inp = nexts[r].next();
            match next_inp {
                Some(str) => {
                    let Ok(msg) = ron::from_str(&str) else {
                        continue;
                    };
                    let n = match msg {
                        Message::Add(n) => n,
                        Message::Restart => {
                            let f = std::fs::File::options().create(true).read(true).write(true).open("test_random").unwrap();
                            gset = NonVolitileCrdt::<GrowSet<i32>>::new(f);
                            continue;

                        }
                        _ => {
                            panic!("dont test read yet");
                        }
                    };
                    gset.local_op(&n);
                }
                None => {
                    let _ =  nexts.remove(r);
                    if nexts.len() == 0 {
                        fs::remove_file("test_random").unwrap();

                        return gset.inner().get_all().clone();
                    }
                }
            }
        }
    }
}

