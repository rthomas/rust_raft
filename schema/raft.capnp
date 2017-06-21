@0xa7ed6c5c8a98ca40;

struct LogEntry {
    term @0 :UInt32;
    key @1 :Text;
    value @2 :Text;
}

struct AppendEntriesResult {
    term @0 :UInt32;
    success @1 :Bool;
}

struct RequestVoteResult {
    term @0 :UInt32;
    voteGranted @1 :Bool;
}

interface Rpc {
    appendEntries @0 (term :UInt32,
                      leaderId :UInt8,
                      prevLogIndex :UInt64,
                      entries :List(LogEntry),
                      leaderCommit :UInt64)
                      -> (result: AppendEntriesResult);

    requestVote @1 (term :UInt32,
                    candidateId :UInt8,
                    lastLogIndex :UInt64,
                    lastLogTerm :UInt32)
                    -> (result: RequestVoteResult);
}

interface Bar {
    baz @0 (x :Int32) -> (y :Int32);
}

interface Qux {
    quux @0 (bar :Bar) -> (y :Int32);
}