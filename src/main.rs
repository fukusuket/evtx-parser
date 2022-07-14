use clap::Parser;
use csv::{QuoteStyle, WriterBuilder};
use evtx::EvtxParser;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    dir: Option<PathBuf>,

    #[clap(short, long, value_parser)]
    file: Option<PathBuf>,
}

fn main() -> std::io::Result<()> {
    let _args = Args::parse();

    let fp = PathBuf::from(format!(
        "{}/samples/",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    ));

    for entry in fs::read_dir(fp)? {
        let entry = entry?;
        let path = entry.path();
        let filename = String::from(entry.file_name().to_string_lossy());
        let mut parser = EvtxParser::from_path(path).unwrap();
        let mut wtr = WriterBuilder::new()
            .quote_style(QuoteStyle::Always)
            .from_writer(BufWriter::new(File::create(format!(
                "./target/{}.csv",
                filename
            ))?));
        for record in parser.records_json_value() {
            match record {
                Ok(r) => {
                    let _ = wtr.write_record([
                        r.event_record_id.to_string(),
                        r.timestamp.to_string(),
                        r.data.to_string(),
                    ]);
                }
                Err(e) => eprintln!("{}", e),
            }
        }
    }
    Ok(())
}
