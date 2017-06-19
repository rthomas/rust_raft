@0xa7ed6c5c8a98ca40;

interface Bar {
    baz @0 (x :Int32) -> (y :Int32);
}

interface Qux {
    quux @0 (bar :Bar) -> (y :Int32);
}