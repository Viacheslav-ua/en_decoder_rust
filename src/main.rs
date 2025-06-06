use std::{io::{BufWriter, Read, Write}, num::IntErrorKind, u8};

fn get_input(query: &str)->String{
    print!("{query}: ");
    std::io::stdout().flush().unwrap();

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    buffer.trim().to_owned()
}

fn process_file_data(data: Vec<u8>, key: u8) -> Vec<u8> {
    let mut processed_data = Vec::with_capacity(data.len());
    for byte in data {
        processed_data.push(byte ^ key);
    }
    processed_data
}

fn main() {
    loop {
        println!("# # # # # # # # # # # # #");
        let input_file_name = get_input("Enter filename to process");
        let input_file = match std::fs::File::open(&input_file_name) {
            Ok(file) => file,
            Err(_) => {
                println!("Error: Could not open file '{}'. Please try again.\n", input_file_name);
                continue;
            }
        };

        let key = match get_input("Enter key for file encryption/decryption: ")
            .parse::<u8>() {
            Ok(k) => k,
            Err(err) => {
                match err.kind() {
                    IntErrorKind::Empty => println!("Error: Key cannot be empty. Please try again.\n"),
                    IntErrorKind::InvalidDigit => println!("Error: Key must be a valid number. Please try again.\n"),
                    IntErrorKind::PosOverflow => println!("Error: Key is too large. Please enter a number between 1 and 255.\n"),
                    _ => println!("Error: Invalid key input. Please try again.\n"),
                }
                continue;
            }
        };

        if key == 0 {
            println!("Error: Key cannot be zero. Please enter a number between 1 and 255.\n");
            continue;
        }

        let mut reader = std::io::BufReader::new(input_file);
        let mut input_data = Vec::new();

        if let Err(err) = reader.read_to_end(&mut input_data) {
            println!("Error reading file: {}\n", err);
            continue;
        }

        let processed_data = process_file_data(input_data, key);

        let output_file_name = get_input("Enter output filename: ");
        let output_file = match std::fs::File::create(&output_file_name) {
            Ok(file) => file,
            Err(_) => {
                println!("Error: Could not create output file '{}'. Please try again.\n", output_file_name);
                continue;
            }
        };

        let mut writer = BufWriter::new(output_file);
        if let Err(err) = writer.write_all(&processed_data) {
            println!("Error writing to output file: {}\n", err);
            continue;
        }

    }
}