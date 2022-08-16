use chrono::{DateTime, TimeZone, Utc};
use clap::{AppSettings, Parser};
use csv::{QuoteStyle, WriterBuilder};
use evtx::{EvtxParser, SerializedEvtxRecord};
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(long_about = None)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
struct Args {
    /// target evtx directory path.
    #[clap(short, long, value_parser)]
    evtx_dir: PathBuf,

    /// CSV output directory path.
    #[clap(short, long, value_parser)]
    output_dir: PathBuf,

    /// First time of the event logs to parse (ex: "2022-04-01 00:00:00")
    #[clap(short, long, value_parser)]
    first_time: Option<String>,

    /// Last time of the event logs to parse (ex: "2022-04-30 00:00:00")
    #[clap(short, long, value_parser)]
    last_time: Option<String>,

    /// Search keyword to grep log.
    #[clap(short, long, value_parser)]
    search_keyword: Option<String>,
}

fn main() {
    let args: Args = Args::parse();
    for entry in fs::read_dir(&args.evtx_dir).expect("failed to open dir") {
        let path = entry.unwrap().path();
        let mut parser = EvtxParser::from_path(&path).unwrap();
        let max_time = Utc.datetime_from_str("2030-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let min_time = Utc.datetime_from_str("2000-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let first_time = match &args.first_time {
            None => min_time,
            Some(s) => Utc.datetime_from_str(s, "%Y-%m-%d %H:%M:%S").unwrap_or(min_time),
        };
        let last_time = match &args.last_time {
            None => max_time,
            Some(s) => Utc.datetime_from_str(s, "%Y-%m-%d %H:%M:%S").unwrap_or(max_time),
        };

        let keyword = match &args.search_keyword {
            None => "",
            Some(s) => s
        };

        let filtered_records = filter_records(&mut parser, &first_time, &last_time, keyword);
        write_to_csv(&path, &args.output_dir, &filtered_records);
    }
}

fn filter_records(parser: &mut EvtxParser<File>, first: &DateTime<Utc>, last: &DateTime<Utc>, keyword: &str) -> Vec<SerializedEvtxRecord<Value>> {
    let records: Vec<SerializedEvtxRecord<Value>> = parser
        .records_json_value()
        .into_iter()
        .filter_map(|v| v.ok())
        .filter(|v| *first <= v.timestamp && v.timestamp <= *last && v.data.to_string().contains(keyword))
        .collect();
    records
}

fn write_to_csv(path: &PathBuf, outdir: &PathBuf, records: &Vec<SerializedEvtxRecord<Value>>) {
    let name = path.file_name().unwrap();
    if !outdir.exists() {
        fs::create_dir(outdir).expect("failed to create output directory.");
    }
    let mut out = outdir.join(PathBuf::from(name));
    out.set_extension("csv");
    let file = File::create(out).expect("failed to create output csv file.");
    let mut wtr = WriterBuilder::new()
        .quote_style(QuoteStyle::Always)
        .from_writer(BufWriter::new(file));
    let _ = wtr.write_record(["event_record_id", "timestamp", "data"]);
    for r in records {
        let _ = wtr.write_record([
            r.event_record_id.to_string(),
            r.timestamp.to_string(),
            r.data.to_string(),
        ]);
    }
    println!("converting {:?} to csv done.", path);
}

#[test]
fn dir_func_test() {
    let p = PathBuf::from("./samples");
    for entry in p.read_dir().expect("failed to open dir") {
        if let Ok(e) = entry {
            println!("{:?}", e.path())
        }
    }
    assert_eq!(1, 1)
}
