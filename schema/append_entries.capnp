@0xbe346dd48a80bb43;

struct LogEntry {
    key @0 :Text;
    value @1 :Text;
}

struct AppendEntries {
    term @0 :UInt32;
    leaderId @1 :UInt8;
    prevLogIndex @2 :UInt64;
    entries @3 :List(LogEntry);
    leaderCommit @4 :UInt64;
}

struct AppendEntriesResult {
    term @0 :UInt32;
    success @1 :Bool;
}