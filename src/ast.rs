use crate::builder::Builder;
use crate::error::{Error, Result};
use crate::Rule;
use pest::iterators::{Pair, Pairs};

macro_rules! rule_todo {
    ($pair:ident) => {
        panic!("Rule type {:?} not implemented", $pair.as_rule())
    };
}

pub struct FeatureAST<'a>(pub Option<Pairs<'a, Rule>>);

impl FeatureAST<'_> {
    pub fn build(&mut self, builder: &mut Builder) -> Result<()> {
        if let Some(pairs) = self.0.take() {
            self.build_items(builder, pairs)?;
        }
        Ok(())
    }

    fn build_items(&mut self, builder: &mut Builder, pairs: Pairs<Rule>) -> Result<()> {
        for pair in pairs {
            self.build_item(builder, pair)?
        }
        Ok(())
    }
    fn build_item(&mut self, builder: &mut Builder, pair: Pair<Rule>) -> Result<()> {
        match pair.as_rule() {
            Rule::include => rule_todo!(pair),
            Rule::file => rule_todo!(pair),
            Rule::topLevelStatement => rule_todo!(pair),
            Rule::glyphClassAssign => rule_todo!(pair),
            Rule::script => rule_todo!(pair),
            Rule::lang => rule_todo!(pair),
            Rule::langsysAssign => rule_todo!(pair),
            Rule::mark_statement => rule_todo!(pair),
            Rule::xval => rule_todo!(pair),
            Rule::yval => rule_todo!(pair),
            Rule::cp => rule_todo!(pair),
            Rule::anchorDef => rule_todo!(pair),
            Rule::valueRecordDef => rule_todo!(pair),
            Rule::conditionsetDef => rule_todo!(pair),
            Rule::condition => rule_todo!(pair),
            Rule::starttag => rule_todo!(pair),
            Rule::endtag => rule_todo!(pair),
            Rule::featureBlock => self._build_feature_statement(builder, pair.into_inner())?,
            Rule::variationFeatureBlock => rule_todo!(pair),
            Rule::tableBlock => rule_todo!(pair),
            Rule::anonBlock => rule_todo!(pair),
            Rule::lookupBlockTopLevel => rule_todo!(pair),
            Rule::lookupBlockOrUse => rule_todo!(pair),
            Rule::cvParameterBlock => rule_todo!(pair),
            Rule::cvParameterStatement => rule_todo!(pair),
            Rule::cvParameter => rule_todo!(pair),

            // Things where we just recurse into the different options
            Rule::featureStatement => self.build_items(builder, pair.into_inner())?,
            Rule::statement => self.build_items(builder, pair.into_inner())?,
            Rule::substitute => self.build_items(builder, pair.into_inner())?,

            Rule::reverse_substitute => rule_todo!(pair),
            Rule::gsub6 => self.dump(pair.into_inner()),
            Rule::gsub1a => self.dump(pair.into_inner()),
            Rule::gsub1b => self.dump(pair.into_inner()),
            Rule::gsub1c => self.dump(pair.into_inner()),
            Rule::gsub2 => self.dump(pair.into_inner()),
            Rule::gsub3 => self.dump(pair.into_inner()),
            Rule::gsub4 => self._build_ligature_subst(builder, pair.into_inner())?,

            Rule::featureUse => rule_todo!(pair),
            Rule::scriptAssign => rule_todo!(pair),
            Rule::langAssign => rule_todo!(pair),
            Rule::lookupflagAssign => rule_todo!(pair),
            Rule::lookupflagElement => rule_todo!(pair),
            Rule::ignoreSubOrPos => rule_todo!(pair),
            Rule::pos_mark => rule_todo!(pair),
            Rule::position => rule_todo!(pair),
            Rule::valuePattern => rule_todo!(pair),
            Rule::valueRecord => rule_todo!(pair),
            Rule::valueLiteral => rule_todo!(pair),
            Rule::deviceLiteral => rule_todo!(pair),
            Rule::cursiveElement => rule_todo!(pair),
            Rule::baseToMarkElement => rule_todo!(pair),
            Rule::ligatureMarkElement => rule_todo!(pair),
            Rule::parameters => rule_todo!(pair),
            Rule::sizemenuname => rule_todo!(pair),
            Rule::featureNames => rule_todo!(pair),
            Rule::subtable => rule_todo!(pair),
            Rule::table_BASE => rule_todo!(pair),
            Rule::baseStatement => rule_todo!(pair),
            Rule::axisTags => rule_todo!(pair),
            Rule::axisScripts => rule_todo!(pair),
            Rule::baseScript => rule_todo!(pair),
            Rule::table_GDEF => rule_todo!(pair),
            Rule::gdefStatement => rule_todo!(pair),
            Rule::gdefGlyphClass => rule_todo!(pair),
            Rule::gdefAttach => rule_todo!(pair),
            Rule::gdefLigCaretPos => rule_todo!(pair),
            Rule::gdefLigCaretIndex => rule_todo!(pair),
            Rule::table_head => rule_todo!(pair),
            Rule::headStatement => rule_todo!(pair),
            Rule::head => rule_todo!(pair),
            Rule::table_hhea => rule_todo!(pair),
            Rule::hheaStatement => rule_todo!(pair),
            Rule::hhea => rule_todo!(pair),
            Rule::table_vhea => rule_todo!(pair),
            Rule::vheaStatement => rule_todo!(pair),
            Rule::vhea => rule_todo!(pair),
            Rule::table_name => rule_todo!(pair),
            Rule::nameStatement => rule_todo!(pair),
            Rule::nameID => rule_todo!(pair),
            Rule::table_OS_2 => rule_todo!(pair),
            Rule::os_2Statement => rule_todo!(pair),
            Rule::os_2 => rule_todo!(pair),
            Rule::table_STAT => rule_todo!(pair),
            Rule::statStatement => rule_todo!(pair),
            Rule::designAxis => rule_todo!(pair),
            Rule::axisValue => rule_todo!(pair),
            Rule::axisValueStatement => rule_todo!(pair),
            Rule::axisValueLocation => rule_todo!(pair),
            Rule::axisValueFlags => rule_todo!(pair),
            Rule::elidedFallbackName => rule_todo!(pair),
            Rule::nameEntryStatement => rule_todo!(pair),
            Rule::elidedFallbackNameID => rule_todo!(pair),
            Rule::nameEntry => rule_todo!(pair),
            Rule::table_vmtx => rule_todo!(pair),
            Rule::vmtxStatement => rule_todo!(pair),
            Rule::vmtx => rule_todo!(pair),
            Rule::anchor => rule_todo!(pair),
            Rule::lookupPattern => rule_todo!(pair),
            Rule::lookupPatternElement => rule_todo!(pair),
            Rule::pattern => rule_todo!(pair),
            Rule::patternElement => rule_todo!(pair),
            Rule::glyphClassOptional => rule_todo!(pair),
            Rule::glyphClass => rule_todo!(pair),
            Rule::gcLiteral => rule_todo!(pair),
            Rule::gcLiteralElement => rule_todo!(pair),
            Rule::glyph => rule_todo!(pair),
            Rule::glyphName => rule_todo!(pair),
            Rule::label => rule_todo!(pair),
            Rule::tag => rule_todo!(pair),
            Rule::fixedNum => rule_todo!(pair),
            Rule::genNum => rule_todo!(pair),
            Rule::subtok => rule_todo!(pair),
            Rule::revtok => rule_todo!(pair),
            Rule::anontok => rule_todo!(pair),
            Rule::enumtok => rule_todo!(pair),
            Rule::postok => rule_todo!(pair),
            Rule::markligtok => rule_todo!(pair),
            Rule::location => rule_todo!(pair),
            Rule::master => rule_todo!(pair),
            Rule::varnum => rule_todo!(pair),
            Rule::num_or_varnum => rule_todo!(pair),
            _ => {}
        }
        Ok(())
    }

    pub fn _build_feature_statement(
        &mut self,
        builder: &mut Builder,
        mut feat: Pairs<Rule>,
    ) -> Result<()> {
        feat.next();
        let name_token = feat.next().unwrap();
        let name = name_token.as_str();
        let mut feat = feat.peekable();
        builder.start_feature(name);
        let mut features = std::mem::take(&mut builder.features);

        if let Some(Rule::USE_EXTENSION) = feat.peek().map(|x| x.as_rule()) {
            feat.next();
        }
        feat.next();

        while let Some(Rule::featureStatement) = feat.peek().map(|x| x.as_rule()) {
            self.build_item(builder, feat.next().unwrap())?;
        }
        feat.next();
        let endtag = feat.next().unwrap().as_str();
        if name != endtag {
            let (line, col) = name_token.as_span().start_pos().line_col();
            return Err(Error::MismatchedTag {
                line,
                col,
                start_tag: name.to_string(),
                end_tag: endtag.to_string(),
            });
        }

        std::mem::swap(&mut builder.features, &mut features);
        builder.features.extend(features);
        builder.end_feature();
        Ok(())
    }
    pub fn dump(&mut self, mut pairs: Pairs<Rule>) {
        for p in pairs {
            println!("{:#?}", p);
        }
        panic!("At the disco");
    }

    pub fn _build_ligature_subst(
        &mut self,
        builder: &mut Builder,
        mut feat: Pairs<Rule>,
    ) -> Result<()> {
        feat.next();
        let mut from_glyphs: Vec<&str> = vec![];
        while let Some(Rule::glyphOrClass) = feat.peek().map(|x| x.as_rule()) {
            from_glyphs.push(feat.next().unwrap().as_str());
        }
        feat.next();
        let to_glyph = feat.next().unwrap().as_str();
        builder.add_ligature_subst(None, from_glyphs, None, to_glyph)?;
        Ok(())
    }
}
