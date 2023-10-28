use clap::{Arg, App};

pub fn parse_args() -> (u8, u8, String) {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("width")
            .help("Field width")
            .required(true)
            .index(1))
        .arg(Arg::with_name("height")
            .help("Field height")
            .required(true)
            .index(2))
        .arg(Arg::with_name("tetrominoes")
            .help("String of tetrominoes")
            .required(true)
            .index(3))
        .after_help(format!("Usage example: \"talos 7 4 I2T2S1Z1L1\"\n\
                             Repository: {}", env!("CARGO_PKG_REPOSITORY")).as_str())
        .get_matches();
    (
        matches.value_of("width").unwrap().parse().unwrap(),
        matches.value_of("height").unwrap().parse().unwrap(),
        matches.value_of("tetrominoes").unwrap().to_string()
    )
}