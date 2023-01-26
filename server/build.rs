use minify_html::{minify, Cfg};
use std::fs;
use std::io::{BufReader, Write};
use std::path::*;
use std::{fs::File, io::Read};
extern crate glob;
use glob::glob;
use tonic_build;

fn parse_file(src: &Path, dist: &Path) {
    let file = File::open(src).unwrap();
    let mut reader = BufReader::new(file);

    let buffer = &mut Vec::new();

    reader.read_to_end(buffer).unwrap();

    let mut cfg = Cfg::new();
    cfg.keep_comments = false;
    cfg.minify_css = true;
    cfg.minify_js = true;

    let minified = minify(&buffer, &cfg);

    let mut file = File::create(dist).unwrap();

    file.write(&minified).expect("error running minify-html");
}

fn parse_folder(path: &str) {
    fs::create_dir_all(format!("tmp/{}", path)).unwrap();

    let pattern = &format!("{}/*", path);
    for entry in glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let src = Path::new(&path);
                let dist = Path::new("tmp");
                let dist = &dist.join(src.clone());

                parse_file(src, dist);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}

fn main() {
    parse_folder("static");
    tonic_build::compile_protos("../proto/control.proto").unwrap();
    tonic_build::compile_protos("../proto/helloworld.proto").unwrap();
}
