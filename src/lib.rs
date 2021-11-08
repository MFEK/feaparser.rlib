//use allsorts;
//use nom;

pub mod language;
pub mod script;

use pest;
#[allow(unused_imports)]
// import actually is used by procedural macro, but Rust can't see it thus false warning
use pest::Parser;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "../fea.pest"]
pub struct FEAParser;
