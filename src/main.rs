use clap::clap_app;
use std::process;

use minigrep::Config;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "0.2")
        (author: "Mohammed G. <gaberm@gmail.com>")
        (about: "Does awesome things")
        // (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
        (@arg PATTERN: +required "Sets the pattern to use")
        (@arg INPUT: +required "Sets the input file to search")
        (@arg case_sensitive: -c ... "Sets the case sensitive flag to true")
        (@subcommand count =>
            (about: "prints the count of matches")
            (version: "0.1")
            // (@arg verbose: -v --verbose "Print test information verbosely")
        )
    )
    .get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let arg_pattern = matches.value_of("PATTERN").unwrap();
    let arg_input = matches.value_of("INPUT").unwrap();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    // let config = matches.value_of("CONFIG").unwrap_or("default.conf");
    // println!("Value for config: {}", config);

    // Vary the output based on how many times the user used the "debug" flag
    // (i.e. 'myapp -d -d -d' or 'myapp -ddd' vs 'myapp -d'

    let mut arg_case = false;
    match matches.occurrences_of("case_sensitive") {
        0 => (), //println!("Defaulting to case_insensitive search"),
        1 => {
            arg_case = true;
        }
        _ => println!("Don't be crazy"),
    }

    let mut args: Vec<String> = Vec::new();
    args.push(arg_pattern.to_string());
    args.push(arg_input.to_string());
    args.push(arg_case.to_string());

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
