use std::io;

mod mission_maker;

fn main() {
    loop {
        println!("Tarkov randomizer(get a random loadout or mission: ");
        println!("1->Get a random ar loadout. ");
        println!("2->Get a random mission. ");
        println!("3->Exit. ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim(){
            "1" => {mission_maker::main_randomizer(); 
            }

            "3" => {
                println!("Quitting...");
                break;
            },
            _=> println!("Invalid choice.")

        }
    }

}