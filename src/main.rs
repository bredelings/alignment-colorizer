mod fasta_parser;
use anyhow::{Context, Result};
use clap::Parser;
use log::info;

#[derive(Parser)]
struct Cli {
    /// The property name
    property: String,
    /// The properties filename
    property_fn: std::path::PathBuf,
    /// The alignment filename
    alignment_fn: std::path::PathBuf
}


fn main() -> Result<()> {

    env_logger::init();
    let args = Cli::parse();

    println!("alignment_fn: {:?} property_fn: {:?}", args.alignment_fn, args.property_fn);

    let alignment_content = std::fs::read_to_string(&args.alignment_fn)
        .with_context(|| format!("could not read file `{:?}`",args.alignment_fn))?;
    info!("Read alignment");
    let _properties_content = std::fs::read_to_string(&args.property_fn)
        .with_context(|| format!("could not read file `{:?}`",args.property_fn))?;
    info!("Read properties file");
    let _property = args.property;

    let recs = fasta_parser::parse_alignment(alignment_content);
    info!("Got {:?} records", recs.len());

    for rec in &recs {
        println!("{}: {}", rec.name, rec.sequence.len());
    }
    println!("Read {} records",recs.len());
    
    Ok(())
}


#[test]
fn check_equality()
{
    assert_eq!(40+2, 42);
}
