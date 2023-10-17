use std::path::PathBuf;

fn main() {
    let dir: PathBuf = ["tree-sitter-universql", "src"].iter().collect();
    let parser_path = dir.join("parser.c");

    cc::Build::new()
        .include(&dir)
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .flag_if_supported("-Wno-trigraphs")
        .file(&parser_path)
        .compile("tree-sitter-universql");

    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());
}
