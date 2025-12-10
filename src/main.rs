mod fasta_parser;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The alignment filename
    alignment_fn: std::path::PathBuf,
    /// The properties filename
    property_fn: std::path::PathBuf
}

fn main() {
    let args = Cli::parse();
    
    println!("alignment_fn: {:?} property_fn: {:?}", args.alignment_fn, args.property_fn);

    let fa = fasta_parser::parse_alignment();
    println!("{:?}", fa);
}
