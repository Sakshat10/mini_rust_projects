use std::io;

pub fn temp_converter(){
    println!("choose conversion: ");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");

    let mut choose = String::new();
    io::stdin().read_line(&mut choose).expect("failed to read a line");
    let choose:u8 = choose.trim().parse().expect("please type  number!");

    if choose == 1{
        println!("enter temperature in Fahrenheit");
        let mut fahrenheit = String::new();
        io::stdin().read_line(&mut fahrenheit).expect("failed to read a line");
        let fahrenheit:f64 = fahrenheit.trim().parse().expect("please type a number!");

        let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
        println!("Temperature in Celsius: {}", celsius);
    }
    else if choose == 2{
        println!("enter temperature in celsius");
        let mut celsius = String::new();
        io::stdin().read_line(&mut celsius).expect("failed to read a line");
        let celsius:f64 = celsius.trim().parse().expect("please type a number!");

        let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
        println!("Temperature in Fahrenheit: {}", fahrenheit);
    }
     else {
        println!("Invalid choice");
    }
    
}