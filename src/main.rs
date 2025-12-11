mod fasta_parser;

use anyhow::{Context, Result};
use clap::Parser;
use indicatif;
use log::{info, warn};

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

    let pb = indicatif::ProgressBar::new(10);
    for i in 0..10 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        pb.println(format!("[+] finished #{i}"));
        pb.inc(1);
    }
    let fa = fasta_parser::parse_alignment(alignment_content);
    warn!("nothing implemented!");
    println!("{:?}", fa);
    Ok(())
}


#[test]
fn check_equality()
{
    assert_eq!(40+2, 42);
}
