use clap::{App, Arg};

/// Builds and returns the CLI parser.
pub fn build_cli() -> App<'static> {
    App::new("Text Combat Game")
        .version("0.1")
        .author("Your Name <you@example.com>")
        .about("A text-based combat game in Rust")
        .arg(
            Arg::with_name("debug")
                .short('d')
                .long("debug")
                .help("Enables debug mode"),
        )
}
