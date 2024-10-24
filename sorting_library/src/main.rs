use std::io;

mod sorting_menu;
mod sorting_library;

fn main(){
    
    let mut name_list: Vec<String> = Vec::new();

    loop {
        println!("Sorting library. ");            
        println!("1: Add a name to the list. ");            
        println!("2: Show the list. ");            
        println!("3: Remove a name. ");            
        println!("4: Sorting options. ");            
        println!("5: Exit. ");             

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

        "5" => {
            println!("Quitting...");
            break;

        },
        _=> println!("Invalid choice")
       

        }
    }
}
