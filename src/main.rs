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

    let alignment_content = std::fs::read_to_string(&args.alignment_fn).expect("could not read alignment file");
    let _properties_content = std::fs::read_to_string(&args.property_fn).expect("could not read properties file");

    let fa = fasta_parser::parse_alignment(alignment_content);
    println!("{:?}", fa);
}
