// Download RFC index file
pub fn index() -> Result<Vec<String>, minreq::Error> {
    println!("Fetching RFC index");
    let response = minreq::get("https://www.rfc-editor.org/rfc-index.txt").send()?;
    let data = scrape(response.as_str()?);
    Ok(data)
}

// Download RFC localy
pub fn rfc(sn: u32) -> Result<String, minreq::Error> {
    println!("Fetching RFC #{}", sn);
    let address = format!("https://www.rfc-editor.org/rfc/rfc{}.txt", sn);

    println!("{}", address);
    let response = minreq::get(&address).send()?;
    Ok(String::from(response.as_str()?))
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
