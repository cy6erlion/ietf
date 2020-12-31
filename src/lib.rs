extern crate dirs_next;
extern crate pager;

use pager::Pager;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

mod fetch;
pub mod storage;

pub fn list_view() {
    if !storage::index_exists().unwrap() {
        // Download all RFCs
        fetch::index().unwrap();
    }

    let home_path = if let Some(p) = storage::get_home_path() {
        p
    } else {
        panic!("Error: 'Could not find home directory!'");
    };

    let path = format!("{}INDEX", home_path);

    let mut index = String::new();
    let f = File::open(&path).expect("Unable to open file");
    let mut br = BufReader::new(f);
    let mut dots = "";

    br.read_to_string(&mut index).expect("Unable to read INDEX");

    Pager::with_pager("less -r").setup();

    for line in index.lines() {
        let line_words: Vec<&str> = line.split(' ').collect();
        let summerize: String = line.chars().skip(line_words[0].len()).take(77).collect();

        if line.len() >= 77 {
            dots = "...";
        }

        println!("{} | {}{}", line_words[0], summerize, dots);

        dots = "";
    }
}

// Read RFC by serial number
pub fn read_rfc(rfc_number: u32) {
    // check if RFC is downloaded
    if !storage::is_rfc_downloaded(rfc_number).unwrap() {
        // download RFC
        fetch::rfc(rfc_number).unwrap();
    }

    let home_path = if let Some(p) = storage::get_home_path() {
        p
    } else {
        panic!("Error: 'Could not find home directory!'");
    };

    let path = format!("{}{}", home_path, rfc_number);

    let mut rfc = String::new();
    let f = File::open(&path).expect("Unable to open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut rfc).expect("Unable to read RFC");

    // Read RFC
    Pager::with_pager("less -r").setup();
    println!("{}", &rfc);
}

// Update RFC
pub fn update() {
    fetch::index().unwrap();
}
