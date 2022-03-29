fn main() {
    let num1:u8 = 5;
    let num2:u8 = 10;

    let sum = num1 + num2;
    println!("{} plus {} is {}", num1, num2, sum);

    let mut mutable = 5;
    mutable = mutable + 1;
    println!("{}", mutable);

    let _flag = 3 == 3;

    println!("{}", _flag);
}
