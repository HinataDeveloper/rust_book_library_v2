use std::io::Write;

pub enum MenuComm {
    AddBook,
    EditBook,
    FineBook,
    RemoveBook,
    ListBook,
    Exit,
    Unknown,
}

fn get_menu_command() -> MenuComm {
    print!(" -> Enter command number: ");
    std::io::stdout().flush().unwrap();
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("--->>> Error to read from user ...");
    match buffer.trim() {
        "1" => MenuComm::AddBook,
        "2" => MenuComm::EditBook,
        "3" => MenuComm::FineBook,
        "4" => MenuComm::RemoveBook,
        "5" => MenuComm::ListBook,
        "6" => MenuComm::Exit,
        _ => MenuComm::Unknown,
    }
}

pub fn get_string(msg: impl AsRef<str>) -> String {
    let message = msg.as_ref();
    print!(" -> {message}: ");
    std::io::stdout().flush().unwrap();
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("--->>> Error to read from user ...");
    buffer.trim().to_string()
}

pub fn display_menu() -> MenuComm {
    println!("\n");
    println!(" -> 1 - Add Book.");
    println!(" -> 2 - Edit Book.");
    println!(" -> 3 - find Book.");
    println!(" -> 4 - Remove Book.");
    println!(" -> 5 - List Books.");
    println!(" -> 0 - Exit.");
    get_menu_command()
}
