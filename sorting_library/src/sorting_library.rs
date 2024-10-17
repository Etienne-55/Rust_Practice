use std::io;



//pub fn bubble_sort(arr: &mut[type of variable]) {
//    let mut n = arr.len();
//    while n > 0 {
//        let  mut last_modified_index = 0;
//        for i in 1..n {
//            if arr[i - 1] > arr[i] {
//                arr.swap(i - 1, i);
//                last_modified_index = i;
//            }
//        }
//        n = last_modified_index;
///    }

//}


pub fn get_user_info () {
    let mut name = String::new();
    let mut age_user = String::new();

    println!("Enter user name: ");
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string();

    println!("Enter user age: ");
    io::stdin().read_line(&mut age_user).expect("Failed to read line");
    let age_user = age_user.trim().to_string();
    

    UserInfo { name, age}

}




pub fn add_name(name_list: &mut Vec<String>) {

    println!("Enter a name to add: ");
    io::stdout();

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    name_list.push(name.trim().to_string());

    println!("Name added!");

}

pub fn show_names(name_list: &Vec<String>) {
    
    if name_list.is_empty() {
        println!("The list is empty");


    } else {
        for (index, name) in name_list.iter().enumerate() {
            println!("{}: {}", index, name);
        }
    }
}

pub fn remove_name(name_list: &mut Vec<String>) {
    show_names(name_list);

    if name_list.is_empty() {
        return;

    }

    println!("Enter the index of the name you want to remove: ");
    io::stdout();

    let mut index_str = String::new();
    io::stdin().read_line(&mut index_str).expect("Failed to read line.");

    let index: usize = match index_str.trim().parse() {
        Ok(i) => i,
        Err(_) => {
            println!("Invalid index.");
            return;
        }
    };

    if index < name_list.len() {
        name_list.remove(index);
        println!("Name removed!");
    } else {
        println!("Index out of bounds");
    }
}
