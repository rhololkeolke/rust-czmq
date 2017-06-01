extern crate pkg_config;

fn main() {
    // find version 4 of libczmq and output the proper linker flags
    pkg_config::Config::new().atleast_version("4.0").probe("libczmq").unwrap();

    // either copy over the bundled bindgen bindings or run bindgen now
    bindings::write_to_out_dir();
}

#[cfg(not(feature = "buildtime_bindgen"))]
mod bindings {
    use std::{env, fs};
    use std::path::{PathBuf, Path};

    pub fn write_to_out_dir() {
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
        let in_path = Path::new("bindgen-bindings/bindgen.rs");
        fs::copy(in_path, out_path).expect("Could not copy bindings to output directory");
    }
}

#[cfg(feature = "buildtime_bindgen")]
mod bindings {
    extern crate bindgen;
    
    use std::env;
    use std::path::PathBuf;

    pub fn write_to_out_dir() {
        let bindings = bindgen::Builder::default()
            .no_unstable_rust()
            .header("bindgen.h")
            .opaque_type("zmq_msg_t")
            .hide_type("IPPORT_RESERVED")
            .hide_type("max_align_t") // https://github.com/servo/rust-bindgen/issues/550
            .generate()
            .expect("Unable to generate bindings");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
        bindings
            .write_to_file(out_path)
            .expect("Couldn't write bindings!");
    }
}
