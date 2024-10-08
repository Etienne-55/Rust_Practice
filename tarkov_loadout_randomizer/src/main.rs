use std::io;

mod mission_maker;
mod loadout_maker;

fn main() {
    loop {
        println!("Tarkov randomizer(get a random loadout or mission: ");
        println!("1->Get a random weapon loadout. ");
        println!("2->Get a random mission. ");
        println!("3->Exit. ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim(){

            "1" => {loadout_maker::main_loadout_randomizer();
            }

            "2" => {mission_maker::main_map_randomizer();
            }

            "3" => {
                println!("Quitting...");
                break;
            },
            _=> println!("Invalid choice.")

        }
    }

}
