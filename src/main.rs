extern crate byteorder;

use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

enum Section {
    Header,
    Data,
    Extension,
}

struct Fits<Header, Data, Extension> {
    header: Header,
    data: Data,
    extension: Vec<Extension>,
}

#[derive(Default)]
struct Header {
    data: HashMap<String, String>,
}

#[derive(Default)]
struct Data<'t> {
    data: &'t [u8],
}

struct Extension {
    data: String,
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

    let mut state = Section::Header;
    let mut block_index = 0;

    let mut fits: Fits<Header, Data, Extension> = Fits {
        header: Default::default(),
        data: Default::default(),
        extension: Default::default(),
    };

    for (current_block, chunk) in fits_data.chunks(BLOCK_SIZE).enumerate() {
        println!("Processing chunk, {}", current_block);

        match state {
            Section::Header => {
                if String::from_utf8_lossy(chunk).contains(" END ") {
                    fits.header = parse_header(&fits_data, block_index, current_block);
                    state = Section::Data;
                    block_index = current_block + 1;
                }
            }
            Section::Data => {
                // @todo: stop hardcoding dim and lookup in header!
                let dimensionality = 2;
                if dimensionality == 2 {
                    let (x, y) = (2000, 4);
                    fits.data = parse_data(&fits_data, block_index, (x, y));
                }
                state = Section::Extension;
                block_index = current_block + (fits.data.data.len() / BLOCK_SIZE) + 1;
            }
            Section::Extension => {
                if String::from_utf8_lossy(chunk).contains("XTENSION") {
                    println!("[Found extension start]");
                    state = Section::Extension;
                }
                // @todo: it seems possible to hit this block and try and parse an
                //        extension without actually finding an "XTENSION" keyword
                // @fix: fix this - and check
                if String::from_utf8_lossy(chunk).contains(" END ") {
                    fits.extension
                        .push(parse_extension(&fits_data, block_index, current_block));
                    state = Section::Header;
                    block_index = current_block + 1;
                }
            }
        };
    }

    render_data(&fits);

    //println!("Size of data unit: {}", fits.data.data.len());
    //println!("Extension data: {}", fits.extension[0].data);
}

// interpret data based on header values,
fn render_data(fits: &Fits<Header, Data, Extension>) {
    println!("[Rendering FITS data]");

    let mut rendered_data: Vec<f32> = vec![0.0; fits.data.data.len() / 4];
    // @todo: check BITPIX - if it's 32 bit...
    use byteorder::{BigEndian, ByteOrder};
    BigEndian::read_f32_into(&fits.data.data, &mut rendered_data);

    // normalise and stretch the data for rendering / visualisation

    // write the data as a PNG or a BMP
}

fn parse_header(fits: &[u8], last_block: usize, current_block: usize) -> Header {
    let mut header_records = HashMap::new();
    let header_data = &fits[last_block * BLOCK_SIZE..(current_block + 1) * BLOCK_SIZE];

    println!("[Found header end]");

    for chunk in header_data.chunks(RECORD_SIZE) {
        let record_string = String::from_utf8_lossy(chunk);

        if let Some(Record { key, value }) = parse_record(record_string) {
            //println!("{}: {}", key, value);
            header_records.insert(key, value);
        }
    }

    Header {
        data: header_records,
    }
}

fn parse_data(fits: &[u8], last_block: usize, (x, y): (u32, u32)) -> Data {
    // @Todo: work out data size by data unit
    let data_size = x * y * 4;
    let data_unit =
        &fits[last_block * BLOCK_SIZE..(data_size as usize + (last_block * BLOCK_SIZE))];
    //println!("Data length: {}", data_unit.len());
    assert_eq!(data_size as usize, data_unit.len());

    Data { data: data_unit }
}

fn parse_extension(fits: &[u8], last_block: usize, current_block: usize) -> Extension {
    println!("[Found extension end]");
    let extension_data = &fits[last_block * BLOCK_SIZE..(current_block + 1) * BLOCK_SIZE];

    Extension {
        data: String::from_utf8_lossy(extension_data).to_string(),
    }
}

fn parse_record(record: Cow<str>) -> Option<Record> {
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
