mod fasta_parser;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    
    println!("alignment_fn: {:?} property_fn: {:?}", args.alignment_fn, args.property_fn);

    let alignment_content = std::fs::read_to_string(&args.alignment_fn)?;
    let _properties_content = std::fs::read_to_string(&args.property_fn)?;

    let fa = fasta_parser::parse_alignment(alignment_content);
    println!("{:?}", fa);
    Ok(())
}
