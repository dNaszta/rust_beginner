fn main() {
    let ten = times_two(5);
    println!("Two times 5 is {}", ten);
}

fn times_two(x: i32) -> i32 {
    2 * x
}