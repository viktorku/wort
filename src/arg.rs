use crate::core::material::DiffuseMethod;
use crate::sinks::Sink;
use clap::{App, Arg};
use std::str::FromStr;
use strum::VariantNames;

pub struct Args {
    pub output: Sink,
    pub filename: Option<String>,
    pub diffuse_method: DiffuseMethod,
    pub verbose: bool,
}

pub fn parse_arguments() -> Args {
    let matches = App::new("wort")
        .version("0.1")
        .author("Viktor K. <viktor@kunovski.com>")
        .about("a week(end) of ray tracing")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT")
                .help("Output - file (PPM) or window")
                .takes_value(true)
                .possible_values(&Sink::VARIANTS),
        )
        .arg(
            Arg::with_name("filename")
                .short("n")
                .long("filename")
                .value_name("FILE")
                .help("Filename - defaults to `image`")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("diffuse")
                .short("d")
                .long("diffuse")
                .value_name("DIFFUSE")
                .help("Diffusing method")
                .takes_value(true)
                .possible_values(&DiffuseMethod::VARIANTS),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .takes_value(false)
                .help("Verbosity, prints remaining scanline"),
        )
        .get_matches();

    let diffuse_str = matches
        .value_of("diffuse")
        .unwrap_or_else(|| DiffuseMethod::Lambert.into());
    let diffuse_method = DiffuseMethod::from_str(diffuse_str).unwrap();

    let output_str = matches
        .value_of("output")
        .unwrap_or_else(|| Sink::Window.into());
    let output = Sink::from_str(output_str).unwrap();

    let filename = if let Some(filename) = matches.value_of("filename") {
        Some(format!("{}_{}.ppm", filename, diffuse_str))
    } else if output != Sink::File {
        None
    } else {
        Some("image.ppm".into())
    };

    let verbose = matches.is_present("verbose");

    Args {
        output,
        filename,
        diffuse_method,
        verbose,
    }
}
