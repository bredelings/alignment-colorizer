mod fasta_parser;

struct Cli {
    alignment_fn: std::path::PathBuf,
    property_fn: std::path::PathBuf
}

fn main() {
    
    let alignment_fn = std::env::args().nth(1).expect("no pattern given");
    let property_fn = std::env::args().nth(2).expect("no path given");

    let args = Cli {
        alignment_fn: std::path::PathBuf::from(alignment_fn),
        property_fn: std::path::PathBuf::from(property_fn)
    };
    
    println!("args: alignment_fn = {:?} property_fn = {:?}", args.alignment_fn, args.property_fn);

    let fa = fasta_parser::parse_alignment();
    println!("{:?}", fa);
}
