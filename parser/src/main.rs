use clap::Parser;
use std::fs;
use templafy::binding::Binding;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    /// Target CSS file
    input_file: String,
    /// Destination file for the output
    #[clap(short, long, group = "output_file")]
    output_file: Option<String>,
}

fn main() -> Result<(), std::io::Error> {
    let cli_args = CliArgs::parse();
    let source = fs::read_to_string(&cli_args.input_file)?;

    let absolute_path = fs::canonicalize(cli_args.input_file)?;
    let filename: String = pathdiff::diff_paths(absolute_path, std::env::current_dir()?)
        .unwrap()
        .to_str()
        .unwrap()
        .into();

    let binding = Binding::parse(filename, "asd");

    Ok(())
}
