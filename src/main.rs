use clap::{App, Arg, SubCommand};

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

    // Read RFC by serial number
    if let Some(n) = matches.value_of("Number") {
        ietf::read_rfc(
            n.parse::<u32>()
                .expect("RFC Serial Number should be a numeric value!"),
        );
        return;
    }

    // Removes RFC by serial number
    if let Some(n) = matches.value_of("Remove") {
        ietf::storage::remove(
            n.parse::<u32>()
                .expect("RFC Serial Number should be a numeric value!"),
        );
        return;
    }

    // Update RFC index
    if let Some(_matches) = matches.subcommand_matches("update") {
        ietf::update();
        return;
    }

    // Remove the ietf directory
    if let Some(_matches) = matches.subcommand_matches("clean") {
        ietf::storage::clean();
        return;
    }

    // Display RFC list view
    ietf::list_view();
}
