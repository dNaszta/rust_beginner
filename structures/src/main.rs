// use std::string;

struct Template {
    x:u32,
    y:i32,
    flag:bool
}

fn main() {
    let temp = Template{
        x: 23,
        y: -34,
        flag: true
    };

    println!("{}, {}, {}", temp.x, temp.y, temp.flag);

    let mut text = String::from("Hello");
    text.push(' ');
    let message = text + "world!";

    println!("{}", message);
}
