use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

// Download RFC index file
pub fn index() -> Result<(), minreq::Error> {
    println!("Fetching RFC index");
    let response = minreq::get("https://www.rfc-editor.org/rfc-index.txt").send()?;
    let data = scrape(response.as_str()?);
    persist_index(data);
    Ok(())
}

// Download RFC localy
pub fn rfc(sn: u32) -> Result<(), minreq::Error> {
    println!("Fetching RFC #{}", sn);
    let address = format!("https://www.rfc-editor.org/rfc/rfc{}.txt", sn);

    println!("{}", address);
    let response = minreq::get(&address).send()?;
    persist_rfc(sn, response.as_str()?);
    Ok(())
}

// Save index localy
pub fn persist_index(index: Vec<String>) {
    if let Some(home_path) = dirs_next::home_dir() {
        let path = if cfg!(unix) || cfg!(macos) {
            format!("{}/rfc/INDEX", home_path.to_str().unwrap())
        } else if cfg!(windows) {
            format!("{}\\rfc\\INDEX", home_path.to_str().unwrap())
        } else {
            panic!("Unsupported OS");
        };

        let _file = File::create(&path).expect("Unable to create file");
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&path)
            .unwrap();

        for rfc in index.iter() {
            if let Err(e) = writeln!(file, "{}", rfc) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    } else {
        panic!("Could not find home directory");
    }
}

// Save RFC localy
pub fn persist_rfc(sn: u32, rfc: &str) {
    if let Some(home_path) = dirs_next::home_dir() {
        let path = if cfg!(unix) || cfg!(macos) {
            format!("{}/rfc/{}", home_path.to_str().unwrap(), sn)
        } else if cfg!(windows) {
            format!("{}\\rfc\\{}", home_path.to_str().unwrap(), sn)
        } else {
            panic!("Unsupported OS");
        };

        let _file = File::create(&path).expect("Unable to create file");
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(path)
            .unwrap();

        if let Err(e) = writeln!(file, "{}", rfc) {
            eprintln!("Couldn't write to file: {}", e);
        }
    } else {
        panic!("Could not find home directory");
    }
}

// TODO: fix bug causing not to return the last RFC
pub fn scrape(data: &str) -> Vec<String> {
    let mut count = 0;
    let mut rfcs = vec![];
    let mut buff = String::from("");

    for line in data.lines() {
        // Skip first 65 lines
        if count > 65 {
            // Detect blank lines
            if line == "" {
                rfcs.push(buff);
                buff = String::from("");
            } else {
                buff = format!("{}{}", buff, line);
            }
        } else {
            count += 1;
        }
    }

    rfcs
}
