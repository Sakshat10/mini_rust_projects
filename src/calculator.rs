use std::io;

pub fn calculator(){
    println!("enter first number:");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("failed to read line");
    let num1: f64 = num1.trim().parse().expect("please type a number!");

    println!("enter second number:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("failed to read a line");
    let num2: f64 = num2.trim().parse().expect("enter a valid number!");

    println!("chose an operation (+,-,*,/)");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("failed to read a line");
    let operation = operation.trim();

    let result = match operation {
        "+" => num1+num2,
        "-"=>num1-num2,
        "*"=>num1*num2,
        "/"=>num1/num2,
        _=>{
            println!("Invalid operation");
            return;
        }
    };
    println!("Result: {}",result);
}