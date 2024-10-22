use std::io;


struct PetInfo {
    specie: String,
    pet_age: String,
    weight: String,
}

fn main() {


    let mut pet_infos: Vec<String> = Vec::new();

    loop {
        println!("1---Create pet profile.");
        println!("2---Show pet profile list.");
        println!("3---Delete pet profile.");
        println!("4---Exit.");
    
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {

        "1" => { 
            let pet_info = get_pet_info();
            let pet_info_string = format!("Specie {}, Age {}, Weight {}",  pet_info.specie, pet_info.pet_age, pet_info.weight);
            pet_infos.push(pet_info_string);
        }

        "2" => { println!("{:?}", pet_infos);
        } 

        "4" =>{
            println!("Quitting...");
            break;
        },
        _=>println!("Invalid choise.")
        }
    }
}


fn get_pet_info() -> PetInfo {

    println!("Enter pet/customer stats.");

    println!("Animal type: ");
    let mut specie = String::new();
    io::stdin().read_line(&mut specie).expect("Failed to read line");
    let specie = specie.trim().to_string();

    println!("Animal age: ");
    let mut pet_age = String::new();
    io::stdin().read_line(&mut pet_age).expect("Failed to read line");
    let pet_age = pet_age.trim().to_string();
    

    println!("Animal weight: ");
    let mut weight = String::new();
    io::stdin().read_line(&mut weight).expect("Failed to read line");
    let weight = weight.trim().to_string();


    PetInfo { specie, pet_age, weight }

}
