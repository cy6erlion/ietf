use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("rfc")
        .version("0.1.0")
        .about("A program to read RFCs in the terminal.")
        .arg(
            Arg::with_name("Number")
                .short("n")
                .long("number")
                .value_name("serial")
                .help("RFC Serial Number")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name("update").about("Update RFC Index"))
        .get_matches();

    // Read RFC by serial number
    if let Some(n) = matches.value_of("Number") {
        rfc::read_rfc(
            n.parse::<u32>()
                .expect("RFC Serial Number should be a numeric value!"),
        );
        return;
    }

    // Update RFC index
    if let Some(matches) = matches.subcommand_matches("update") {
        rfc::update();
        return;
    }

    // Display RFC list view
    rfc::list_view();
}
