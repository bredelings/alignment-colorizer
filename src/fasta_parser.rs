use indicatif;

pub struct FastaRecord {
    pub name: String,
    pub sequence: String
}

pub fn parse_alignment(content: String) -> Vec<FastaRecord> {
    let lines = content.lines();
    let n_lines = lines.count();
    let pb = indicatif::ProgressBar::new(n_lines.try_into().unwrap());


    let mut records: Vec<FastaRecord> = Vec::new();
    let mut cur_rec: Option<FastaRecord> = None;

    for line in content.lines()
    {
        if let Some(name) = line.strip_prefix('>')
        {
            if let Some(rec) = cur_rec.take()
            {
                records.push(rec);
            }

            cur_rec = Some(FastaRecord{name: name.to_string(), sequence: String::new()});
        }
        else
        {
            if let Some(rec) = cur_rec.as_mut()
            {
                let chars: String = line.chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();
                rec.sequence.push_str(&chars);
            }
        }
        pb.inc(1);
    }

    records
}

