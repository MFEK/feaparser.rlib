use pest_ascii_tree::into_ascii_tree;
use feaparser::{FEAParser, Rule};
use pest::{self, Parser};

use std::env;

#[macro_use]
mod macros;

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
