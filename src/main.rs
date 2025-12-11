mod fasta_parser;

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The alignment filename
    alignment_fn: std::path::PathBuf,
    /// The properties filename
    property_fn: std::path::PathBuf,
    /// The property name
    property: String
}

fn main() -> Result<()> {
    let args = Cli::parse();

    println!("alignment_fn: {:?} property_fn: {:?}", args.alignment_fn, args.property_fn);

    let alignment_content = std::fs::read_to_string(&args.alignment_fn)
        .with_context(|| format!("could not read file `{:?}`",args.alignment_fn))?;
    let _properties_content = std::fs::read_to_string(&args.property_fn)
        .with_context(|| format!("could not read file `{:?}`",args.property_fn))?;
    let _property = args.property;

    let fa = fasta_parser::parse_alignment(alignment_content);
    println!("{:?}", fa);
    Ok(())
}
