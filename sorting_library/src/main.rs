use std::io;

mod sorting_menu;
mod sorting_library;

fn main(){
    
    struct UserInfo {
        name: String,
        age: u32,
    }


    let mut name_list: Vec<String> = Vec::new();
    let mut user_list: Vec<String> = Vec::new();

    loop {
        println!("Sorting library. ");            
        println!("1: Add a name to the list. ");            
        println!("2: Show name list. ");            
        println!("3: Remove number from linked list. ");            
        println!("4: Sorting options. ");            
        println!("5: Exit. ");            
        println!("6: test struct. ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim(){

        "1" => { sorting_library::add_name(&mut name_list);
        }

        "2" => { sorting_library::show_names(&mut name_list);
        }
        
        "3" => { sorting_library::remove_name(&mut name_list);
        }

        "4" => {sorting_menu::sorting_options();
        }

        "6" => loop {
                let user_info = sorting_library::get_user_info();
                let info_string = format!("Name: {}, Age: {}", user_info.name, user_info.age_user);
                user_info.push(info_string);

                println!("Do you wanto to add a new user? y/n");
                let mut choice2 = String::new();
                io::stdin().read_line(&mut choice2).expect("failed to read line");

                if choice2.trim().to_lowercase() != "y" {
                    break;
               
                }
            }
        

        "5" => {
            println!("Quitting...");
            break;
        },
        _=> println!("Invalid choice")
       

        }
    }
}
