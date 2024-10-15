let mut name_list: Vec<String> = Vec::new();


fn add_name(name_list: &mut Vec<String>) {

    println!("Enter a name to add: ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    name_list.push(name.trim().to_string());

    println!("Name added!");

}

fn show_names(name_list: &Vec<String>) {

    if name_list.is_empty() {
        println!("The list is empty. ");

    } else {
        for (index, name) in name_list.iter().enumerate() {
            println!("{}: {}", index, name);

        }
    }
}

fn remove_name(name_list: &mut Vec<String>) {
    show_names(name_list);

    if name_list.is_empty() {
        return;

    }

    println!("Enter the index of the name to remove: ");
    io::stdout().flush().unwrap();

    let mut index_str = String::new();
    io::stdin().read_line(&mut index_str).expect("Failed to read line");


    let index: usize = match index_str.trim().parse() {
        ok(i) => i,
        Err(_) => {
            println!("Invalid index.");
            return;
        }
    };

    if index < name_list.len() {
        name_list.remove(index);
        println!("Name removed!");
    } else {
        println!("Index out of bounds.");
    }
}