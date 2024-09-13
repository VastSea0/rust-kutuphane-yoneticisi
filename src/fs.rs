use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Write, Error, Read, BufReader, BufRead};
use std::num::ParseIntError;
use crate::lib;
use crate::lib::Book;


fn main() {

}

pub fn fs(command: String, vector: &Vec<Book>) -> Result<(), Error> {

    let mut file = OpenOptions::new()
        .read(true)

        .append(true)
        .create(true) // Eğer dosya yoksa oluştur
        .open("helo.txt")
        .expect("Unable to open file");

    //let mut file = File::open("helo.txt").expect("File not found");



    match command.trim() {
        "read" => {
            println!("{:?}",file);
            let mut contents = String::new();

            file.read_to_string(&mut contents)
                .expect("Something went wrong reading the file");

            println!("File Content: {:?}", contents);
        },
        "delete" => {
            println!("{:?}",file);
            println!("helo");
            // Kullanıcı bu komutu kullanmasa daha iyi
            // let delete_dir_result = fs::remove_file("helo.txt");
            // match delete_dir_result {
            //     Ok(_) => println!("Successfully deleted directory"),
            //     Err(_) => println!("Error deleting directory: {:?}", delete_dir_result),
            //
            // }
        },
        "write" => {
            println!("{:?}",file);

            file.set_len(0).expect("Failed to truncate file");

            for book in vector {

                let content = format!("\n{} {} {} {}\n", book.name, book.author, book.date, book.id); // Kitap başlıklarını yazdırıyoruz
                file.write_all(content.as_bytes()).expect("Failed to write to file");
            }


        },

        _ => {
            println!("Unknown command");
        }
    }

    Ok(())
}


pub fn get_file_content() -> Vec<Book> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("helo.txt")
        .expect("Unable to open file");

    let reader = BufReader::new(file);

    let mut books = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(content) => {
                println!("{}: {}", index + 1, content);
                let parts: Vec<&str> = content.split_whitespace().collect();
                println!("parts: {:?}", parts);


                if parts.len() >= 4 {
                    let name = parts[0].to_string();
                    let author = parts[1].to_string();
                    let date = parts[2].to_string();
                    let id_result: Result<i32, ParseIntError> = parts[3].parse();

                    match id_result {
                        Ok(id) => {
                            let my_book = Book {
                                name,
                                author,
                                date,
                                id,
                            };
                            println!("{:?}", my_book);
                            books.push(my_book);
                        }
                        Err(e) => eprintln!("Error parsing ID: {}", e),
                    }
                } else {
                    eprintln!("Invalid line format: {}", content);
                }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    books
}