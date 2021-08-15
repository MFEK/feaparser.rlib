//use allsorts;
//use nom;

pub mod ast;
pub mod builder;
pub mod error;
pub mod language;
pub mod script;

use crate::ast::FeatureAST;
pub use builder::Builder;

use pest::{self, error::Error, Parser};
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "../fea.pest"]
pub struct FEAParser;

impl<'a> FEAParser {
    pub fn parse_to_ast(input: &'a str) -> Result<FeatureAST<'a>, Error<Rule>> {
        FEAParser::parse(Rule::file, input).map(|x| FeatureAST(Some(x)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::Builder;
    use fonttools::font::{Font, Table};
    use fonttools::maxp::maxp;
    use std::ffi::OsStr;
    use std::fs;
    use std::fs::File;

    macro_rules! test_parses {
        ($name:ident, $string:expr, $expect:expr) => {
            #[test]
            fn $name() {
                let ast = FEAParser::parse($expect, $string);
                if let Err(err) = ast {
                    panic!("{}", err);
                }
            }
        };
    }

    test_parses!(test_anchor, "<anchor 200 300>", Rule::anchor);
    test_parses!(
        test_gpos4,
        "pos mark hamza <anchor 200 300> mark @MC1",
        Rule::gpos4
    );

    test_parses!(
        test_gpos4_2,
        "pos mark [acute grave macron ogonek]
        <anchor 500 200> mark @TOP_MARKS
        <anchor 500 -80> mark @BOTTOM_MARKS",
        Rule::gpos4
    );

    test_parses!(
        test_feature_names,
        "featureNames {
        name \"Feature description for MS Platform, script Unicode, language English\";
        };",
        Rule::statement
    );

    test_parses!(test_gsub4, "sub f i by f_i", Rule::gsub4);
    test_parses!(test_gsub4_sub, "sub f i by f_i", Rule::substitute);
    #[test]
    fn test_fonttools_test_suite() {
        let mut ok = 0;
        let mut fails = 0;
        let paths = fs::read_dir("./tests/data/").unwrap();

        for path in paths {
            let path = path.as_ref().unwrap().path();
            if path.extension() == Some(OsStr::new("fea")) {
                let data = fs::read_to_string(path.clone()).unwrap();
                let ast = FEAParser::parse(Rule::file, &data);
                if let Err(err) = ast {
                    println!("{}{}\n", path.to_str().unwrap().to_string(), err);
                    fails += 1;
                } else {
                    ok += 1;
                }
            }
        }
        if fails > 0 {
            println!("\n{} OK, {} failing\n", ok, fails);
            panic!();
        }
    }

    use std::iter::FromIterator;
    macro_rules! glyphset {
        ($($k:expr => $v:expr),* $(,)?) => {
            std::collections::HashMap::<String, u16>::from_iter(std::array::IntoIter::new([$(($k.to_string(), $v),)*]))
        };
    }

    #[test]
    fn test_feabuilder() {
        let mut font = Font::new(fonttools::font::SfntVersion::TrueType);
        font.tables.insert(*b"maxp", Table::Maxp(maxp::new05(5)));

        let test = r#"
                @stuff = [f i fi];

                languagesystem DFLT dflt;

                lookup foobar {
                    lookupflag RightToLeft UseMarkFilteringSet [f i];
                    sub a by b;
                } foobar;

                feature liga {
                    sub f f i by f_f_i;
                    sub f i by fi;

                    script latn;
                    language TRK;
                    sub f l by fl;
                } liga;

                feature test {
                    sub @stuff by f;
                } test;
        "#;

        let ast = FEAParser::parse_to_ast(test);
        assert!(ast.is_ok());

        let glyphset = glyphset!(
            "a" => 1,
            "b" => 2,
            "f" => 6,
            "i" => 9,
            "l" => 12,
            "fi" => 120,
            "f_f_i" => 121,
            "fl" => 122,
        );
        let mut builder = Builder::new(glyphset);
        builder
            .build(ast.unwrap(), &mut font)
            .expect("Couldn't build");
        let mut outfile = File::create("test.ttf").expect("Could not create file");
        font.save(&mut outfile);
    }
}
