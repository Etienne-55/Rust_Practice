use std::io


fn main() {

    println!("Enter the function: ");
    let mut function = String::new();
    io::stdin().read_line(&mut function).expect("failed to read line");
    let function: f64 = fucntion.trim().parse().expect("Please enter a valid function");


    println!("Enter the value of F(n): ");
    let mut value_of_n = String::new();
    io::stdin().read_line(&mut function).expect("failed to read line");
    let value_of_n: f64 = fucntion.trim().parse().expect("Please enter a valid function");


    println!("Enter the first const: ");
    let mut const1 = String::new();
    io::stdin().read_line(&mut function).expect("failed to read line");
    let const1: f64 = fucntion.trim().parse().expect("Please enter a valid function");



    println!("Enter the secont const: ");
    let mut const2 = String::new();
    io::stdin().read_line(&mut function).expect("failed to read line");
    let const2: f64 = fucntion.trim().parse().expect("Please enter a valid function");


}
