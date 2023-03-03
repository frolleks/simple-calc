use std::io;

fn main() {
    let mut x = String::new();
    let mut y = String::new();
    let mut operator = String::new();

    println!("Please enter the first number");
    io::stdin().read_line(&mut x).expect("Failed to read line");
    println!("Please enter the second number");
    io::stdin().read_line(&mut y).expect("Failed to read line");
    println!("Please enter an operator (e.g. multiply, add, subtract, divide)");
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line");

    let x: usize = x.trim().parse().expect("Please type a number!");
    let y: usize = y.trim().parse().expect("Please type a number!");
    let _operator: String = operator.trim().parse().expect("Please enter an operator!");

    if &_operator == "add" {
        println!("Answer: {}", x + y);
    } else if &_operator == "subtract" {
        println!("Answer: {}", x - y);
    } else if &_operator == "multiply" {
        println!("Answer: {}", x * y);
    } else if &_operator == "divide" {
        println!("Answer: {}", x / y);
    }
}
