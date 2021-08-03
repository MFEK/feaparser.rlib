use crate::error::{Error, Result};
use crate::FeatureAST;
use fonttools::font::Font;
use fonttools::font::Table;
use fonttools::layout::common::{LanguageSystem, Lookup, LookupFlags, Script};
use fonttools::layout::gsub1::SingleSubst;
use fonttools::layout::gsub2::MultipleSubst;
use fonttools::layout::gsub3::AlternateSubst;
use fonttools::layout::gsub4::LigatureSubst;
use fonttools::types::Tag;
use fonttools::GPOS::{Positioning, GPOS};
use fonttools::GSUB::{Substitution, GSUB};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;

fn tag(s: &str) -> Tag {
    (*s).as_bytes().try_into().unwrap()
}

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
impl PartialOrd for FeatureKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let one = self.feature_name.cmp(&other.feature_name);
        if one != Ordering::Equal {
            return Some(one);
        }
        let two = self.lang.cmp(&other.lang);
        if two != Ordering::Equal {
            return Some(two);
        }
        Some(self.script.cmp(&other.script))
    }
}

impl Ord for FeatureKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
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
        Builder {
            glyph_map,
            ..Default::default()
        }
    }
    pub fn build(&mut self, mut parsetree: FeatureAST, font: &mut Font) -> Result<()> {
        parsetree.build(self)?;
        // Build aalt
        let gsubgpos = self.make_gsubgpos();
        if let Some(gsub) = gsubgpos.0 {
            font.tables.insert(*b"GSUB", Table::GSUB(gsub));
        } else {
            font.tables.remove(b"GSUB");
        }

        if let Some(gpos) = gsubgpos.1 {
            font.tables.insert(*b"GPOS", Table::GPOS(gpos));
        } else {
            font.tables.remove(b"GPOS");
        }
        Ok(())
        // recalc os2 max context
    }

    fn make_gsubgpos(&mut self) -> (Option<GSUB>, Option<GPOS>) {
        // Partition lookup list by type
        let mut original_id_to_gpos: HashMap<usize, usize> = HashMap::new();
        let mut original_id_to_gsub: HashMap<usize, usize> = HashMap::new();
        let mut gsub = GSUB::default();
        let mut gpos = GPOS::default();
        for (orig_index, lookup) in self.lookups.drain(0..).enumerate() {
            match lookup {
                SomeLookup::GsubLookup(x) => {
                    original_id_to_gsub.insert(orig_index, gsub.lookups.len());
                    gsub.lookups.push(x);
                }
                SomeLookup::GposLookup(x) => {
                    original_id_to_gpos.insert(orig_index, gpos.lookups.len());
                    gpos.lookups.push(x);
                }
            }
        }

        let mut gsub_features: HashMap<(&str, Vec<usize>), usize> = HashMap::new();
        let mut gpos_features: HashMap<(&str, Vec<usize>), usize> = HashMap::new();

        // { 'latn': {'DEU': [23, 24]} } for feature #23,24
        let mut gsub_scripts: HashMap<&str, HashMap<&str, Vec<usize>>> = HashMap::new();
        let mut gpos_scripts: HashMap<&str, HashMap<&str, Vec<usize>>> = HashMap::new();
        for key in self.features.keys().sorted() {
            let feature_indices = self.features.get(key).unwrap();
            let gsub_indices: Vec<usize> = feature_indices
                .iter()
                .map(|x| original_id_to_gsub.get(x))
                .flatten()
                .copied()
                .collect();
            if !gsub_indices.is_empty() {
                let gsub_feature_key = (key.feature_name.as_str(), gsub_indices.clone());
                let gsub_feature_index =
                    gsub_features.entry(gsub_feature_key).or_insert_with(|| {
                        gsub.features.push((
                            tag(key.feature_name.as_str()),
                            gsub_indices,
                            None, // feature parameters
                        ));
                        gsub.features.len() - 1
                    });
                gsub_scripts
                    .entry(key.feature_name.as_str())
                    .or_insert_with(HashMap::new)
                    .entry(key.lang.as_str())
                    .or_insert_with(Vec::new)
                    .push(*gsub_feature_index);
            }

            let gpos_indices: Vec<usize> = feature_indices
                .iter()
                .map(|x| original_id_to_gpos.get(x))
                .flatten()
                .copied()
                .collect();
            if !gpos_indices.is_empty() {
                let gpos_feature_key = (key.feature_name.as_str(), gpos_indices.clone());
                let gpos_feature_index =
                    gpos_features.entry(gpos_feature_key).or_insert_with(|| {
                        gpos.features.push((
                            tag(key.feature_name.as_str()),
                            gpos_indices,
                            None, // feature parameters
                        ));
                        gpos.features.len() - 1
                    });
                gpos_scripts
                    .entry(key.feature_name.as_str())
                    .or_insert_with(HashMap::new)
                    .entry(key.lang.as_str())
                    .or_insert_with(Vec::new)
                    .push(*gpos_feature_index);
            }
        }
        for script_tag in gsub_scripts.keys() {
            let entry = gsub_scripts.get(script_tag).unwrap();
            let mut script_record = Script::default();
            for lang in entry.keys() {
                let feature_indices = entry.get(lang).unwrap();
                let ls = LanguageSystem {
                    required_feature: None, // XXX
                    feature_indices: feature_indices.to_vec(),
                };
                if lang == &"dflt" {
                    script_record.default_language_system = Some(ls);
                } else {
                    script_record.language_systems.insert(tag(lang), ls);
                }
            }
            gsub.scripts.scripts.insert(tag(script_tag), script_record);
        }

        for script_tag in gpos_scripts.keys() {
            let entry = gpos_scripts.get(script_tag).unwrap();
            let mut script_record = Script::default();
            for lang in entry.keys() {
                let feature_indices = entry.get(lang).unwrap();
                let ls = LanguageSystem {
                    required_feature: None, // XXX
                    feature_indices: feature_indices.to_vec(),
                };
                if lang == &"dflt" {
                    script_record.default_language_system = Some(ls);
                } else {
                    script_record.language_systems.insert(tag(lang), ls);
                }
            }
            gpos.scripts.scripts.insert(tag(script_tag), script_record);
        }

        (
            if gsub.lookups.is_empty() && gsub.features.is_empty() {
                None
            } else {
                Some(gsub)
            },
            if gpos.lookups.is_empty() && gpos.features.is_empty() {
                None
            } else {
                Some(gpos)
            },
        )
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

    fn ensure_sub_lookup(&mut self, lookup_type: u16) -> Result<usize> {
        if let Some(ix) = self.cur_lookup {
            if let Some(SomeLookup::GsubLookup(lu)) = self.lookups.get(ix) {
                if lu.lookup_type() == lookup_type {
                    return Ok(ix);
                } else if self.cur_lookup_name.is_some() {
                    return Err(Error::DisparateRulesInLookup);
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
        let lookup_ix = self.ensure_sub_lookup(4)?;
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
