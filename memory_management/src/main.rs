fn main() {
    // primitive is part of the stack
    let _a = 5;

    // heap - just the pointer part is part of the stack, the the content is not
    // stack parts
    // - pointer
    // - len
    // - capacity
    // heap parts
    // - index
    // - value
    let _s1 = String::from("Hello");

    // Shallow copy
    // let _s2 = _s1; than _s1 won't work anymore - _s2 will replace _s1

    // Rules of ownership
    // 1. Each value in Rust has a variable that's called owner
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of scope, the value will be dropped

    // Deep copy
    let _s2 = _s1.clone();
    println!("{}", _s1);
    println!("{}", _s2);

    // At primitives - deep copy is the default (i32, etc.)

    f1();
}

fn f1() {
    let _b = 10;
    f2();
}

fn f2() {
    let _c = 15;
}
