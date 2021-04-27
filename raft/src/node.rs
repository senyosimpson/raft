use std::{fs::File, io::prelude::*, net::SocketAddr};

use serde::{Deserialize, Serialize};
use tide::{prelude::Status, Request};

use crate::raft::{AppendEntriesRequest, AppendEntriesResponse, AppendEntriesStatus, Entry};

pub type NodeId = u64;

struct Log {
    storage: File,
}

impl Log {
    pub fn new(id: NodeId) -> Log {
        let filename = format!("server-{}", id);
        let file = File::create(filename).unwrap();

        Log { storage: file }
    }

    pub fn append(&mut self, entry: Entry) {
        self.storage.write_all(entry.cmd.as_bytes()).unwrap()
    }
}

pub enum NodeState {
    Leader,
    Follower,
    Candidate,
}

// Node<State> ?? Read Ana's blog post again
// https://hoverbear.org/blog/rust-state-machine-pattern/
pub struct Node {
    id: NodeId,
    log: Log,
    term: u64,
    state: NodeState,
}

impl Node {
    pub fn new(id: NodeId, state: NodeState) -> Node {
        let state = match state {
            NodeState::Leader => NodeState::Leader,
            NodeState::Follower | NodeState::Candidate => NodeState::Follower,
        };

        Node {
            id: id,
            log: Log::new(id),
            term: 0,
            state: state,
        }
    }

    /// Starts the state machine
    pub async fn start(&self, port: u16) -> tide::Result<()> {
        tide::log::start();
        let mut server = tide::new();
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        server.at("/test/:name").post(hello);
        server.at("/append-entry").post(append_entry);
        server.listen(addr).await?;
        Ok(())
    }
}

async fn hello(req: Request<()>) -> tide::Result<String> {
    let name = req.param("name")?;
    Ok(format!("Hello, {}", name))
}

/// Start
async fn append_entry(mut req: Request<()>) -> tide::Result<String> {
    let append_entries: AppendEntriesRequest = req.body_json().await.unwrap();

    // Ok(AppendEntriesResponse {
    //     status: AppendEntriesStatus::Successful
    // })
    Ok("Success".into())
}
