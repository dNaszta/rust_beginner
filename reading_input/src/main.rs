use std::io;

fn main() {
    let mut text = String::new();

    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line");

    

    let number: u32 = text
        .trim()
        .parse()
        .expect("conversion failed, please enter a number");

    println!("The input numner is {}", number);
}
