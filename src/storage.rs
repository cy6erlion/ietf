use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

pub struct Storage {
    pub rfc_dir_path: String,
    pub index_file_path: String,
}

impl Storage {
    // Create new Storage instance
    pub fn new() -> Self {
        if let Some(home_path) = dirs_next::home_dir() {
            let rfc_dir_path = if cfg!(unix) || cfg!(macos) {
                format!("{}/rfc/", home_path.to_str().unwrap())
            } else if cfg!(windows) {
                format!("{}\\rfc\\", home_path.to_str().unwrap())
            } else {
                panic!("Unsupported OS");
            };

            // Check if storage directory (~/rfc) has been created
            // otherwise create storage directory
            if !Path::new(&rfc_dir_path).exists() {
                // create RFC storage directory
                std::fs::create_dir(&rfc_dir_path).unwrap();
            }

            let index_file_path = format!("{}INDEX", &rfc_dir_path);

            // Check if RFC INDEX file is downloaded
            if !Path::new(&index_file_path).exists() {
                let _create_index =
                    File::create(&index_file_path).expect("Unable to creat INDEX file");

                // Fetch remote INDEX file data
                let data = super::fetch::index().unwrap();

                // Persist RFC INDEX
                Storage::persist_index(&index_file_path, data);
            }

            Storage {
                rfc_dir_path,
                index_file_path,
            }
        } else {
            panic!("Error: 'Could not find home directory!'");
        }
    }

    // Save index localy
    pub fn persist_index(index_file_path: &str, data: Vec<String>) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&index_file_path)
            .unwrap();

        for rfc in data.iter() {
            if let Err(e) = writeln!(file, "{}", rfc) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }

    // Update index
    pub fn update_index(&self) {
        if Path::new(&self.index_file_path).exists() {
            std::fs::remove_file(&self.index_file_path).unwrap();
        }

        let _create_index =
            File::create(&self.index_file_path).expect("Unable to creat INDEX file");

        // Fetch remote INDEX file data
        let data = super::fetch::index().unwrap();

        // Persist RFC INDEX
        Storage::persist_index(&self.index_file_path, data);
    }

    // Check if an RFC file has been downloaded locally
    pub fn is_rfc_downloaded(&self, rfc_number: u32) -> Result<bool, ()> {
        let rfc_file_path = format!("{}{}", self.rfc_dir_path, rfc_number);

        if Path::new(&rfc_file_path).exists() {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // Save RFC localy
    pub fn persist_rfc(&self, rfc_number: u32, rfc_data: &str) {
        let rfc_file_path = format!("{}{}", self.rfc_dir_path, rfc_number);

        let _file = File::create(&rfc_file_path).expect("Unable to create file");
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(rfc_file_path)
            .unwrap();

        if let Err(e) = writeln!(file, "{}", rfc_data) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    // Removes RFC by Serial Number
    pub fn remove(&self, rfc_number: u32) {
        let rfc_file_path = format!("{}{}", &self.rfc_dir_path, rfc_number);

        if Path::new(&rfc_file_path).exists() {
            std::fs::remove_file(&rfc_file_path).unwrap();
        }
    }

    // Removes the rfc directory
    pub fn clean(&self) {
        if Path::new(&self.rfc_dir_path).exists() {
            std::fs::remove_dir_all(&self.rfc_dir_path).unwrap();
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
}
