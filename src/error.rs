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

    #[snafu(display("Unknown glyph class {}", name))]
    UnknownGlyphClass { name: String },

    #[snafu(display("Lookup {} has already been defined", name))]
    DuplicateLookup { name: String },

    #[snafu(display("Within a named lookup, all rules must of the same lookup type and flag"))]
    DisparateRulesInLookup,

    #[snafu(display("Language statements are not allowed within a {} feature", feature))]
    BadLanguageInFeature { feature: String },
    #[snafu(display("Script statements are not allowed within a {} feature", feature))]
    BadScriptInFeature { feature: String },

    #[snafu(display("Language statements are not allowed within a lookup"))]
    BadLanguageInLookup,
    #[snafu(display("Script statements are not allowed within a lookup"))]
    BadScriptInLookup,

    #[snafu(display("Named lookups are not permitted within an aalt feature"))]
    NoLookupsInAalt,

    #[snafu(display(
        "A substitution was already declared from {} to GID{}",
        from_glyph,
        to_gid
    ))]
    DuplicateSubstitution { from_glyph: String, to_gid: u16 },

    #[snafu(display(
        "A glyph {} was already assigned to a different mark class {}",
        glyph,
        old_id
    ))]
    GlyphAlreadyAssignedToMarkAttachmentClass { glyph: String, old_id: u16 },
}

pub type Result<'a, T, E = Error> = std::result::Result<T, E>;
