use std::fs::OpenOptions;
use std::io;
use std::io::{BufRead, BufReader};
use crate::fs::get_file_content;
use crate::lib::Command;

mod lib;
mod fs;

fn main() {
    println!("Hello, world!");
    let mut books = get_file_content();
    let mut book_id = 0;


   //get_file_content();

    loop {
        println!("Please enter a command:");
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let mut lib_command = Command::List;

        // Komut ve kitap işlemleri
        match command.trim() {
            "exit" => std::process::exit(0),
            "add" => {
                lib_command = Command::Add;
            },
            "list" => {
                lib_command = Command::List;
            },
            "remove" => {
                lib_command = Command::Remove;
            }
            "vector" => {
                println!("Vector: {:?}", books);
                println!("Miras alınan vector: {:?}", get_file_content());
            }
            _ => {
                println!("Invalid command");
            }
        }

        match lib_command {
            Command::Add => {
                println!("Kitap bilgisi alınıyor!");
                println!("Kitap adı girin:");
                let mut book_name = String::new();
                io::stdin()
                    .read_line(&mut book_name)
                    .expect("Failed to read line");

                println!("Kitap yazarını girin");
                let mut book_author =  String::new();
                io::stdin()
                    .read_line(&mut book_author)
                    .expect("Failed to read line");

                let my_book = lib::Book {
                    name: book_name.trim().to_string(),
                    author: book_author.trim().to_string(),
                    date: String::from("2024"),
                    id: book_id
                };
                println!("{:?}, {:?}", my_book, books);

                books.push(my_book);
                book_id += 1;


                let _ = fs::fs("write".to_string(), &books);
            },
            Command::Remove => {
                println!("Kitap silinecek!");
                println!("Kitap ID girin:");
                let mut book_id_str = String::new();
                io::stdin()
                    .read_line(&mut book_id_str)
                    .expect("Failed to read line");

                let book_id = book_id_str.trim().parse::<i32>().unwrap();
                books.retain(|book| book.id != book_id);

                println!("Kitap silindi: {:?}", books);
            },
            Command::List => {
                println!("Kitaplar listelenecek!");
                println!("Vector: {:?}", books);
            },
        }
    }
}
