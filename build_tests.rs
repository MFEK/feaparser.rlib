use std::env;
use std::fs::read_dir;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let destination = Path::new(&out_dir).join("tests.rs");
    let tests = read_dir("tests/data/fonttools/").unwrap();

    let mut test_file = File::create(&destination).unwrap();

    for f in tests {
        let f = f.unwrap();
        let filename = f.file_name().to_str().unwrap().to_string();
        if &filename == "README.md" {
            continue
        }
        write!(test_file, include_str!("test_template.rs"), name = filename.strip_suffix(".fea").unwrap_or(&filename), path = f.path().display()).unwrap();
    }
}
