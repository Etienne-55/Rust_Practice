use std::io;

mod sorting_menu;


fn main(){
    loop {
        println!("Sorting library. ");            
        println!("1: Add numbers to the linked list to be sorted. ");            
        println!("2: Show linked list. ");            
        println!("3: Remove number from linked list. ");            
        println!("4: Sorting options. ");            
        println!("5: Exit. ");            

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim(){

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
