use regex::Regex;

fn validate_accession(input: &str) -> bool {
    let re = Regex::new(r"^GC[A|F]_[0-9]{9}\.[0-9]$").unwrap();
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

pub struct NCBIGenome {
    version: u32,
    refseq_or_genbank: String,
    accession_code: String,
    assembly_accession: String,
    assembly_name: String,
}

impl NCBIGenome {
    pub fn from_str(accession: &str, name: &str) -> NCBIGenome {
        if !validate_accession(accession) {
            panic!("Invalid format: {accession}")
        }
        let split_on_underscore: Vec<&str> = accession.split('_').collect();
        let accession_code = split_on_underscore[1].split(".").next().unwrap();
        let refseq_or_genbank = split_on_underscore[0];
        let version: u32 = accession
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .expect("Incorrect version number");
        NCBIGenome {
            version,
            refseq_or_genbank: refseq_or_genbank.to_string(),
            accession_code: accession_code.to_string(),
            assembly_accession: format!(
                "{}_{}.{}",
                refseq_or_genbank.to_string(),
                accession_code,
                &version.to_string()
            ),
            assembly_name: name.to_string(),
        }
    }
    fn get_ftp_folder_url(&self) -> String {
        let accession_code = &self.accession_code.clone();
        let (part1, part2, part3) = (
            &accession_code[0..3],
            &accession_code[3..6],
            &accession_code[6..9],
        );
        let ftp_folder = "https://ftp.ncbi.nlm.nih.gov/genomes/all";
        format!(
            "{}/{}/{}/{}/{}/{}_{}",
            ftp_folder,
            &self.refseq_or_genbank,
            part1,
            part2,
            part3,
            &self.assembly_accession,
            &self.assembly_name
        )
    }

    pub fn get_assembly_report_url(&self) -> String {
        let folder_url = &self.get_ftp_folder_url();
        let assembly_report = format!(
            "{}/{}_{}_assembly_report.txt",
            folder_url.to_string(),
            &self.assembly_accession,
            &self.assembly_name
        );
        assembly_report
    }
}
