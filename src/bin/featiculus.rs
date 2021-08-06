use clap::{App, Arg};
use feaparser::{Builder, FEAParser};
use fonttools::font::Table;
use std::collections::HashMap;
use std::fs::File;

fn main() {
    let matches = App::new("featiculus")
        .about("Add features to a font file (quickly)")
        .arg(
            Arg::with_name("OUTPUT")
                .short("o")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("FONT")
                .help("Sets the font file to use")
                .required(true),
        )
        .arg(
            Arg::with_name("FEA")
                .help("Sets the feature file to use")
                .required(true),
        )
        .get_matches();
    let filename = matches.value_of("FONT").unwrap();
    let infile = File::open(filename).unwrap();
    let mut font = fonttools::font::load(infile).expect("Could not parse font");

    let fea_fn = matches.value_of("FEA").unwrap();
    let fea = std::fs::read_to_string(fea_fn).expect("Couldn't read feature file");

    let ast = FEAParser::parse_to_ast(&fea).expect("Couldn't parse feature file");

    let mut glyphset: HashMap<String, u16> = HashMap::new();

    let glyphnames = font
        .get_table(b"post")
        .expect("Error reading post table")
        .expect("No post table found")
        .post_unchecked()
        .glyphnames
        .as_ref()
        .expect("No glyphnames");
    for (ix, name) in glyphnames.iter().enumerate() {
        glyphset.insert(name.to_string(), ix as u16);
    }

    let mut builder = Builder::new(glyphset);
    let res = builder.build(ast, &mut font);
    if res.is_err() {
        println!("Building failed: {}", res.err().unwrap());
        std::process::exit(1);
    }

    let mut outfile = if matches.is_present("OUTPUT") {
        File::create(matches.value_of("OUTPUT").unwrap())
    } else {
        File::create(filename)
    }
    .expect("Could not open file for writing");
    font.save(&mut outfile);
}
