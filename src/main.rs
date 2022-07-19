use clap::Parser;
use csv::{QuoteStyle, WriterBuilder};
use evtx::EvtxParser;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(long_about = None)]
struct Args {
    /// target evtx dir path.
    #[clap(short, long, value_parser)]
    dir: PathBuf,

}

fn main() {
    let args: Args  = Args::parse();
    for entry in fs::read_dir(args.dir).expect("failed to open dir") {
        let path = entry.unwrap().path();
        write_to_csv(&path);
    }
}

fn write_to_csv(path: &PathBuf) {
    let name = path.file_name().unwrap();
    let mut out = PathBuf::from("./target").join(PathBuf::from(name));
    out.set_extension("csv");
    let file = File::create(out).expect("failed to create output csv file.");
    let mut parser = EvtxParser::from_path(path).unwrap();
    let mut wtr = WriterBuilder::new()
        .quote_style(QuoteStyle::Always)
        .from_writer(BufWriter::new(file));
    for record in parser.records_json_value() {
        match record {
            Ok(r) => {
                let _ = wtr.write_record([r.event_record_id.to_string(), r.timestamp.to_string(), r.data.to_string(), ]);
            }
            Err(e) => eprintln!("{:?}", e),
        }
    }
    println!("converting {:?} to csv done.", path);
}


#[test]
fn dir_func_test() {
    let p = PathBuf::from("./samples");
    for entry in p.read_dir().expect("failed to open dir"){
        if let Ok(e) = entry {
            println!("{:?}", e.path())
        }
    }
    assert_eq!(1, 1)
}