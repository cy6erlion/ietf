use clap::{App, Arg, SubCommand};
extern crate pager;

mod fetch;
pub mod storage;

use pager::Pager;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let matches = App::new("ietf")
        .version("0.1.0")
        .about("A program to read RFCs in the terminal.")
        .before_help("██▄██ ▄▄█▄ ▄█ ▄▄\n██ ▄█ ▄▄██ ██ ▄█\n█▄▄▄█▄▄▄██▄██▄██\n▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀")
        .arg(
            Arg::with_name("Number")
                .short("n")
                .long("number")
                .value_name("serial")
                .help("RFC Serial Number")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("Remove")
                .short("r")
                .long("remove")
                .value_name("serial")
                .help("RFC Serial Number")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name("update").about("Update RFC Index"))
        .subcommand(SubCommand::with_name("clean").about("Remove the rfc directory"))
        .get_matches();

    let storage = storage::Storage::new();

    // Read RFC by serial number
    if let Some(n) = matches.value_of("Number") {
        let rfc_number = n.parse::<u32>().unwrap();

        // check if RFC is downloaded
        if !storage.is_rfc_downloaded(rfc_number).unwrap() {
            // download RFC
            let rfc_data = fetch::rfc(rfc_number).unwrap();

            // persist RFC
            storage.persist_rfc(rfc_number, &rfc_data);
        }

        let rfc_file_path = format!("{}{}", storage.rfc_dir_path, rfc_number);

        let mut rfc_data = String::new();
        let index_file = File::open(&rfc_file_path).expect("Unable to open file");
        let mut buffer_reader = BufReader::new(index_file);
        buffer_reader
            .read_to_string(&mut rfc_data)
            .expect("Unable to read RFC");

        Pager::with_pager("less -r").setup();
        println!("{}", &rfc_data);
        return;
    }

    // Removes RFC by serial number
    if let Some(n) = matches.value_of("Remove") {
        storage.remove(
            n.parse::<u32>()
                .expect("RFC Serial Number should be a numeric value!"),
        );
        return;
    }

    // Update RFC index
    if let Some(_matches) = matches.subcommand_matches("update") {
        storage.update_index();
        return;
    }

    // Remove the ietf directory
    if let Some(_matches) = matches.subcommand_matches("clean") {
        storage.clean();
        return;
    }

    // ---------- Display RFC list view ------------
    let mut index_data = String::new();
    let index_file = File::open(&storage.index_file_path).expect("Unable to open file");
    let mut buffer_reader = BufReader::new(index_file);
    let mut read_more_dots = "";

    buffer_reader
        .read_to_string(&mut index_data)
        .expect("Unable to read INDEX");

    Pager::with_pager("less -r").setup();

    for line in index_data.lines() {
        let line_words: Vec<&str> = line.split(' ').collect();
        let summerize: String = line.chars().skip(line_words[0].len()).take(77).collect();

        if line.len() >= 77 {
            read_more_dots = "...";
        }

        println!("{} | {}{}", line_words[0], summerize, read_more_dots);

        read_more_dots = "";
    }
}
