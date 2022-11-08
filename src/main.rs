use std::path::Path;
mod hit;
mod search;
use clap::Parser;

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let target = args.target;
    let results = search::search_dir(Path::new("./"), &target, args.limit)?;
    results.iter().for_each(|x| println!("{}", x));

    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// String to fuzzy search for
    target: String,
    /// Maximum acceptable character difference for the search
    #[arg(short, long, default_value_t = 2)]
    limit: usize,
}
