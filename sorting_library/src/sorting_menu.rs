use std::io;

pub fn sorting_options() {
    loop{
        println!("choose a sorting method. ");
        println!("1: Bubble sort:");
        println!("2: Quick sort:");
        println!("3: Insertion sort: ");
        println!("4: Exit. ");


        let mut choice2 = String::new();
        io::stdin().read_line(&mut choice2).expect("Failed to read line");

        match choice2.trim(){

        "" => {}

        "4" => {
            break;
        },
        _=> println!("Invalid choice")



        }
    }
}
