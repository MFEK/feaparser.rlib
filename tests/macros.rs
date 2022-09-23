#[macro_export]
macro_rules! test_ast {
    ($ast:ident, $prefix:expr) => {
        match &$ast {
            Err(err) => eprintln!("{}{}", $prefix, err),
            Ok(ref a) => {
                if env::var("DUMP_TREES").is_ok() {
                    let a = a.clone();
                    eprintln!("{}", into_ascii_tree(a).unwrap())
                }
            }
        };
    };
    ($ast:ident) => {
        test_ast!($ast, "");
    };
}
#[macro_export]
macro_rules! test_parses {
    ($name:ident, $string:expr, $expect:expr) => {
        #[test]
        fn $name() {
            let ast = FEAParser::parse($expect, $string);
            test_ast!(ast);
        }
    };
}
