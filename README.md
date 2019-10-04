# FITS in Rust
Simple Rust libraries to process FITS files. See and [FITS at NASA/GFSC](https://fits.gsfc.nasa.gov/) and [FITS - Wikipedia](https://en.wikipedia.org/wiki/FITS).

## Brief overview
Cover the objective and goals of the series.

## Intro to a subset of Rust that we will use
	* Cargo and rustup (stable vs nightly etc) and running our first Rust program
	* Functions, control flow, looping
	* Structs and method dispatch
	* Enums a parameterised enums
	* Error handling
	* Concurrency primitives, especially channels and message passing
	* Std library functions we might be interested in, file and directory handling etc

## Parsing FITS files
	* Simple file handling in Rust
	* Parse FITS v4 files
	* Simple - Header and HDUs
	* Intermediate - Add functionality to inspect headers and HDUs
	* Advanced - Return more metadata and other FITS detail
	* Enhance the code to do what it can currently do in parallel (whatever is embarrassingly parallel)
	* Handing FITS files that don’t fit into available system memory, i.e. memory mapping
	* Working with compressed FITS files

## Display FITS files
	* Use a cross platform crate to do UI?
	* Render binary / images
	* Add a view into other data types, e.g. tabular data

## Going faster
	* Benchmarking existing functions
	* Comparing to astro.py FITS capability
	* Making the code go faster
  
## Processing and updating FITS files
	* Updating FITS files
	* Creating new FITS files
	* Validating FITS files
  


