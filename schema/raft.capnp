@0x969abc084e5aa2b6;

struct NodeInfo {
    leader @0 :Text;
    nodes @1 :List(Text);
}

struct LogEntry {
    term @0 :UInt64;
    key @1 :Text;
    value @2 :Text;
}

interface Raft {
    appendEntries @0 (term :UInt64,
                      leaderId :UInt8,
                      prevLogIndex :UInt64,
                      prevLogTerm :UInt64,
                      entries :List(LogEntry),
                      leaderCommit :UInt64)
                      -> (term :UInt64, success :Bool, nodeInfo :NodeInfo);

    requestVote @1 (term :UInt64,
                    candidateId :UInt8,
                    lastLogIndex :UInt64,
                    lastLogTerm :UInt32)
                    -> (term :UInt64, voteGraup :Bool);
}
