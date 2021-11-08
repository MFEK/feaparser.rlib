use pest_ascii_tree::into_ascii_tree;
use feaparser::{FEAParser, Rule};
use pest::{self, Parser};

use std::env;
use std::fs;

macro_rules! test_parses {
    ($name:ident, $string:expr, $expect:expr) => {
        #[test]
        fn $name() {
            let ast = FEAParser::parse($expect, $string);
            match ast {
                Err(err) => panic!("{}", err),
                Ok(ast) => {
                    if env::var("DUMP_TREES").is_ok() {
                        println!("{}", into_ascii_tree(ast).unwrap())
                    }
                }
            };
        }
    };
}

test_parses!(test_feaparser_long, include_str!("../tests/data/feaparser_long.fea"), Rule::file);

test_parses!(test_anon, "anonymous vroo { l } vroo;", Rule::anonBlock);

test_parses!(test_anchor, "<anchor 200 300>", Rule::anchor);
test_parses!(
    test_pos_mark,
    "mark hamza <anchor 200 300> mark @MC1",
    Rule::pos_mark
);

test_parses!(
    test_pos_mark_2,
    "mark [acute grave macron ogonek]
    <anchor 500 200> mark @TOP_MARKS
    <anchor 500 -80> mark @BOTTOM_MARKS",
    Rule::pos_mark
);

test_parses!(
    test_feature_names,
    "featureNames {
    name \"Feature description for MS Platform, script Unicode, language English\";
    };",
    Rule::statement
);

#[test]
fn test_fonttools_test_suite() {
    let mut ok = 0;
    let mut fails = 0;
    let paths = fs::read_dir("./tests/data/fonttools/").unwrap();

    for path in paths {
        let path = path.as_ref().unwrap().path();
        let data = fs::read_to_string(path.clone()).unwrap();
        let ast = FEAParser::parse(Rule::file, &data);
        if let Err(err) = ast {
            println!("{}{}\n", path.to_str().unwrap().to_string(), err);
            fails += 1;
        } else {
            ok += 1;
        }
    }
    if fails > 0 {
        println!("\n{} OK, {} failing\n", ok, fails);
        panic!();
    }
}
