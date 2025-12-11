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

fn main() {
    let args = Cli::parse();
    
    println!("alignment_fn: {:?} property_fn: {:?}", args.alignment_fn, args.property_fn);

    let alignment_result = std::fs::read_to_string(&args.alignment_fn);
    let alignment_content = match alignment_result {
        Ok(content) => { content },
        Err(error) => { panic!("Can't deal with {error}, just exit here") }
    };
            
    let _properties_content = std::fs::read_to_string(&args.property_fn).expect("could not read properties file");

    let fa = fasta_parser::parse_alignment(alignment_content);
    println!("{:?}", fa);
}
