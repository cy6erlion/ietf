extern crate dirs_next;
extern crate pager;

use pager::Pager;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

mod fetch;

pub fn list_view() {
    if !index_exists().unwrap() {
        // Download all RFCs
        fetch::index().unwrap();
    }

    let path = if let Some(home_path) = dirs_next::home_dir() {
        if cfg!(unix) || cfg!(macos) {
            format!("{}/rfc/INDEX", home_path.to_str().unwrap())
        } else if cfg!(windows) {
            format!("{}\\rfc\\INDEX", home_path.to_str().unwrap())
        } else {
            panic!("Unsupported OS");
        }
    } else {
        panic!("No home directory");
    };

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
pub fn read_rfc(sn: u32) {
    // check if RFC is downloaded
    if !is_rfc_downloaded(sn).unwrap() {
        // download RFC
        println!("Downloading rfc");
        fetch::rfc(sn).unwrap();
    }

    let path = if let Some(home_path) = dirs_next::home_dir() {
        if cfg!(unix) || cfg!(macos) {
            format!("{}/rfc/{}", home_path.to_str().unwrap(), sn)
        } else if cfg!(windows) {
            format!("{}\\rfc\\{}", home_path.to_str().unwrap(), sn)
        } else {
            panic!("Unsupported OS");
        }
    } else {
        panic!("No home directory");
    };

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

pub fn clean() -> () {
    if let Some(home_path) = dirs_next::home_dir() {
        let path = if cfg!(unix) || cfg!(macos) {
            format!("{}/rfc", home_path.to_str().unwrap())
        } else if cfg!(windows) {
            format!("{}\\rfc", home_path.to_str().unwrap())
        } else {
            panic!("Unsupported OS");
        };

        if Path::new(&path).exists() {
            std::fs::remove_dir_all(&path).unwrap();
        }
    } else {
        panic!("Could not find home directory");
    }
}

// Check if it is first time running by
// checking if config files exist
fn index_exists() -> Result<bool, ()> {
    if let Some(home_path) = dirs_next::home_dir() {
        let path = if cfg!(unix) || cfg!(macos) {
            format!("{}/rfc/INDEX", home_path.to_str().unwrap())
        } else if cfg!(windows) {
            format!("{}\\rfc\\INDEX", home_path.to_str().unwrap())
        } else {
            panic!("Unsupported OS");
        };

        if Path::new(&path).exists() {
            Ok(true)
        } else {
            init_storage_sir().unwrap();
            Ok(false)
        }
    } else {
        panic!("Could not find home directory");
    }
}

// Check if an RFC has been downloaded locally
fn is_rfc_downloaded(sn: u32) -> Result<bool, ()> {
    if let Some(home_path) = dirs_next::home_dir() {
        let path = if cfg!(unix) || cfg!(macos) {
            format!("{}/rfc/{}", home_path.to_str().unwrap(), sn)
        } else if cfg!(windows) {
            format!("{}\\rfc\\{}", home_path.to_str().unwrap(), sn)
        } else {
            panic!("Unsupported OS");
        };

        if Path::new(&path).exists() {
            Ok(true)
        } else {
            init_storage_sir().unwrap();
            Ok(false)
        }
    } else {
        panic!("Could not find home directory");
    }
}

// Check and create storage directory
fn init_storage_sir() -> std::io::Result<()> {
    if let Some(home_path) = dirs_next::home_dir() {
        let path = if cfg!(unix) || cfg!(macos) {
            format!("{}/rfc", home_path.to_str().unwrap())
        } else if cfg!(windows) {
            format!("{}\\rfc", home_path.to_str().unwrap())
        } else {
            panic!("Unsupported OS");
        };

        if Path::new(&path).exists() {
            Ok(())
        } else {
            std::fs::create_dir(path)?;
            Ok(())
        }
    } else {
        panic!("Could not find home directory");
    }
}
