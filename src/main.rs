use std::io;
const VALUE_NOT_FOUND: isize = -1;

fn main() {
    let scientist_first_names = [
        "Tim", "Brendan", "Bill", "Hedy", "Barbara", "Elon", "Larry", "Carl", "Guido", "Mark",
    ];

    let scientist_surnames = [
        "Berners-Lee",
        "Eich",
        "Gates",
        "Lamarr",
        "Liskov",
        "Musk",
        "Page",
        "Sassenrath",
        "Van-Rassum",
        "Zuckerberg",
    ];

    let scientist_surname = enter_surname();

    let scientist_first_name_index = linear_search(&scientist_surnames, &scientist_surname);

    if scientist_first_name_index == VALUE_NOT_FOUND {
        println!("Computer scientist not present");
    } else {
        let scientist_first_name = scientist_first_names
            .iter()
            .nth(scientist_first_name_index as usize)
            .unwrap();

        println!("{scientist_first_name}");
    }
}

fn enter_surname() -> String {
    let mut input = String::new();

    println!("Enter a computer scientist surname");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_owned()
}

fn linear_search(array_to_search: &[&str], value_to_find: &str) -> isize {
    let value_to_find = value_to_find.to_lowercase();

    for index in 0..array_to_search.len() {
        let value = array_to_search[index].to_string();

        if value.to_lowercase() == value_to_find {
            return index as isize;
        }
    }

    VALUE_NOT_FOUND
}
