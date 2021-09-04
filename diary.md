# Development Diary

## 4 September '21

[raft-rs]: https://github.com/tikv/raft-rs
[etcd's raft]: https://github.com/etcd-io/etcd/tree/main/raft

The current mental model of how to put together my Raft implementation does not make sense. I was
thinking that I'd have a Raft library which provides Raft functionality and embed that directly into
a HTTP server i.e each server would be a Raft node. You would then have end points `appendEntries`
and `RequestVote` end points. So stupid argghhhhhh.

So what do we actually need?

![dist-db](./assets/distributed-database-arch.png)

In the diagram above is an example architecture of a web service backed by a distributed database. This
gives key insights into how I ought to think about this. First, Raft works on the *storage* system
(obviously, c'mon Senyo!). Our Raft nodes are nodes of our storage system, not the HTTP servers themselves.
Each of the HTTP server replicas write to the same storage system and so we have one distributed database
for the web service. In steps, the process would be:

1. Client makes a (write) request to web service
2. A replica accepts the request and writes to the database
3. The write request is accepted by the leader of the storage system cluster
4. The write is replicated to the followers in the cluster
5. Done!

Now I have a good mental model to work with! We will still need the Raft library but instead, it will
be used by the storage component.

***

Being true to keeping it simple, I'm going to attempt the most basic implementation I can. To facilitate
this: the replicated log will just be a vector of `Entry`s kept in memory. There's no need for persistence
as of yet.

```rust
pub struct Entry {
    pub cmd: String, // I wish I remembered what this was for lmao
    pub term: u64,
    pub index: u64,
    pub state: EntryState,
}

pub struct ReplicatedLog {
    log: Vec<Entry>
}
```

No data will actually be stored anywhere for now (though this may help with debugging 🤔). For now,
we would like for our replicated log to remain consistent across nodes i.e at each index, the entries
are identical across all nodes.

The Raft cluster will have only one method for interfacing with it, `write`. It'll just take in some
data and return success/fail.

```rust
fn write(data: String) -> bool {}
```
