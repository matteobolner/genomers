use regex::Regex;
use url::Url;

fn validate_url(url: &str) -> url::Url {
    match Url::parse(url) {
        Ok(url) => url,
        Err(e) => {
            panic!("Could not parse '{}'. {}.", url, e);
        }
    }
}

fn validate_accession(input: &str) -> bool {
    let re = Regex::new(r"^GC[A|F]_[0-9]{9}\.[0-9]$").unwrap();
    re.is_match(input)
}

pub struct NCBIGenome {
    //version: u32,
    refseq_or_genbank: String,
    accession_code: String,
    assembly_accession: String,
    assembly_name: String,
}

impl NCBIGenome {
    /// Initiate the struct from assembly accession and assembly name
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
            //version,
            refseq_or_genbank: refseq_or_genbank.to_string(),
            accession_code: accession_code.to_string(),
            assembly_accession: format!(
                "{}_{}.{}",
                refseq_or_genbank,
                accession_code,
                &version.to_string()
            ),
            assembly_name: name.to_string(),
        }
    }

    /// Obtain url for the assembly folder
    fn get_ftp_folder_url(&self) -> url::Url {
        let accession_code = &self.accession_code.clone();
        let (part1, part2, part3) = (
            &accession_code[0..3],
            &accession_code[3..6],
            &accession_code[6..9],
        );
        let ftp_folder = "https://ftp.ncbi.nlm.nih.gov/genomes/all";
        let ftp_folder = format!(
            "{}/{}/{}/{}/{}/{}_{}",
            ftp_folder,
            &self.refseq_or_genbank,
            part1,
            part2,
            part3,
            &self.assembly_accession,
            &self.assembly_name
        );
        validate_url(&ftp_folder)
    }

    /// Obtain url for the assembly report
    pub fn get_assembly_report_url(&self) -> url::Url {
        let folder_url = &self.get_ftp_folder_url();
        let assembly_report = format!(
            "{}/{}_{}_assembly_report.txt",
            folder_url, &self.assembly_accession, &self.assembly_name
        );
        validate_url(&assembly_report)
    }

    /// Obtain url for the assembly sequence
    pub fn get_assembly_sequence_url(&self) -> url::Url {
        let folder_url = &self.get_ftp_folder_url();
        let assembly_sequence = format!(
            "{}/{}_{}_genomic.fna.gz",
            folder_url, &self.assembly_accession, &self.assembly_name
        );
        validate_url(&assembly_sequence)
    }
}
