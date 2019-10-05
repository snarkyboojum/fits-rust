use std::fs::File;
use std::path::Path;
use std::io::Read;

use std::error::Error;

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
        Ok(bytes) => println!("Sucessfully read {} bytes from {}", bytes, data_path.display())  
    };

}
