/*use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
    .version("0.1.0")
    .author(" Asuka Ohira <rozeo1505@gmail.com>")
    .about("Rust echo")
    .arg(
        Arg::with_name("text")
        .value_name("TEXT")
        .help("Input text")
        .required(true)
        .min_values(1),
    )
    .arg(
        Arg::with_name("omit_newline")
        .short("n")
        .help("Do not print newline")
        .takes_value(false),
    )
    .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    print!("{}{}", text.join(" "), if omit_newline { "" } else {"\n"});
}
*/

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `echo`
struct Args {
    /// Input text
    #[arg(required(true))]
    text: Vec<String>,

    /// Do not print newline
    #[arg(short('n'))]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();
    print!(
        "{}{}",
        args.text.join(" "),
        if args.omit_newline { "" } else { "\n" }
    );
}