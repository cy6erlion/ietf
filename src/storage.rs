use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

// Save index localy
pub fn persist_index(index: Vec<String>) {
    let home_path = if let Some(p) = get_home_path() {
        p
    } else {
        panic!("Error: 'Could not find home directory!'");
    };

    let index_file_path = format!("{}INDEX", home_path);

    let _file = File::create(&index_file_path).expect("Unable to create file");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&index_file_path)
        .unwrap();

    for rfc in index.iter() {
        if let Err(e) = writeln!(file, "{}", rfc) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

// Save RFC localy
pub fn persist_rfc(rfc_number: u32, rfc: &str) {
    let home_path = if let Some(p) = get_home_path() {
        p
    } else {
        panic!("Error: 'Could not find home directory!'");
    };

    let rfc_file_path = format!("{}{}", home_path, rfc_number);

    let _file = File::create(&rfc_file_path).expect("Unable to create file");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(rfc_file_path)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", rfc) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

// Removes RFC by Serial Number
pub fn remove(rfc_number: u32) {
    let home_path = if let Some(p) = get_home_path() {
        p
    } else {
        panic!("Error: 'Could not find home directory!'");
    };

    let rfc_file_path = format!("{}{}", home_path, rfc_number);

    if Path::new(&rfc_file_path).exists() {
        std::fs::remove_file(&rfc_file_path).unwrap();
    }
}

// Removes the rfc directory
pub fn clean() -> () {
    let rfc_directory_path = if let Some(p) = get_home_path() {
        p
    } else {
        panic!("Error: 'Could not find home directory!'");
    };

    if Path::new(&rfc_directory_path).exists() {
        std::fs::remove_dir_all(&rfc_directory_path).unwrap();
    }
}

// Check if an RFC file has been downloaded locally
pub fn is_rfc_downloaded(rfc_number: u32) -> Result<bool, ()> {
    let home_path = if let Some(p) = get_home_path() {
        p
    } else {
        panic!("Error: 'Could not find home directory!'");
    };

    let rfc_file_path = format!("{}{}", &home_path, rfc_number);

    if Path::new(&rfc_file_path).exists() {
        Ok(true)
    } else {
        initialize_storage().unwrap();
        Ok(false)
    }
}

// Check if storage directory (~/rfc) has been created
// if it does not, create the directory
fn initialize_storage() -> std::io::Result<()> {
    let home_path = if let Some(p) = get_home_path() {
        p
    } else {
        panic!("Error: 'Could not find home directory!'");
    };

    if Path::new(&home_path).exists() {
        Ok(())
    } else {
        std::fs::create_dir(&home_path)?;
        Ok(())
    }
}

// Check if the rfc INDEX file has been downloaded
pub fn index_exists() -> Result<bool, ()> {
    let home_path = if let Some(p) = get_home_path() {
        p
    } else {
        panic!("Error: 'Could not find home directory!'");
    };

    let index_file_path = format!("{}INDEX", home_path);

    if Path::new(&index_file_path).exists() {
        Ok(true)
    } else {
        initialize_storage().unwrap();
        Ok(false)
    }
}

// Get path of home directory
// (`~/rfc/` on unix systems and `C:\Users\{NAME}` on windows)
pub fn get_home_path() -> Option<String> {
    if let Some(home_path) = dirs_next::home_dir() {
        let path = if cfg!(unix) || cfg!(macos) {
            format!("{}/rfc/", home_path.to_str().unwrap())
        } else if cfg!(windows) {
            format!("{}\\rfc\\", home_path.to_str().unwrap())
        } else {
            panic!("Unsupported OS");
        };

        Some(path)
    } else {
        None
    }
}
