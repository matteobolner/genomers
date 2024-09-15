use clap::Parser;
use download::download;

pub mod accession;
use crate::accession::NCBIGenome;
pub mod download;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Assembly Accession
    #[arg(required = true, short, long)]
    accession: String,
    /// Assembly name
    #[arg(required = true, short, long)]
    name: String,
    /// Download assembly report
    #[arg(group = "download", short, long)]
    report: bool,
    /// Download assembly report
    #[arg(group = "download", short, long)]
    genome: bool,
}

//-a GCF_000005845.2 -n ASM584v2

//TODO if no assembly name specified download the most recent (needs ftp?)

fn main() {
    let args = Args::parse();

    let genome_accession: NCBIGenome = NCBIGenome::from_str(&args.accession, &args.name);
    let sequence_url = genome_accession.get_assembly_sequence_url();
    let report_url = genome_accession.get_assembly_report_url();
    if args.report {
        eprintln!("Downloading assembly report: {}", report_url);
        download(report_url.to_string()).expect("FAILURE")
    }
    if args.genome {
        eprintln!("Downloading genome: {}", report_url);
        download(sequence_url.to_string()).expect("FAILURE")
    };
}
