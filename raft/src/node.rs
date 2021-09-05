use crate::raft::Entry;
use flume::{Receiver, Sender};

pub type NodeId = u64;

pub struct ReplicatedLog {
    log: Vec<Entry>,
}

impl ReplicatedLog {
    pub fn new() -> ReplicatedLog {
        ReplicatedLog { log: Vec::new() }
    }

    pub fn append(&mut self, entry: Entry) {
        self.log.push(entry);
    }

    pub fn append_many(&mut self, entries: &mut Vec<Entry>) {
        self.log.append(entries);
    }
}

// Node states
pub struct Follower;
pub struct Candidate;
pub struct Leader;


pub struct Node<S> {
    pub id: NodeId,
    pub log: ReplicatedLog,
    pub term: u64,
    pub state: S,
    pub mailbox: Receiver<Entry>,
    pub friends: Vec<Sender<Entry>>,
}

// Don't think this is necessary, at least now. But it does allow me
// to implement any general functions applicable to a Node
// pub trait NodeState {}
// impl NodeState for Follower {}
// impl NodeState for Candidate {}
// impl NodeState for Leader {}
// impl<S: NodeState> Node<S> {}

impl Node<Follower> {
    pub fn new(id: NodeId, mailbox: Receiver<Entry>, friends: Vec<Sender<Entry>>) -> Node<Follower> {
        Node {
            id: id,
            log: ReplicatedLog::new(),
            term: 0,
            state: Follower {},
            mailbox: mailbox,
            friends: friends,
        }
    }
}

impl Node<Leader> {
    // Currently a hack so I don't have to implement leader election right now.
    // However, this is actually invalid. All Raft nodes start in Follower state
    pub fn new(id: NodeId, mailbox: Receiver<Entry>, friends: Vec<Sender<Entry>>) -> Node<Leader> {
        Node {
            id: id,
            log: ReplicatedLog::new(),
            term: 0,
            state: Leader {},
            mailbox: mailbox,
            friends: friends,
        }
    }
}