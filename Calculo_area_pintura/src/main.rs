use std::io;


fn main() {
    println!("Informe a área a ser pintada para saber qual opção de tinta comprar: ");
    let mut area = String::new();
    io::stdin().read_line(&mut area).expect("Failed to read line.");
    let mut area: f64 = area.trim().parse().expect("Please enter valid number.");

    let area_galao_pequeno: f64 = 3.6 * 8.0;
    let area_galao_grande: f64= 18.0 * 8.0;

    let area_total_p: f64 = area_galao_pequeno * area;
    let area_total_g: f64 = area_galao_grande * area;

    if area_total_p < area_total_g {
        println!("Compre galão de 18 litros.");
    } else {
        println!("Compre galão de 3,6 litros.");
    } 
}
