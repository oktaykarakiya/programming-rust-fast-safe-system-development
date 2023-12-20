use regex::Regex;
use std::env;
use std::fs;
use text_colorizer::*;

#[derive(Debug)] // This attriute tells the compiler to generate some extra code that allows us to format the Arguments struct with {?} in println!
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn print_usage() {
    eprintln!(
        "{} - change occurrence of one strin into another",
        "quickreplace".yellow().bold()
    );
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expected 4, got {}.",
            "Error: ".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}

fn main() {
    // You could check the differences using this command: diff Cargo.toml Copy.toml
    let args = parse_args();

    println!("{:?}", args);

    let data = match fs::read_to_string(&args.filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "{} failed to read from file '{}' {:?}",
                "Error".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    };

    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{} failed to replace text {:?}", "Error".red().bold(), e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &replaced_data) {
        Ok(_) => {}
        Err(e) => {
            eprint!(
                "{} failed to write to dile '{}': {:?}",
                "Error".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    };

    print_usage();
}
