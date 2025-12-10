mod fasta_parser;

fn main() {
    
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    let fa = fasta_parser::parse_alignment();
    println!("Hello, world!  pattern: {:?} path: {:?}", pattern, path);
    println!("{:?}", fa);
}
