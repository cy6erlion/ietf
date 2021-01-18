use clap::{App, Arg, SubCommand};
use cursive::align::HAlign;
use cursive::event::EventResult;
use cursive::traits::With;
use cursive::traits::*;
use cursive::views::{Dialog, OnEventView, SelectView, TextView};
use cursive::Cursive;

mod fetch;
mod storage;

use std::fs::File;
use std::io::{BufReader, Read};

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("ietf")
        .version("0.2.1")
        .about("CLI for reading IETF RFCs.")
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
    let mut siv = cursive::default();
    siv.set_theme(cursive::theme::Theme::default().with(|theme| {
        use cursive::theme::{BaseColor::*, Color::*, PaletteColor::*};
        theme.palette[Background] = TerminalDefault;
        theme.palette[Primary] = Dark(Black);
        theme.palette[Secondary] = Rgb(255, 12, 42);
    }));

    // Read RFC by rfcnumber
    if let Some(n) = matches.value_of("Number") {
        let rfc_number = n.parse::<u32>().unwrap();

        // check if RFC is downloaded
        if !storage.is_rfc_downloaded(rfc_number).unwrap() {
            println!("Fetching RFC...");

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

        siv.add_layer(TextView::new(rfc_data).with_name("text").scrollable());
        siv.run();
        return Ok(());
    }

    // Removes RFC by serial number
    if let Some(n) = matches.value_of("Remove") {
        storage.remove(
            n.parse::<u32>()
                .expect("RFC Serial Number should be a numeric value!"),
        );
        return Ok(());
    }

    // Update RFC index
    if let Some(_matches) = matches.subcommand_matches("update") {
        storage.update_index();
        return Ok(());
    }

    // Remove the ietf directory
    if let Some(_matches) = matches.subcommand_matches("clean") {
        storage.clean();
        return Ok(());
    }

    // ---------- Display RFC list view ------------
    let mut index_data = String::new();
    let index_file = File::open(&storage.index_file_path).expect("Unable to open file");
    let mut buffer_reader = BufReader::new(index_file);
    let mut _read_more_dots = "";

    // Let's put the callback in a separate function to keep it clean,
    // but it's not required.
    let show_next_window = move |siv: &mut Cursive, rfc_title: &str| {
        let rfc_title: Vec<&str> = rfc_title.split(' ').collect();

        let rfc_number = rfc_title[0]
            .parse::<u32>()
            .expect("Could not parse RFC number");

        // check if RFC is downloaded
        if !storage.is_rfc_downloaded(rfc_number).unwrap() {
            println!("Fetching RFC...");
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

        siv.add_layer(TextView::new(rfc_data).with_name("text").scrollable());
    };

    buffer_reader
        .read_to_string(&mut index_data)
        .expect("Unable to read INDEX");

    let lines = index_data.lines();

    let mut select = SelectView::new()
        // Center the text horizontally
        .h_align(HAlign::Center)
        // Use keyboard to jump to the pressed letters
        .autojump();

    select.add_all_str(lines);

    // Sets the callback for when "Enter" is pressed.
    select.set_on_submit(show_next_window);

    // Let's override the `p` and `n` keys for navigation
    let select = OnEventView::new(select)
        .on_pre_event_inner('p', |s, _| {
            let cb = s.select_up(1);
            Some(EventResult::Consumed(Some(cb)))
        })
        .on_pre_event_inner('n', |s, _| {
            let cb = s.select_down(1);
            Some(EventResult::Consumed(Some(cb)))
        });

    siv.add_layer(Dialog::around(select.scrollable().fixed_size((30, 20))).title("IETF RFC INDEX"));
    siv.run();
    Ok(())
}
