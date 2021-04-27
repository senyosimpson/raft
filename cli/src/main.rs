use raft::node::Node;
use raft::node::NodeState;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Raft", about = "CLI to control Raft algorithm")]
enum Command {
    /// Starts a server. Can choose between a leader and follower
    Start {
        id: u64,
        port: u16,
        #[structopt(long)]
        leader: bool,
    },
}

#[async_std::main]
async fn main() {
    let cmd = Command::from_args();
    match cmd {
        Command::Start {id, port, leader } => {
            let state = match leader {
                true => NodeState::Leader,
                false => NodeState::Follower,
            };

            let node = Node::new(id, state);
            node.start(port).await.unwrap();
        }
    }
}
