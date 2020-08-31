# edlib-rs

This crate provides a Rust interface to the Edlib C++ library by Martin Šošić. [Martinsos edlib](https://github.com/Martinsos/edlib)

The algorithm implemented is the computation of the edit (or Levenshtein) distance as described in the paper:

Martin Šošić, Mile Šikić; Edlib: a C/C ++ library for fast, exact sequence alignment using edit distance. Bioinformatics 2017 [btw753. doi] <https://doi.org/10.1093/bioinformatics/btw753>

The interface mostly relies on bindgen (see module bindings). A more Rust idiomatic interface is provided
in module edlibrs. it comes at the cost of cloning information stored pointers startLocations and endLocations in **EdlibAlignResult** to get a Rust **struct EdlibAlignResultRs** with **Option<Vec\<u8\>>** fields.

## Installation

The crate relies on the C++ edlib library being installed and compiled as described in edlib documentation.  
Before running cargo build (or cargo install) the environment variable EDLIB_DIR must be set to where the original C++ edlib directory was cloned for the build.rs step of cargo to access the necessary includes.

## License

Licensed under either of

* Apache License, Version 2.0, [LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>
* MIT license [LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>

at your option.

This software was written on my own while working at [CEA](http://www.cea.fr/), [CEA-LIST](http://www-list.cea.fr/en/)