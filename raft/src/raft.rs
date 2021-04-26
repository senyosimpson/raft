use uuid::Uuid;

use crate::node::NodeId;

enum EntryState {
    Pending,
    Replicated,
    Committed,
}

// Entry<State> ??
struct Entry {
    // Cmd is just a String for now. Will do something better in
    // follow up iterations
    cmd: String,
    term: u64,
    index: u64,
    state: EntryState,
}

impl Entry {
    // Need to code in domain invariant. Can only go from replicated -> committed
    pub fn commit(&mut self) {
        self.state = EntryState::Committed
    }
}

struct AppendEntriesRequest {
    /// The log entries
    pub entries: Vec<Entry>,
    /// The index of the next log entry to send to the given follower
    pub next_index: u64,
    /// The highest known index in the log that has been committed
    pub leader_commit_index: u64,
    /// The index of the previous entry in the log
    pub previous_entry_index: u64,
    /// The term of the previous entry in the log
    pub previous_entry_term: u64,
    /// The ID of the leader node. Useful when redirecting clients
    pub leader_id: NodeId,
    /// Serial number submitted by the client
    pub serial_number: Uuid,
}

enum AppendEntriesStatus {
    Successful,
    Rejected,
}

struct AppendEntriesResponse {
    status: AppendEntriesStatus,
}

struct RequestVoteRequest {
    /// The term of the last entry in the candidate's log
    pub candidate_term: u64,
    /// The index of the last entry in the candidate's log
    pub candidate_index: u64,
}

struct RequestVoteResponse {}
