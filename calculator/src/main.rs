fn main() {
    let mut choice = String::new();

    loop{
        calculator();
        println!("Do you want to continue? (Y to continue)");
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        if choice != "y" && choice != "Y" {
            break;
        }
    }
}

fn calculator() {
    println!("First number:");
    let first_number = read_an_integer();

    println!("Second number:");
    let second_number = read_an_integer();

    println!("Operation - choose from: +, -, *, /, ^");
    let operation = read_operation();

    do_operation(first_number, second_number, operation);
}

fn read_an_integer() -> u32 {
    let mut text = String::new();

    std::io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line");

    return text
        .trim()
        .parse::<u32>()
        .expect("conversion failed, please enter a number");
}

fn read_operation() -> char {
    let mut text = String::new();

    std::io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line");

    let mut char = text
        .trim()
        .parse::<char>()
        .expect("conversion failed, please enter a valid character");

    if char != '+' && char != '-' && char != '*' && char != '/' && char != '^' {
        char = '+';
    }

    char
}

fn do_operation(a: u32, b: u32, o: char) {
    print!("{} {} {} = ", a, o, b);
    if o == '+' {
        println!("{}", a + b);
    } else if o == '-' {
        println!("{}", a - b);
    } else if o == '*' {
        println!("{}", a * b);
    } else if o == '/' {
        println!("{}", a as f32 / b as f32);
    } else if o == '^' {
        println!("{}", u32::pow(a, b));
    }
}