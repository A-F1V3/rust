// Tests that one can't run a destructor twice with the repeated vector
// literal syntax.

struct Foo {
    x: int,

    drop {
        io::println("Goodbye!");
    }
}

fn main() {
    let a = Foo { x: 3 };
    let _ = [ a, ..5 ];     //~ ERROR copying a noncopyable value
}

