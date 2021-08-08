use crate::builder::Builder;
use crate::error::{Error, Result};
use crate::Rule;
use fonttools::layout::common::LookupFlags;
use fonttools::layout::valuerecord::ValueRecord;
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
            Rule::glyphClassAssign => self._assign_glyph_class(builder, pair.into_inner())?,
            Rule::script => rule_todo!(pair),
            Rule::lang => rule_todo!(pair),
            Rule::langsysAssign => self._assign_language_system(builder, pair.into_inner())?,
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
            Rule::cvParameterBlock => rule_todo!(pair),
            Rule::cvParameterStatement => rule_todo!(pair),
            Rule::cvParameter => rule_todo!(pair),
            Rule::lookupBlock => self._build_lookup_block(builder, pair.into_inner())?,
            Rule::lookupBlockTopLevel => self._build_lookup_block(builder, pair.into_inner())?,
            Rule::lookupUse => self._build_lookup_use(builder, pair.into_inner())?,

            // Things where we just recurse into the different options
            Rule::topLevelStatement => self.build_items(builder, pair.into_inner())?,
            Rule::featureStatement => self.build_items(builder, pair.into_inner())?,
            Rule::statement => self.build_items(builder, pair.into_inner())?,
            Rule::substitute => self.build_items(builder, pair.into_inner())?,
            Rule::position => self.build_items(builder, pair.into_inner())?,
            Rule::lookupBlockOrUse => self.build_items(builder, pair.into_inner())?,

            Rule::reverse_substitute => rule_todo!(pair),
            Rule::gsub6 => self.dump(pair),
            Rule::gsub1a => self._build_gsub1a(builder, pair.into_inner())?,
            Rule::gsub1b => self._build_gsub1b(builder, pair.into_inner())?,
            Rule::gsub1c => self.dump(pair),
            Rule::gsub2 => self.dump(pair),
            Rule::gsub3 => self.dump(pair),
            Rule::gsub4 => self._build_ligature_subst(builder, pair.into_inner())?,

            Rule::gpos1 => self._build_gpos1(builder, pair.into_inner())?,
            Rule::gpos2 => rule_todo!(pair),
            Rule::gpos3 => rule_todo!(pair),
            Rule::gpos4 => rule_todo!(pair),
            Rule::gpos5 => rule_todo!(pair),
            Rule::gpos6 => rule_todo!(pair),
            Rule::gpos8 => rule_todo!(pair),
            Rule::gpos_complex => rule_todo!(pair),

            Rule::featureUse => rule_todo!(pair),
            Rule::scriptAssign => self._build_script(builder, pair.into_inner())?,
            Rule::langAssign => self._build_language(builder, pair.into_inner())?,
            Rule::lookupflagAssign => self._build_lookup_flag(builder, pair.into_inner())?,
            Rule::lookupflagElement => rule_todo!(pair),
            Rule::ignoreSubOrPos => rule_todo!(pair),

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

    fn _build_feature_statement(
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

    fn _build_lookup_use(&mut self, builder: &mut Builder, mut lookup: Pairs<Rule>) -> Result<()> {
        lookup.next();
        let name_token = lookup.next().unwrap();
        let name = name_token.as_str();
        builder.add_lookup_call(name)
    }

    fn _build_lookup_block(
        &mut self,
        builder: &mut Builder,
        mut lookup: Pairs<Rule>,
    ) -> Result<()> {
        // LOOKUP ~ label ~ USE_EXTENSION? ~ LCBRACE ~ statement* ~ RCBRACE ~ label ~ SEMI
        lookup.next();
        let name_token = lookup.next().unwrap();
        let name = name_token.as_str();
        let mut lookup = lookup.peekable();
        builder.start_lookup_block(name)?;
        if let Some(Rule::USE_EXTENSION) = lookup.peek().map(|x| x.as_rule()) {
            lookup.next();
        }
        lookup.next();

        while let Some(Rule::statement) = lookup.peek().map(|x| x.as_rule()) {
            self.build_item(builder, lookup.next().unwrap())?;
        }
        lookup.next();
        let endtag = lookup.next().unwrap().as_str();
        if name != endtag {
            let (line, col) = name_token.as_span().start_pos().line_col();
            return Err(Error::MismatchedTag {
                line,
                col,
                start_tag: name.to_string(),
                end_tag: endtag.to_string(),
            });
        }

        builder.end_lookup_block();
        Ok(())
    }

    fn _build_lookup_flag(
        &mut self,
        builder: &mut Builder,
        mut statement: Pairs<Rule>,
    ) -> Result<()> {
        statement.next();
        let mut flag = LookupFlags::empty();
        let mut mark_attachment = None;
        let mut mark_filtering_set = None;
        for el in statement {
            if el.as_rule() == Rule::NUM {
                flag = LookupFlags::from_bits_truncate(el.as_str().parse::<u16>().unwrap());
                break;
            } else {
                // It's a lookupflagElement
                let mut lookup_flag_element = el.into_inner();
                match lookup_flag_element.next().unwrap().as_rule() {
                    Rule::RIGHT_TO_LEFT => flag |= LookupFlags::RIGHT_TO_LEFT,
                    Rule::IGNORE_BASE_GLYPHS => flag |= LookupFlags::IGNORE_BASE_GLYPHS,
                    Rule::IGNORE_MARKS => flag |= LookupFlags::IGNORE_MARKS,
                    Rule::IGNORE_LIGATURES => flag |= LookupFlags::IGNORE_LIGATURES,
                    Rule::MARK_ATTACHMENT_TYPE => {
                        mark_attachment =
                            Some(self._expand_class(builder, lookup_flag_element.next().unwrap())?);
                    }
                    Rule::USE_MARK_FILTERING_SET => {
                        mark_filtering_set =
                            Some(self._expand_class(builder, lookup_flag_element.next().unwrap())?);
                    }
                    _ => panic!("Can't happen"),
                }
            }
        }
        builder.set_lookup_flag(flag, mark_attachment, mark_filtering_set)
    }

    pub fn dump(&mut self, pair: Pair<Rule>) {
        println!("It's a {:?}", pair);

        for p in pair.into_inner() {
            println!("  {:#?}", p);
        }
        panic!("At the disco");
    }

    fn _assign_language_system(
        &mut self,
        builder: &mut Builder,
        mut statement: Pairs<Rule>,
    ) -> Result<()> {
        statement.next();
        let script = statement.next().unwrap().as_str();
        let lang = statement.next().unwrap().as_str();
        builder.add_language_system(script, lang)
    }

    fn _assign_glyph_class(
        &mut self,
        builder: &mut Builder,
        mut statement: Pairs<Rule>,
    ) -> Result<()> {
        let name = statement.next().unwrap();
        statement.next();
        let glyphs: Vec<String> = self._expand_class(builder, statement.next().unwrap())?;
        builder
            .glyph_classes
            .insert(name.as_str().to_string(), glyphs);
        Ok(())
    }

    fn _expand_class(&mut self, builder: &mut Builder, class: Pair<Rule>) -> Result<Vec<String>> {
        let mut glyphs: Vec<String> = vec![];
        let class = class.into_inner().next().unwrap();
        match class.as_rule() {
            Rule::gcLiteral => {
                let mut elements = class.into_inner().peekable();
                elements.next(); // [
                while let Some(Rule::gcLiteralElement) = elements.peek().map(|x| x.as_rule()) {
                    let element = elements.next().unwrap();
                    let this_element: Vec<Pair<Rule>> = element.into_inner().collect();
                    let first = this_element.first().unwrap();
                    // if it's a GCLASS, recurse.
                    if let Rule::GCLASS = first.as_rule() {
                        let mut inner_class = self._expand_class(builder, first.clone())?;
                        glyphs.append(&mut inner_class);
                    } else if this_element.len() == 3 {
                        // It's a range
                        panic!("Feed me");
                    } else {
                        glyphs.push(first.as_str().to_string());
                    }
                }
                Ok(glyphs)
            }
            Rule::GCLASS => {
                let gcstring = class.as_str();
                return builder
                    .glyph_classes
                    .get(gcstring)
                    .map(|x| x.to_vec())
                    .ok_or_else(|| Error::UnknownGlyphClass {
                        name: gcstring.to_string(),
                    });
            }
            _ => panic!("It's neither"),
        }
    }

    fn _build_gsub1a(&mut self, builder: &mut Builder, mut feat: Pairs<Rule>) -> Result<()> {
        feat.next();
        let from_glyph = feat.next().unwrap().as_str();
        // validate the glyph here
        feat.next();
        let to_glyph = feat.next().unwrap().as_str();
        // validate the glyph here
        builder.add_single_subst(None, from_glyph, None, to_glyph)
    }

    fn _build_gsub1b(&mut self, builder: &mut Builder, mut feat: Pairs<Rule>) -> Result<()> {
        feat.next();
        let from_glyphs: Vec<String> = self._expand_class(builder, feat.next().unwrap())?;
        feat.next();
        let to_glyph = feat.next().unwrap().as_str();
        for from_glyph in from_glyphs {
            // validate the glyph here
            builder.add_single_subst(None, &from_glyph, None, to_glyph)?;
        }
        Ok(())
    }

    fn _build_ligature_subst(
        &mut self,
        builder: &mut Builder,
        mut feat: Pairs<Rule>,
    ) -> Result<()> {
        feat.next();
        let mut from_glyphs: Vec<&str> = vec![];
        while let Some(Rule::glyphOrClass) = feat.peek().map(|x| x.as_rule()) {
            // validate the glyph here
            from_glyphs.push(feat.next().unwrap().as_str());
        }
        feat.next();
        let to_glyph = feat.next().unwrap().as_str();
        builder.add_ligature_subst(None, from_glyphs, None, to_glyph)
    }

    fn _build_gpos1(&mut self, builder: &mut Builder, mut feat: Pairs<Rule>) -> Result<()> {
        feat.next();
        let mut glyphs: Vec<&str> = vec![];
        while let Some(Rule::glyphOrClass) = feat.peek().map(|x| x.as_rule()) {
            // validate the glyph here
            glyphs.push(feat.next().unwrap().as_str());
        }
        let value_record = self.ot_value_record(builder, feat.next().unwrap())?;
        builder.add_single_pos(None, glyphs, None, value_record)
    }

    fn _build_language(&mut self, builder: &mut Builder, mut feat: Pairs<Rule>) -> Result<()> {
        feat.next();
        let language = feat.next().unwrap().as_str();
        let mut include_default = true;
        let mut required = false;
        while feat.peek().is_some() {
            let token = feat.next().unwrap().as_str();
            if token == "excludeDFLT" || token == "exclude_dflt" {
                include_default = false;
            }
            if token == "required" {
                required = true;
            }
        }
        builder.set_language(language, include_default, required)
    }

    fn _build_script(&mut self, builder: &mut Builder, mut feat: Pairs<Rule>) -> Result<()> {
        feat.next();
        let script = feat.next().unwrap().as_str();
        builder.set_script(script)
    }

    fn ot_value_record(&mut self, builder: &Builder, vr: Pair<Rule>) -> Result<ValueRecord> {
        match vr.as_rule() {
            Rule::valueRecord => self.ot_value_record(builder, vr.into_inner().next().unwrap()),
            Rule::valueLiteral => {
                let mut elements = vr.into_inner().peekable();
                if elements.peek().unwrap().as_rule() == Rule::num_or_varnum {
                    // pos a b 80
                    let x_advance =
                        elements
                            .next()
                            .unwrap()
                            .as_str()
                            .parse::<i16>()
                            .map_err(|_| {
                                Error::InternalError {
                            what:
                                "Thing we parsed as a number couldn't be turned into a Rust number"
                                    .to_string(),
                        }
                            })?;
                    let mut val = ValueRecord::new();
                    val.xAdvance = Some(x_advance);
                    Ok(val)
                } else {
                    unimplemented!()
                }
            }
            Rule::namedValueRecord => {
                unimplemented!()
            }
            _ => panic!(),
        }
    }
}
