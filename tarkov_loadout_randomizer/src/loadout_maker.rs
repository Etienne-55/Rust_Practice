use rand::{seq::SliceRandom, Rng}; 

pub fn gun_randomizer() -> String {
    let gun = vec!["m4a1", "ak-74m", "ash-12", "sr-25", "axmc"];
    let mut rng = rand::thread_rng();
    gun.choose(&mut rng).unwrap().to_string()
}



pub fn sight_randomizer() -> String {
    let sight = vec!["eotech vudu", "eotech holographic", "vortex razer", "smith&bend"];
    let mut rng = rand::thread_rng();
    sight.choose(&mut rng).unwrap().to_string()
}



pub fn ammo_randomizer() -> String {
    let ammo = vec!["fmj", "ap", "tracer"];
    let mut rng = rand::thread_rng();
    ammo.choose(&mut rng).unwrap().to_string()
}


pub fn main_loadout_randomizer() { 

    let rand_gun = gun_randomizer();
    let rand_sight = sight_randomizer();
    let rand_ammo = ammo_randomizer();

    println!("Use as main loadout a {}, with a {} sight and {} ammo, good luck!", rand_gun, rand_sight, rand_ammo);
}
