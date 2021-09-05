use flume;
use raft::node::{Candidate, Follower, Leader, Node};
use raft::raft::{Entry, EntryState};
use std::thread;
use std::time::Duration;

fn main() {
    let mut txs = Vec::new();
    let mut rxs = Vec::new();

    for _ in 0..5 {
        let (tx, rx) = flume::unbounded();
        txs.push(tx);
        rxs.push(rx);
    }


    let mut handles = Vec::new();
    for (i, rx) in rxs.into_iter().enumerate() {
        if i == 0 {
            let node = Node::<Leader>::new(i as u64, rx, txs.clone());
            let handle = thread::spawn(move || loop {
                let entry = node.mailbox.recv().unwrap();
                println!("Leader: Received entry: {:#?}", entry);
            });
            handles.push(handle);
        } else {
            let node =  Node::<Follower>::new(i as u64, rx,txs.clone());
            let handle = thread::spawn(move || loop {
                let entry = node.mailbox.recv().unwrap();
                println!("Peer {}: Received entry: {:#?}", i, entry);
            });
            handles.push(handle);
        };

    }
    let tx_leader = txs[0].clone();
    let handle = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(2));
        let entry = Entry {
            cmd: String::from("Woza!"),
            term: 1,
            index: 1,
            state: EntryState::Committed,
        };
        tx_leader.send(entry).unwrap();
    });

    handles.push(handle);

    for handle in handles {
        handle.join();
    }
}
