use std::io;

use crate::sorting_library::add_id_num;
use crate::sorting_library::show_even_id;
use crate::sorting_library::show_id;


pub fn sorting_options() {

    let mut id_list: Vec<i32> = Vec::new();

    loop{
        println!("choose a sorting method. ");
        println!("1: Add an id number: ");
        println!("2: Show id list: ");
        println!("3: Sort id list: ");
        println!("4: Show even id numbers:");
        println!("5: Show odd id numbers: ");
        println!("6: Exit. ");


        let mut choice2 = String::new();
        io::stdin().read_line(&mut choice2).expect("Failed to read line");

        match choice2.trim(){

        "1" => { add_id_num(&mut id_list);
        }

        "2" => { show_id(&mut id_list);
        }

        "4" => { show_even_id(&mut id_list);
        }

        "6" => {
            break;
        },
        _=> println!("Invalid choice")


        }
    }
}
