use std::process::Command;
use std::env;
use std::path;

use std::io::Write;

#[derive(derive_more::From,Debug)]
enum BuildErrors {
    IOError(std::io::Error),
    EnvError(std::env::VarError),
    BindError,
}


fn main() -> Result<(), BuildErrors> {


    if !path::Path::new("myhtml/include/myhtml/api.h").exists() {
        Command::new("git").args(&["submodule", "init"]).status()?;
        Command::new("git").args(&["submodule", "update"]).status()?;
        Command::new("make").arg("static").current_dir("myhtml").status()?;
    }

    let out_dir = env::var("OUT_DIR")?;
    let out_path = path::PathBuf::from(&out_dir);


    if !path::Path::new("src/bindings.rs").exists() {
        let file = std::fs::OpenOptions::new().create(true).write(true).open("src/bindings.rs")?;
        let mut file = std::io::BufWriter::new(file);
        file.write_all("
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(dead_code)]
".as_bytes())?;

        let bindings = bindgen::Builder::default()
            .clang_arg("-Imyhtml/include")
            .header("wrapper.h")
            .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: false })
            .generate().or(Err(BuildErrors::BindError))?;

        bindings
            .write(Box::new(file))?;
    }
    std::fs::copy("myhtml/lib/libmyhtml_static.a", out_path.join("libmyhtml_static.a"))?;
    println!("cargo:rustc-link-search=native={}", &out_dir);
    println!("cargo:rustc-link-lib=static=myhtml_static");

    Ok(())
}