use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use clap::Parser;
use csv::{QuoteStyle, WriterBuilder};
use evtx::EvtxParser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

    #[clap(short, long, value_parser, value_name = "Directory")]
    dir: Option<PathBuf>,

    #[clap(short, long, value_parser, value_name = "File")]
    file: Option<PathBuf>,
}


fn main()  -> std::io::Result<()> {
    let _args = Args::parse();

    // Change this to a path of your .evtx sample.
    let fp = PathBuf::from(format!("{}/samples/system.evtx", std::env::var("CARGO_MANIFEST_DIR").unwrap()));

    let mut parser = EvtxParser::from_path(fp).unwrap();
    let o = File::create("out.csv")?;
    let mut wtr = WriterBuilder::new().quote_style(QuoteStyle::Always).from_writer(BufWriter::new(o));
    for (i, record) in parser.records_json_value().enumerate() {
        let r = record.unwrap();
        let _ = wtr.write_record([r.event_record_id.to_string(), r.timestamp.to_string(), r.data.to_string()]);
    }
    Ok(())
}
