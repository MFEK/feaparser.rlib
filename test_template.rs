test_parses!(test_{name}, include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/{path}")), Rule::file);
