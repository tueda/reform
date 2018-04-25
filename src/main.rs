extern crate clap;
extern crate env_logger;
extern crate reform;

#[cfg(feature = "profile")]
extern crate cpuprofiler;

use clap::{App, Arg};

#[cfg(feature = "profile")]
use cpuprofiler::PROFILER;

fn main() {
    env_logger::init();
    #[cfg(feature = "profile")]
    PROFILER.lock().unwrap().state(); // Ensure linking to gperftools

    let matches = App::new("reFORM")
        .version("0.1.0")
        .author("Ben Ruijl <benruyl@gmail.com>")
        .about("A symbolic manipulation toolkit")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("log")
                .short("l")
                .long("log")
                .help("Create a log file with the output"),
        )
        .arg(
            Arg::with_name("workers")
                .short("w")
                .long("workers")
                .help("Number of workers (threads)")
                .default_value("1")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .default_value("test.frm")
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    let mut program = reform::parser::parse_file(matches.value_of("INPUT").unwrap());
    program.do_program(
        matches.is_present("log"),
        matches.occurrences_of("v"),
        matches.value_of("workers").unwrap().parse().unwrap(),
    );
}
