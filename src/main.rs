use clap::Parser;
use regex::Regex;

fn validate_accession(input: &str) -> bool {
    let re = Regex::new(r"^[A-Z]{3}_[0-9]{9}\.[0-9]$").unwrap();
    re.is_match(input)
}

/*
enum RefSeqOrGenBank {
    RefSeq,
    GenBank,
}

impl RefSeqOrGenBank {
    // Function to convert &str to Option<ElementType>
    fn from_str(input: &str) -> Option<RefSeqOrGenBank> {
        match input {
            "GCA" => Some(RefSeqOrGenBank::GenBank),
            "GCF" => Some(RefSeqOrGenBank::RefSeq),
            _ => panic!("Code is not RefSeq (GCF) or GenBank (GCA): {input}"),
        }
    }
}
*/

#[derive(Debug)]
struct GenomeAccession {
    version: u32,
    refseq_or_genbank: String,
    accession_code: String,
}

impl GenomeAccession {
    fn from_str(input: &str) -> Result<GenomeAccession, String> {
        if !validate_accession(input) {
            panic!("Invalid format: {input}")
        }
        //return Err("Invalid format: {input}".to_string());
        let split_on_underscore: Vec<&str> = input.split('_').collect();
        let accession_code = split_on_underscore[1].split(".").next().unwrap();
        let refseq_or_genbank = split_on_underscore[0];
        let version: u32 = input
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .expect("Incorrect version number");
        println!("{}", version);
        Ok(GenomeAccession {
            version: version,
            refseq_or_genbank: refseq_or_genbank.to_string(),
            accession_code: accession_code.to_string(),
        })
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    accession: String,
}

fn main() {
    let args = Args::parse();

    let genome_accession: Result<GenomeAccession, String> =
        GenomeAccession::from_str(&args.accession);
    println!("Accession: {:#?}", genome_accession)
}
