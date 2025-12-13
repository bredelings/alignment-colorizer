mod fasta_parser;
use anyhow::{anyhow, Context, Result};
use clap::Parser;
use log::info;
use serde_json;

#[derive(Parser)]
struct Cli {
    /// The property name
    property: String,
    /// The properties filename
    property_fn: std::path::PathBuf,
    /// The alignment filename
    alignment_fn: std::path::PathBuf
}

fn get_alignment(filename: &std::path::Path) -> Result<Vec<fasta_parser::FastaRecord>, anyhow::Error>
{
    let alignment_content = std::fs::read_to_string(filename)
        .with_context(|| format!("could not read file `{:?}`",filename))?;
    info!("Read alignment");
    let sequences = fasta_parser::parse_alignment(alignment_content);
    info!("Got {:?} sequences.", sequences.len());
    
    Ok(sequences)
}



fn get_property_values(filename: &std::path::Path, property: &str) -> Result<serde_json::Value>
{
    let properties_content = std::fs::read_to_string(filename)
        .with_context(|| format!("could not read file `{:?}`",filename))?;
    info!("Read properties file");
    let properties_json: serde_json::Value = serde_json::from_str(&properties_content)
        .with_context(|| format!("failed to parse JSON from file `{}`", filename.display()))?;

    properties_json.get(property)
        .ok_or_else(|| anyhow!("Properties JSON has no property '{}'",property))
        .map(|json| json.clone())
}

fn main() -> Result<()>
{
    env_logger::init();

    let args = Cli::parse();

    let sequences = get_alignment(&args.alignment_fn)?;

    let _property_values = get_property_values(&args.property_fn, &args.property)?;

    for sequence in &sequences
    {
        println!("{}: {}", sequence.name, sequence.sequence);
    }
    println!("Read {} records",sequences.len());
    
    Ok(())
}


#[test]
fn check_equality()
{
    assert_eq!(40+2, 42);
}
