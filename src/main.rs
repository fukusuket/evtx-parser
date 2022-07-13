use std::path::PathBuf;
use evtx::EvtxParser;


fn main() {
    // Change this to a path of your .evtx sample.
    let fp = PathBuf::from(format!("{}/samples/system.evtx", std::env::var("CARGO_MANIFEST_DIR").unwrap()));

    let mut parser = EvtxParser::from_path(fp).unwrap();
    for record in parser.records_json_value() {
        match record {
            Ok(r) => println!("Record {}:{}\n{}", r.event_record_id, r.timestamp, r.data),
            Err(e) => eprintln!("{}", e),
        }
    }
}
