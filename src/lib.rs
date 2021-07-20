//use allsorts;
//use nom;

pub mod ast;
pub mod builder;
pub mod language;
pub mod script;

use crate::ast::FeatureAST;
use crate::builder::Builder;

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
    use std::fs;

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

    #[test]
    fn test_feaparser() {
        let test = r#"@lol = [Qol Mol @lol];
    languagesystem DFLT dflt;
    include(te\)st);
    # include(te)st); would fail
    include (lol);

    anonymous jig { @lol = [lol]; {@Q = [Q R S T];} name ";}{"; {}; "}"; } jig;
    anonymous jjig { } jjig;
    anonymous jjig {} jjig;
    anonymous FIVE { {} {} {} } FIVE;
    # This'd be invalid FEA syntax due to tag mismatch. It's up to struct builder to check this! Not possible in a grammar.
    anonymous LAST {
        It's the end of the world as we know it
        And I feel fine!
    } FRST;

    name 0x3 0x1 0x411;

    feature mark {

    } mark;

    feature liga {
        featureNames {
            name 0x3 0x1 0x411 "Feature description for MS Platform, script Unicode, language Japanese";
            name "b";
        };
        name "C";
    } liga;

    table GDEF {
        LigatureCaretByPos lol 0;
    } GDEF;

    table head {
        FontRevision 0.0;
    } head;

    table OS/2 {
    FSType 4;
    Panose 2 15 0 0 2 2 8 2 9 4;
    TypoAscender 800;
    TypoDescender -200; # Note that TypoDescender is negative for descent below the baseline.
    winAscent 832;
    winDescent 321; # Note that winDescent is positive for descent below the baseline.
    UnicodeRange
        0   # Basic Latin
        1   # Latin-1 Supplement
        9   # Cyrillic
        55  # CJK Compatibility
        59  # CJK Unified Ideographs
        60  # Private Use Area
        ;
    CodePageRange
        1252    # Latin 1
        1251    # Cyrillic
        932     # JIS/Japan
        ;
    XHeight 400;
    CapHeight 600;
    WeightClass 800;
    WidthClass 3;
    Vendor "ADBE";
    FamilyClass 0x0805;  # Class 8 (Sans-serif), Subclass 5 (Neo-grotesque Gothic)
} OS/2;

feature aalt {
    featureNames {
        name "Fancy Q's";
    };
    lookup aalt_1 {
        sub Q from [Q.ss01 Q.ss02 Q.ss03];
    } aalt_1;
} aalt;


variation rvrn heavy {
    lookup symbols_heavy;
    lookup letters_heavy;
} rvrn;

anchorDef 120 120 ANCHOR_1;
anchorDef 120 -20 contourpoint 5 ANCHOR_2;
valueRecordDef -10 FIRST_KERN;
valueRecordDef <0 0 20 0> SECOND_KERN;

feature liga {
    sub A by B;
    sub @A by @B;
    sub B by A B C;
    sub f f by f_f;
    subtable;
    sub f i by f_i;
    sub f l by f_l;
    sub f l' lookup test;
    sub f l' by y;
    sub Q by NULL;
    lookup inside_lu {sub \NULL by NULL;}inside_lu;
} liga;
# comment ça va
#
    "#;
        let ast = FEAParser::parse(Rule::file, test);
        //eprintln!("{:?}", ast);
        use pest_ascii_tree::{self, into_ascii_tree};
        eprintln!("{}", into_ascii_tree(ast.unwrap()).unwrap());
    }

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
        let paths = fs::read_dir("./tests/").unwrap();

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

    #[test]
    fn test_feabuilder() {
        let test = r#"
    feature liga {
        sub f i by f_i;
    } liga;
    "#;
        let mut ast = FEAParser::parse_to_ast(test);
        assert!(ast.is_ok());

        let mut builder = Builder::new();
        ast.unwrap().build(&mut builder);
    }
}
