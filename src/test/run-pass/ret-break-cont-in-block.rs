// xfail-fast
#[legacy_modes];

use cmp::Eq;

fn iter<T>(v: ~[T], it: fn(T) -> bool) {
    let mut i = 0u, l = v.len();
    while i < l {
        if !it(v[i]) { break; }
        i += 1u;
    }
}

fn find_pos<T:Eq>(n: T, h: ~[T]) -> Option<uint> {
    let mut i = 0u;
    for iter(h) |e| {
        if e == n { return Some(i); }
        i += 1u;
    }
    None
}

fn bail_deep(x: ~[~[bool]]) {
    let mut seen = false;
    for iter(x) |x| {
        for iter(x) |x| {
            assert !seen;
            if x { seen = true; return; }
        }
    }
    assert !seen;
}

fn ret_deep() -> ~str {
    for iter(~[1, 2]) |e| {
        for iter(~[3, 4]) |x| {
            if e + x > 4 { return ~"hi"; }
        }
    }
    return ~"bye";
}

fn main() {
    let mut last = 0;
    for vec::all(~[1, 2, 3, 4, 5, 6, 7]) |e| {
        last = e;
        if e == 5 { break; }
        if e % 2 == 1 { loop; }
        assert e % 2 == 0;
    };
    assert last == 5;

    assert find_pos(1, ~[0, 1, 2, 3]) == Some(1u);
    assert find_pos(1, ~[0, 4, 2, 3]) == None;
    assert find_pos(~"hi", ~[~"foo", ~"bar", ~"baz", ~"hi"]) == Some(3u);

    bail_deep(~[~[false, false], ~[true, true], ~[false, true]]);
    bail_deep(~[~[true]]);
    bail_deep(~[~[false, false, false]]);

    assert ret_deep() == ~"hi";
}
