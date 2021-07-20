use crate::builder::Builder;
use crate::Rule;
use pest::iterators::Pairs;
use std::collections::HashMap;

macro_rules! rule_todo {
    ($pair:ident) => {
        panic!("Rule type {:?} not implemented", $pair.as_rule())
    };
}

pub struct FeatureAST<'a>(pub Option<Pairs<'a, Rule>>);

impl FeatureAST<'_> {
    pub fn build(&mut self, builder: &mut Builder) {
        if let Some(pairs) = self.0.take() {
            self.build_items(builder, pairs);
        }
    }

    fn build_items(&mut self, builder: &mut Builder, pairs: Pairs<Rule>) {
        for pair in pairs {
            match pair.as_rule() {
                Rule::feature_statement => {
                    self._build_feature_statement(builder, pair.into_inner())
                }
                Rule::anchor_coordinates => rule_todo!(pair),
                Rule::anchor_name => rule_todo!(pair),
                Rule::anchordef_statement => rule_todo!(pair),
                Rule::anchordef_tag => rule_todo!(pair),
                Rule::anonymous_inner => rule_todo!(pair),
                Rule::anonymous_inner_statement => rule_todo!(pair),
                Rule::anonymous_quoted_string => rule_todo!(pair),
                Rule::anonymous_statement => rule_todo!(pair),
                Rule::anonymous_word => rule_todo!(pair),
                Rule::apply_by => rule_todo!(pair),
                Rule::apply_lookup => rule_todo!(pair),
                Rule::attach_gdef_statement => rule_todo!(pair),
                Rule::barename => rule_todo!(pair),
                Rule::block => rule_todo!(pair),
                Rule::by_tag => rule_todo!(pair),
                Rule::class => rule_todo!(pair),
                Rule::class_definition_statement => rule_todo!(pair),
                Rule::class_inner => rule_todo!(pair),
                Rule::class_name => rule_todo!(pair),
                Rule::comment_inner => rule_todo!(pair),
                Rule::contourpoint => rule_todo!(pair),
                Rule::contourpoint_tag => rule_todo!(pair),
                Rule::default_language => rule_todo!(pair),
                Rule::default_script => rule_todo!(pair),
                Rule::dflt_tag => rule_todo!(pair),
                Rule::end_tag => {}
                Rule::fea_decimal_integer => rule_todo!(pair),
                Rule::fea_hex_integer => rule_todo!(pair),
                Rule::fea_integer => rule_todo!(pair),
                Rule::feature_names_statement => rule_todo!(pair),
                Rule::file => rule_todo!(pair),
                Rule::font_revision_statement => rule_todo!(pair),
                Rule::from_tag => rule_todo!(pair),
                Rule::gdef_block => rule_todo!(pair),
                Rule::gdef_statement => rule_todo!(pair),
                Rule::glyph_or_class => rule_todo!(pair),
                Rule::glyphclassdef_gdef_statement => rule_todo!(pair),
                Rule::gsub_1a => rule_todo!(pair),
                Rule::gsub_1b => rule_todo!(pair),
                Rule::gsub_1c => rule_todo!(pair),
                Rule::gsub_2a => rule_todo!(pair),
                Rule::gsub_2b => rule_todo!(pair),
                Rule::gsub_3 => rule_todo!(pair),
                Rule::gsub_4 => self._build_ligature_subst(builder, pair.into_inner()),
                Rule::gsub_6 => rule_todo!(pair),
                Rule::gsub_glyph_seq => rule_todo!(pair),
                Rule::gsub_statement => self.build_items(builder, pair.into_inner()),
                Rule::ignore_tag => rule_todo!(pair),
                Rule::include_inner => rule_todo!(pair),
                Rule::include_statement => rule_todo!(pair),
                Rule::include_string_char => rule_todo!(pair),
                Rule::inline_class => rule_todo!(pair),
                Rule::language => rule_todo!(pair),
                Rule::language_statement => rule_todo!(pair),
                Rule::languagesystem_statement => rule_todo!(pair),
                Rule::ligaturecaretbyindex_gdef_statement => rule_todo!(pair),
                Rule::ligaturecaretbypos_gdef_statement => rule_todo!(pair),
                Rule::lookup_name => rule_todo!(pair),
                Rule::lookup_statement => rule_todo!(pair),
                Rule::lookupflag_complex => rule_todo!(pair),
                Rule::lookupflag_complex_tag => rule_todo!(pair),
                Rule::lookupflag_simple => rule_todo!(pair),
                Rule::lookupflag_statement => rule_todo!(pair),
                Rule::mid_glyph_name => rule_todo!(pair),
                Rule::name_inner => rule_todo!(pair),
                Rule::name_statement => rule_todo!(pair),
                Rule::nameid_statement => rule_todo!(pair),
                Rule::null_token => {}
                Rule::os2_block => rule_todo!(pair),
                Rule::os2_panose => rule_todo!(pair),
                Rule::os2_range => rule_todo!(pair),
                Rule::os2_range_tag => rule_todo!(pair),
                Rule::os2_single_number => rule_todo!(pair),
                Rule::os2_single_number_tag => rule_todo!(pair),
                Rule::os2_statement => rule_todo!(pair),
                Rule::os2_vendor => rule_todo!(pair),
                Rule::os2_vendor_id => rule_todo!(pair),
                Rule::qs_char => rule_todo!(pair),
                Rule::qs_inner => rule_todo!(pair),
                Rule::quoted_string => rule_todo!(pair),
                Rule::required_tag => rule_todo!(pair),
                Rule::script => rule_todo!(pair),
                Rule::script_statement => rule_todo!(pair),
                Rule::start_glyph_name => rule_todo!(pair),
                Rule::start_tag => rule_todo!(pair),
                Rule::statement => rule_todo!(pair),
                Rule::sub_tag => rule_todo!(pair),
                Rule::subtable_statement => rule_todo!(pair),
                Rule::subtitute_tag => rule_todo!(pair),
                Rule::table_gdef_statement => rule_todo!(pair),
                Rule::table_head_statement => rule_todo!(pair),
                Rule::table_name_statement => rule_todo!(pair),
                Rule::table_os2_statement => rule_todo!(pair),
                Rule::table_statement => rule_todo!(pair),
                Rule::tag => rule_todo!(pair),
                Rule::tagged_block => rule_todo!(pair),
                Rule::use_extension => rule_todo!(pair),
                Rule::valuerecord => rule_todo!(pair),
                Rule::valuerecord1 => rule_todo!(pair),
                Rule::valuerecord4 => rule_todo!(pair),
                Rule::valuerecord_name => rule_todo!(pair),
                Rule::valuerecorddef_statement => rule_todo!(pair),
                Rule::valuerecorddef_tag => rule_todo!(pair),
                Rule::variation_block => rule_todo!(pair),
                Rule::variation_lookup_statement => rule_todo!(pair),
                Rule::variation_name => rule_todo!(pair),
                Rule::variation_statement => rule_todo!(pair),
                Rule::variation_tag => rule_todo!(pair),
                Rule::version => rule_todo!(pair),
                _ => {}
            }
        }
    }

    pub fn _build_feature_statement(&mut self, builder: &mut Builder, mut feat: Pairs<Rule>) {
        let name = feat.next().unwrap().as_str();
        builder.start_feature(name);
        let mut features = std::mem::take(&mut builder.features);
        self.build_items(builder, feat);
        std::mem::swap(&mut builder.features, &mut features);
        builder.features.extend(features);
        builder.end_feature();
    }

    pub fn _build_ligature_subst(&mut self, builder: &mut Builder, mut feat: Pairs<Rule>) {
        feat.next();
        let from_glyphs: Vec<&str> = feat.next().unwrap().as_str().split_whitespace().collect();
        feat.next();
        let to_glyph = feat.next().unwrap().as_str();
        builder.add_ligature_subst(from_glyphs, to_glyph);
    }
}
