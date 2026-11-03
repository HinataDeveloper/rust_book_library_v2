use std::io::Write;

fn input_menu_command() -> u8 {
    print!("Enter command number: ");
    std::io::stdout().flush().unwrap();
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("--->>> Error to read from user ...");
    let result: u8 = buffer
        .trim()
        .parse()
        .expect("--->>> Error in parsing operaion ...");
    result
}

pub enum MenuComm {
    AddBook = 1,
    EditBook = 2,
    FineBook = 3,
    RemoveBook = 4,
    ListBook = 5,
    Exit = 0,
}

pub fn display_menu() -> u8 {
    println!("\n");
    println!(" -> 1 - Add Book.");
    println!(" -> 2 - Edit Book.");
    println!(" -> 3 - find Book.");
    println!(" -> 4 - Remove Book.");
    println!(" -> 5 - List Books.");
    println!(" -> 0 - Exit.");
    input_menu_command()
}
