use crate::FeatureAST;
use fonttools::layout::common::Lookup;
use fonttools::layout::common::LookupFlags;
use fonttools::GPOS::GPOS;
use fonttools::GSUB::GSUB;
use std::collections::HashMap;
use std::collections::HashSet;

pub enum SomeLookup {
    GsubLookup(Lookup<GSUB>),
    GposLookup(Lookup<GPOS>),
}

pub struct Builder {
    pub glyph_map: HashMap<String, u16>,
    pub glyph_classes: HashMap<String, Vec<String>>,
    pub lookup_flag: LookupFlags,
    pub cur_mark_filter_set: Option<u16>,
    pub default_language_systems: HashSet<(String, String)>,
    pub cur_lookup: Option<SomeLookup>,
    pub cur_feature_name: Option<String>,
    pub cur_lookup_name: Option<String>,
    pub cur_language_systems: HashSet<(String, String)>,
    pub named_lookups: HashMap<String, SomeLookup>,
    pub script: Option<String>,
    pub features: HashMap<String, Vec<SomeLookup>>,
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            glyph_map: HashMap::new(),
            glyph_classes: HashMap::new(),
            lookup_flag: LookupFlags::empty(),
            cur_mark_filter_set: None,
            default_language_systems: HashSet::new(),
            cur_lookup: None,
            cur_feature_name: None,
            cur_lookup_name: None,
            cur_language_systems: HashSet::new(),
            named_lookups: HashMap::new(),
            features: HashMap::new(),
            script: None,
        }
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

    pub fn add_ligature_subst(&mut self, from_glyphs: Vec<&str>, to_glyphs: &str) {
        // I think we're gonna need a Glyph type
        // println!("Sub from {:?} to {:?}", from_glyphs, to_glyphs);
    }
}
