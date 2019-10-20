use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::collections::HashMap;

use std::error::Error;

struct Record {
    key: String,
    value: String,
}

const BLOCK_SIZE: usize  = 2880;
const RECORD_SIZE: usize = 80;


fn main() {
    println!("Welcome to the FITS processing tool, built in Rust!");

    let data_path = Path::new("data/HRSz0yd020fm_c2f.fits");
    let mut file = match File::open(data_path) {
        Err(e) => panic!("Couldn't open {}: {}", data_path.display(), e.description()),
        Ok(file) => file,
    };

    let mut fits_data = Vec::new();

    match file.read_to_end(&mut fits_data) {
        Err(e) => panic!("Couldn't read file {}: {}", data_path.display(), e.description()),
        Ok(bytes) => println!("Successfully read {} bytes from {}", bytes, data_path.display())  
    };
    
    let mut header_hdu: &[u8] = Default::default();

    for (i, _) in fits_data.iter().enumerate().step_by(BLOCK_SIZE) {
        //println!("Found block: {}", i);
        let header_part = stringify(&fits_data[i..(i+BLOCK_SIZE)]);

        if header_part.contains(" END ") { 
            println!("[Found header]");
            header_hdu = &fits_data[0..(i+BLOCK_SIZE)];
            break;
        }
    }

    //println!("Header content: {}", stringify(&header_hdu));

    let mut header_records = HashMap::new();

    for (i, _) in header_hdu.iter().enumerate().step_by(RECORD_SIZE) {
        let record = &header_hdu[i..(i+RECORD_SIZE)];
        let record_string = stringify(record);

        match parse_record(record_string) {
            Some(Record{key, value}) => {
                //println!("{}: {}", key, value);
                header_records.insert(key, value);
            }
            None => { /* println!("Didn't find a sensible record") */ }
        }
    }
    println!("Number of header records: {}", header_records.len());

}

fn parse_record(record: String) -> Option<Record> {
    if record.contains("=") {
        let records: Vec<&str> = record.splitn(2, "=").collect();
        let r = Record {key: records[0].trim().to_string(), value: records[1].trim().to_string()};
        return Some(r);
    }
    else {
        return None; 
    }
}

fn stringify(data: &[u8]) -> String {
    let s = String::from_utf8_lossy(data).into_owned();
    return s;
}
