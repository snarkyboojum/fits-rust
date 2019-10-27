use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

enum FitsSection {
    Header,
    Data,
    Extension,
}

struct Record {
    key: String,
    value: String,
}

const BLOCK_SIZE: usize = 2880;
const RECORD_SIZE: usize = 80;

fn main() {
    println!("Welcome to the FITS processing tool, built in Rust!");

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let file_name = if args.len() > 1 {
        &args[1]
    } else {
        "data/HRSz0yd020fm_c2f.fits"
    };

    let data_path = Path::new(file_name);
    let mut file = File::open(data_path).expect("Couldn't open data file");

    let mut fits_data = Vec::new();
    let bytes = file
        .read_to_end(&mut fits_data)
        .expect("Couldn't read data file");

    println!(
        "Successfully read {} bytes from {}",
        bytes,
        data_path.display()
    );

    use crate::FitsSection::*;

    let mut state = Header;
    let mut block_index = 0;

    let _chunks: Vec<()> = fits_data
        .chunks(BLOCK_SIZE)
        .enumerate()
        .map(|(i, chunk)| {
            println!("Processing chunk, {}", i);

            match state {
                Header => {
                    if stringify(chunk).contains(" END ") {
                        parse_header(&fits_data[block_index..(block_index + (i + 1) * BLOCK_SIZE)]);
                        state = Data;
                        block_index = i;
                    }
                }
                Data => {
                    let data_size =
                        parse_data(&fits_data[block_index..(block_index + (i + 1) * BLOCK_SIZE)]);
                    state = Extension;
                    block_index = i;
                }
                Extension => {
                    if stringify(chunk).contains("XTENSION") {
                        println!("[Found extension start]");
                        state = Extension;
                    }
                    // @todo: it seems possible to hit this block and try and parse an extension without actually finding an "XTENSION" keyword
                    // @fix: fix this - and check
                    if stringify(chunk).contains(" END ") {
                        parse_extension(
                            &fits_data[block_index..(block_index + (i + 1) * BLOCK_SIZE)],
                        );
                        state = Header;
                        block_index = i;
                    }
                }
            };
        })
        .collect();
}

fn parse_header(data: &[u8]) {
    let mut header_records = HashMap::new();

    println!("[Found header end]");

    for (i, _) in data.iter().enumerate().step_by(RECORD_SIZE) {
        let record = &data[i..(i + RECORD_SIZE)];
        let record_string = stringify(record);

        if let Some(Record { key, value }) = parse_record(record_string) {
            //println!("{}: {}", key, value);
            header_records.insert(key, value);
        }
    }
}

fn parse_data(data: &[u8]) -> usize {
    0
}

fn parse_extension(data: &[u8]) {
    println!("[Found extension end]");
}

fn parse_record(record: String) -> Option<Record> {
    if record.contains('=') {
        let records: Vec<&str> = record.splitn(2, '=').collect();
        let r = Record {
            key: records[0].trim().to_string(),
            value: records[1].trim().to_string(),
        };
        Some(r)
    } else {
        None
    }
}

fn stringify(data: &[u8]) -> String {
    String::from_utf8_lossy(data).into_owned()
}
