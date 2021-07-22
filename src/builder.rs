use crate::error::{Error, Result};
use crate::FeatureAST;
use fonttools::layout::common::Lookup;
use fonttools::layout::common::LookupFlags;
use fonttools::layout::gsub1::SingleSubst;
use fonttools::layout::gsub2::MultipleSubst;
use fonttools::layout::gsub3::AlternateSubst;
use fonttools::layout::gsub4::LigatureSubst;
use fonttools::GPOS::Positioning;
use fonttools::GSUB::Substitution;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
pub enum SomeLookup {
    GsubLookup(Lookup<Substitution>),
    GposLookup(Lookup<Positioning>),
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct FeatureKey {
    script: String,
    lang: String,
    feature_name: String,
}

#[derive(Default, Debug)]
pub struct Builder {
    pub glyph_map: HashMap<String, u16>,
    pub glyph_classes: HashMap<String, Vec<String>>,
    pub lookup_flag: LookupFlags,
    pub lookups: Vec<SomeLookup>,
    pub cur_mark_filter_set: Option<u16>,
    pub default_language_systems: HashSet<(String, String)>,
    pub cur_lookup: Option<usize>,
    pub cur_feature_name: Option<String>,
    pub cur_lookup_name: Option<String>,
    pub cur_language_systems: HashSet<(String, String)>,
    pub named_lookups: HashMap<String, usize>,
    pub script: Option<String>,
    pub features: HashMap<FeatureKey, Vec<usize>>,
}

impl Builder {
    pub fn new(glyph_map: HashMap<String, u16>) -> Self {
        let mut n = Builder::default();
        n.glyph_map = glyph_map;
        n
    }
    pub fn build(&mut self, mut parsetree: FeatureAST) {
        let res = parsetree.build(self);
    }

    fn get_default_language_systems(&self) -> HashSet<(String, String)> {
        if !self.default_language_systems.is_empty() {
            return self.default_language_systems.clone();
        }
        let mut dflt = HashSet::new();
        dflt.insert(("DFLT".to_string(), "dflt".to_string()));
        dflt
    }

    pub fn start_feature(&mut self, feature_name: &str) {
        self.cur_language_systems = self.get_default_language_systems();
        self.script = Some("DFLT".to_string());
        self.cur_lookup = None;
        self.cur_feature_name = Some(feature_name.to_string());
        self.lookup_flag = LookupFlags::empty();
        self.cur_mark_filter_set = None;
    }

    pub fn end_feature(&mut self) {
        self.cur_feature_name = None;
        self.cur_language_systems = HashSet::new();
        self.cur_lookup = None;
        self.lookup_flag = LookupFlags::empty();
        self.cur_mark_filter_set = None;
    }

    fn ensure_sub_lookup(&mut self, lookup_type: u16) -> Result<usize, String> {
        if let Some(ix) = self.cur_lookup {
            if let Some(SomeLookup::GsubLookup(lu)) = self.lookups.get(ix) {
                if lu.lookup_type() == lookup_type {
                    return Ok(ix);
                } else if self.cur_lookup_name.is_some() {
                    return Err(
                        "Within a named lookup, all rules must of the same lookup type and flag"
                            .to_string(),
                    );
                }
            }
        }
        self.lookups.push(SomeLookup::GsubLookup(Lookup {
            flags: self.lookup_flag,
            rule: match lookup_type {
                1 => Substitution::Single(vec![SingleSubst::default()]),
                2 => Substitution::Multiple(vec![MultipleSubst::default()]),
                3 => Substitution::Alternate(vec![AlternateSubst::default()]),
                4 => Substitution::Ligature(vec![LigatureSubst::default()]),
                5 => Substitution::Contextual,
                6 => Substitution::ChainedContextual,
                7 => Substitution::Extension,
                8 => Substitution::ReverseChaining,
                _ => panic!("No such lookup type"),
            },
            mark_filtering_set: self.cur_mark_filter_set,
        }));
        let ix = self.lookups.len() - 1;
        self.cur_lookup = Some(ix);
        if let Some(name) = &self.cur_lookup_name {
            self.named_lookups.insert(name.to_string(), ix);
        }

        if let Some(feature) = &self.cur_feature_name {
            self.add_lookup_to_feature(ix, feature.to_string())
        }
        Ok(ix)
    }

    fn add_lookup_to_feature(&mut self, ix: usize, feature_name: String) {
        for (script, lang) in &self.cur_language_systems {
            let featurekey = FeatureKey {
                script: script.to_string(),
                lang: lang.to_string(),
                feature_name: feature_name.clone(),
            };
            self.features
                .entry(featurekey)
                .or_insert_with(Vec::new)
                .push(ix);
        }
    }

    pub fn add_ligature_subst(
        &mut self,
        _prefix: Option<Vec<&str>>,
        from_glyphs: Vec<&str>,
        _suffix: Option<Vec<&str>>,
        to_glyph: &str,
    ) -> Result<()> {
        println!("Sub from {:?} to {:?}", from_glyphs, to_glyph);
        let from_gids = self.glyph_ids(from_glyphs)?;
        let to_gid = self.glyph_id(to_glyph)?;
        let lookup_ix = self.ensure_sub_lookup(4).expect("Boo");
        let lookup = self.lookups.get_mut(lookup_ix).unwrap();
        // I think we're gonna need a Glyph type
        if let SomeLookup::GsubLookup(lu) = lookup {
            if let Substitution::Ligature(ligs) = &mut lu.rule {
                let subtable = ligs.last_mut().unwrap();
                subtable.mapping.insert(from_gids, to_gid);
            }
        }
        Ok(())
    }

    pub fn glyph_ids(&self, names: Vec<&str>) -> Result<Vec<u16>> {
        names.iter().map(|n| self.glyph_id(n)).collect()
    }

    pub fn glyph_id(&self, name: &str) -> Result<u16> {
        self.glyph_map
            .get(name)
            .copied()
            .ok_or_else(|| Error::UnknownGlyph {
                name: name.to_string(),
            })
    }
}
