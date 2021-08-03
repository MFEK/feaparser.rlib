use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Parse error: {} at l.{}, col.{}", message, line, col))]
    Parse {
        line: usize,
        col: usize,
        message: String,
    },

    #[snafu(display(
        "Parse error: mismatched tag: {} at l.{} col.{} ended with {}",
        start_tag,
        line,
        col,
        end_tag
    ))]
    MismatchedTag {
        line: usize,
        col: usize,
        start_tag: String,
        end_tag: String,
    },

    #[snafu(display("Unknown glyph: {} not found in font", name))]
    UnknownGlyph { name: String },

    #[snafu(display("Within a named lookup, all rules must of the same lookup type and flag"))]
    DisparateRulesInLookup,

    #[snafu(display("Language statements are not allowed within a {} feature", feature))]
    BadLanguageInFeature { feature: String },

    #[snafu(display("Language statements are not allowed within a lookup"))]
    BadLanguageInLookup,
}

pub type Result<'a, T, E = Error> = std::result::Result<T, E>;
