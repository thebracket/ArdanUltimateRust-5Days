use std::{path::PathBuf, env};

fn main() {
    cc::Build::new()
        .file("src/crust.c")
        .compile("crust");

    let bindings = bindgen::Builder::default()
        .header("src/crust.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}