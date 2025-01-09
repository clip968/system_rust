use std::path::PathBuf;

fn main() -> miette::Result<()> {
    let path = PathBuf::from("src");

    let mut b = autocxx_build::Builder::new("./src/main.rs", &["src"])
        .build()
        .map_err(|e| miette::Report::msg(format!("Autocxx build error: {:?}", e)))?; // 에러 변환

    b.flag_if_supported("-std=c++14").compile("autocxx-demo");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/input.h");

    Ok(())
}
