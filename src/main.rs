// main.rs
// Date: Thu Nov 05 2026
// Educational and Practice Rust Programming Language Code

// Project: Book Library V2
// Goal: Learning Packaging and MVC model
// Dependency: Without any dependency

// rustc 1.101.0-nightly (2d20b42b8 2026-11-04)
// binary: rustc
// commit-hash: 2d20b42b883dcfc014f3c2c56081eeb6091c2179
// commit-date: 2026-11-04
// host: x86_64-unknown-linux-gnu
// release: 1.101.0-nightly
// LLVM version: 23.1.5

// cargo 1.101.0-nightly (3c747ea3e 2026-11-03)
// release: 1.101.0-nightly
// commit-hash: 3c747ea3ead23951c1b43a5edfcbdb449135ddeb
// commit-date: 2026-11-03
// host: x86_64-unknown-linux-gnu
// libgit2: 1.9.6 (sys:0.21.0 vendored)
// libcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)
// ssl: OpenSSL 3.6.3 9 Jun 2026
// os: Fedora 44.0.0 [64-bit]

// Kernel Version: 7.2.15-200.fc44.x86_64
// Firmware Version: 71CN51WW(V1.21)

use book_library::BookRepository;
use book_library::BookView;
use book_library::MenuComm;
use book_library::display_menu;
use book_library::display_message;

fn main() {
    println!("\n");

    let br = BookRepository::new();
    let mut bv = BookView::new(br);

    'outer: loop {
        match display_menu() {
            MenuComm::AddBook => bv.new_existence(),
            MenuComm::EditBook => bv.new_survive(),
            MenuComm::RemoveBook => bv.inexistence(),
            MenuComm::FineBook => bv.demand(),
            MenuComm::ListBook => bv.mass_expose(),
            MenuComm::Exit => break 'outer,
            MenuComm::Unknown => display_message("Unknown Command ..."),
        }
    }
    println!("\nThe End ...\n");
}
