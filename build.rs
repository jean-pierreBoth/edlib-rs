extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

// We have path to edlib, generate wrapper.h with  path to correct include 
fn write_wrapper(edlibpath : &PathBuf) {
    let mut inclpath = edlibpath.clone();
    inclpath.push("edlib");
    inclpath.push("include");
    inclpath.push("edlib.h");
    let mut incl = String::from("#include<");
    incl.push_str(& inclpath.to_str().unwrap());
    incl.push_str(">");
    // now we write file wrapper.h
    let mut f = File::create("wrapper.h").unwrap();
    f.write_all(incl.as_bytes()).expect("cannot write wrapper.h");
}



fn main() {
    let edlib_env = env::var("EDLIB_DIR");
    let edlib_src = edlib_env.expect("env variable EDLIB_DIR not set");
    let edlib_path = PathBuf::from(edlib_src);
    //
    write_wrapper(&edlib_path);
    //
    let mut libdir = String::from("cargo:rustc-link-search=");
    libdir.push_str(edlib_path.to_str().unwrap());
    libdir.push_str("/build/lib");
    println!("{}",libdir);
//    println!("cargo:rustc-link-search=/home.1/jpboth/Soft/edlib/build/lib");
    println!("cargo:rustc-link-lib=edlib");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        //Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
         // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let src_path = PathBuf::from("src");
    //
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    // We dump bindings.rs in src directory beccause it is included in lib.rs
    bindings.write_to_file(src_path.join("bindings.rs")).expect("Couldn't write bindings in src/ !");
    
}