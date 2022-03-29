fn main() {
    let number = 0;
    if number == 0 {
        println!("Number is zero");
    } else if number == 1 {
        println!("Number is one");
    } else {
        println!("I don't know the number")
    }

    let _flag = true;
    let num = if _flag {10} else {2};
    println!("The num is {}", num);


    let mut counter  = 0;
    while counter < 5 {
        println!("The value counter is {}", counter);
        counter += 1;
    }

    loop {
        println!("Hello loop");
        break;
    }
}
