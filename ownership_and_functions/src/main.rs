fn main() {
    let s1 = String::from("Hello");
    let length = calculate_length(&s1);

    println!("The lenght of {} is {}", s1, length);
}

// Cause an error because of Ownership - it is a shallow copy
// fn calculate_length(s: String) -> usize {
// We need to change it to a reference variable
fn calculate_length(s: &String) -> usize {
    // we cannot modify the value if it is not mutable, like
    // fn calculate_length(s: &mut String) -> usize {
    // and the variable is not mutable
    // s.push_str(" world");
    s.len()
}