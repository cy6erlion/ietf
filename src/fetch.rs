// Download RFC index file
pub fn index() -> Result<Vec<String>, minreq::Error> {
    let response = minreq::get("https://www.rfc-editor.org/rfc-index.txt").send()?;
    let data = scrape(response.as_str()?);
    Ok(data)
}

// Download RFC localy
pub fn rfc(sn: u32) -> Result<String, minreq::Error> {
    let address = format!("https://www.rfc-editor.org/rfc/rfc{}.txt", sn);
    let response = minreq::get(&address).send()?;
    Ok(String::from(response.as_str()?))
}

pub fn scrape(data: &str) -> Vec<String> {
    let mut rfcs = vec![];
    let mut iter = data.lines().skip(66).peekable();

    while iter.peek().is_some() {
        rfcs.push(
            iter.by_ref()
                .take_while(|s| !s.is_empty())
                .fold(String::new(), |acc, s| acc + s),
        );
    }

    rfcs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_rfc_scrape() {
        let data = String::from("\n".repeat(66))
            + "8989 Additional Criteria for Nominating Committee Eligibility. B.\n"
            + "     Carpenter, S. Farrell. February 2021. (Format: HTML, TXT, PDF, XML)\n"
            + "     (Status: EXPERIMENTAL) (DOI: 10.17487/RFC8989) \n";

        let rfcs = vec![
            String::from("8989 Additional Criteria for Nominating Committee Eligibility. B.")
                + "     Carpenter, S. Farrell. February 2021. (Format: HTML, TXT, PDF, XML)"
                + "     (Status: EXPERIMENTAL) (DOI: 10.17487/RFC8989) ",
        ];

        assert_eq!(rfcs, scrape(&data));
    }

    #[test]
    fn multi_rfc_scrape() {
        let data = String::from("\n".repeat(66))
            + "8989 Additional Criteria for Nominating Committee Eligibility. B.\n"
            + "     Carpenter, S. Farrell. February 2021. (Format: HTML, TXT, PDF, XML)\n"
            + "     (Status: EXPERIMENTAL) (DOI: 10.17487/RFC8989) \n"
            + "\n"
            + "9003 Extended BGP Administrative Shutdown Communication. J. Snijders, J.\n"
            + "     Heitz, J. Scudder, A. Azimov. January 2021. (Format: HTML, TXT, PDF,\n"
            + "     XML) (Obsoletes RFC8203) (Updates RFC4486) (Status: PROPOSED\n"
            + "     STANDARD) (DOI: 10.17487/RFC9003) \n";

        let rfcs = vec![
            String::from("8989 Additional Criteria for Nominating Committee Eligibility. B.")
                + "     Carpenter, S. Farrell. February 2021. (Format: HTML, TXT, PDF, XML)"
                + "     (Status: EXPERIMENTAL) (DOI: 10.17487/RFC8989) ",
            String::from(
                "9003 Extended BGP Administrative Shutdown Communication. J. Snijders, J.",
            ) + "     Heitz, J. Scudder, A. Azimov. January 2021. (Format: HTML, TXT, PDF,"
                + "     XML) (Obsoletes RFC8203) (Updates RFC4486) (Status: PROPOSED"
                + "     STANDARD) (DOI: 10.17487/RFC9003) ",
        ];

        assert_eq!(rfcs, scrape(&data));
    }
}
