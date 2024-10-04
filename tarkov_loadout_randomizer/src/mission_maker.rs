use rand::{seq::SliceRandom, Rng}; // 0.7.2

pub fn spawn_randomizer() -> String {
    let entry_map = vec!["Customs", "Reserve", "Lighthouse", "Ground zero", "Woods", "Shoreline", "Interchange", "Streets", "Labs", "Factory"]; 
    let mut rng = rand::thread_rng();
    entry_map.choose(&mut rng).unwrap().to_string()
}


pub fn mission_randomizer() -> String {
    let mission = vec!["Undercover: Collect russian military intel,", "True beliver: kill the cultist priest", "Vengeance: kill two rogue agents"];
    let mut rng = rand::thread_rng();
    mission.choose(&mut rng).unwrap().to_string()
}


pub fn exfil_randomizer() -> String {
    let exfil_map = vec!["Customs", "Reserve", "Lighthouse", "Ground zero", "Woods", "Shoreline", "Interchange", "Streets", "Labs", "Factory"]; 
    let mut rng = rand::thread_rng();
    exfil_map.choose(&mut rng).unwrap().to_string()
}

pub fn main_randomizer() {

    let spawn = spawn_randomizer();
    let objective = mission_randomizer();
    let exit = exfil_randomizer();

    println!("Spawn on {}, {} and extract from {}", spawn, objective, exit);

}
