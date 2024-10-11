use std::io;

fn evaluate_polynomial(polynomial: &str, n: f64) -> f64 {
    let terms: Vec<&str> = polynomial.split('+').collect();
    let mut result = 0.0;

    for term in terms {
        let term = term.trim();
        let mut coeff = 1.0;
        let mut power = 0;

        // Check for 'n' and extract coefficient and power
        if term.contains("n") {
            let parts: Vec<&str> = term.split('*').collect();
            for part in parts {
                if part.contains('n') {
                    if let Some(p) = part.split('^').last() {
                        power = p.parse::<i32>().unwrap_or(1);
                    } else {
                        power = 1;
                    }
                } else {
                    coeff = part.parse::<f64>().unwrap_or(1.0);
                }
            }
        } else {
            coeff = term.parse::<f64>().unwrap();
        }

        // Calculate the term value and add it to the result
        result += coeff * n.powi(power);
    }

    result
}
fn main() {

    let mut polynomial = String::new();
    println!("Enter a the function to determine complexity: ");
    io::stdin().read_line(&mut polynomial).unwrap();
    let polynomial = polynomial.trim();


    println!("Enter the first const: ");
    let mut const1 = String::new();
    io::stdin().read_line(&mut const1).expect("failed to read line");
    let const1: f64 = const1.trim().parse().expect("Please enter a valid function");



    println!("Enter the secont const: ");
    let mut const2 = String::new();
    io::stdin().read_line(&mut const2).expect("failed to read line");
    let const2: f64 = const2.trim().parse().expect("Please enter a valid function");


    let n_one: f64 = 1.0;
    let n_two: f64 = 2.0;
    let n_three: f64 = 3.0;
    let n_four: f64 = 4.0;
    let n_five: f64 = 5.0;


    let result1 = evaluate_polynomial(&polynomial.trim(), n_one);
    let const1_n1 = const1 * n_one;
    let const2_n1 = const2 * n_one;

    let result2 = evaluate_polynomial(&polynomial.trim(), n_two);
    let const1_n2 = const1 * n_two;
    let const2_n2 = const2 * n_two;

    let result3 = evaluate_polynomial(&polynomial.trim(), n_three);
    let const1_n3 = const1 * n_three;
    let const2_n3 = const2 * n_three;

    let result4 = evaluate_polynomial(&polynomial.trim(), n_four);
    let const1_n4 = const1 * n_four;
    let const2_n4 = const2 * n_four;

    let result5 = evaluate_polynomial(&polynomial.trim(), n_five);
    let const1_n5 = const1 * n_five;
    let const2_n5 = const2 * n_five;

    println!("{}", polynomial);
    println!("1 --- c1: {} --- f(n): {} --- c2: {}", const1_n1, result1, const2_n1);
    println!("2 --- c1: {} --- f(n): {} --- c2: {}", const1_n2, result2, const2_n2);
    println!("3 --- c1: {} --- f(n): {} --- c2: {}", const1_n3, result3, const2_n3);
    println!("4 --- c1: {} --- f(n): {} --- c2: {}", const1_n4, result4, const2_n4);
    println!("5 --- c1: {} --- f(n): {} --- c2: {}", const1_n5, result5, const2_n5);
}
