struct Log {

}

pub type NodeId = u64;

enum NodeState {
    Leader,
    Follower,
    Candidate
}

// Node<State> ?? Read Ana's blog post again
// https://hoverbear.org/blog/rust-state-machine-pattern/
struct Node {
    id: NodeId,
    term: u64,
    state: NodeState
}

impl Node {
    pub fn new(&self, state: NodeState) {
        match state {
            NodeState::Leader => self.new_leader(),
            NodeState::Follower | NodeState::Candidate => self.new_follower()
        }
    }

    fn new_leader(&self) {

    }
    
    fn new_follower(&self) {

    }

    /// Starts the state machine
    fn start(&self) {

    }
}