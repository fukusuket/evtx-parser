use std::path::PathBuf;
use clap::Parser;
use evtx::EvtxParser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

    #[clap(short, long, value_parser, value_name = "Directory")]
    dir: Option<PathBuf>,

    #[clap(short, long, value_parser, value_name = "File")]
    file: Option<PathBuf>,
}


fn main() {
    let _args = Args::parse();

    // Change this to a path of your .evtx sample.
    let fp = PathBuf::from(format!("{}/samples/system.evtx", std::env::var("CARGO_MANIFEST_DIR").unwrap()));

    let mut parser = EvtxParser::from_path(fp).unwrap();
    for record in parser.records_json_value() {
        match record {
            Ok(r) => println!("{},{},{}", r.event_record_id, r.timestamp, r.data),
            Err(e) => eprintln!("{}", e),
        }
    }
}
