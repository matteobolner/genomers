use clap::Parser;
use download::download;

pub mod accession;
use crate::accession::NCBIGenome;
pub mod download;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Assembly Accession
    #[arg(group = "assembly", short, long)]
    accession: String,
    /// Assembly name
    #[arg(group = "assembly", short, long)]
    name: String,
}

fn main() {
    let args = Args::parse();

    let genome_accession: NCBIGenome = NCBIGenome::from_str(&args.accession, &args.name);
    //println!("{}", genome_accession.get_ftp_folder_url());
    let url = genome_accession.get_assembly_sequence_url();
    //let url = genome_accession.get_assembly_report_url();
    download(url.to_string()).expect("FAILURE")
}
