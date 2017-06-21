@0xa7ed6c5c8a98ca40;

struct LogEntry {
    term @0 :UInt64;
    key @1 :Text;
    value @2 :Text;
}

interface Rpc {
    appendEntries @0 (term :UInt64,
                      leaderId :UInt8,
                      prevLogIndex :UInt64,
                      entries :List(LogEntry),
                      leaderCommit :UInt64)
                      -> (term :UInt64, success :Bool);

    requestVote @1 (term :UInt64,
                    candidateId :UInt8,
                    lastLogIndex :UInt64,
                    lastLogTerm :UInt32)
                    -> (term :UInt64, voteGraup :Bool);
}