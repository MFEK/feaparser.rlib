#![allow(non_snake_case)]

use feaparser::{FEAParser, Rule};
use pest::Parser as _;
use pest_ascii_tree::into_ascii_tree;

use std::env;

#[macro_use]
mod macros;

include!(concat!(env!("OUT_DIR"), "/tests.rs"));
