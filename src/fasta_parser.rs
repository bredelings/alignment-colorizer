use indicatif;

pub struct FastaRecord {
    pub name: String,
    pub sequence: String
}

pub fn parse_alignment(content: String) -> Vec<FastaRecord> {
    let lines = content.lines();
    let n_lines = lines.count();
    let pb = indicatif::ProgressBar::new(n_lines.try_into().unwrap());

    let mut i=0;
    for _line in content.lines() {
        std::thread::sleep(std::time::Duration::from_millis(10));
        pb.println(format!("[+] finished #{i}"));
        pb.inc(1);
        i += 1;
    }

    let rec1 = FastaRecord {name: "Homo".to_string(), sequence: "ATGC".to_string()};

    Vec::from([rec1])
}

