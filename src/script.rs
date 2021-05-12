static SCRIPT_NAMES: &[&[&str]] = &[
    &["Adlam"],
    &["Ahom"],
    &["Anatolian Hieroglyphs"],
    &["Arabic"],
    &["Armenian"],
    &["Avestan"],
    &["Balinese"],
    &["Bamum"],
    &["Bassa Vah"],
    &["Batak"],
    &["Bengali"],
    &["Bengali v.2"],
    &["Bhaiksuki"],
    &["Bopomofo"],
    &["Brahmi"],
    &["Braille"],
    &["Buginese"],
    &["Buhid"],
    &["Byzantine Music"],
    &["Canadian Syllabics"],
    &["Carian"],
    &["Caucasian Albanian"],
    &["Chakma"],
    &["Cham"],
    &["Cherokee"],
    &["Chorasmian"],
    &["CJK Ideographic"],
    &["Coptic"],
    &["Cypriot Syllabary"],
    &["Cyrillic"],
    &["Default"],
    &["Deseret"],
    &["Devanagari"],
    &["Devanagari v.2"],
    &["Dives Akuru"],
    &["Dogra"],
    &["Duployan"],
    &["Egyptian Hieroglyphs"],
    &["Elbasan"],
    &["Elymaic"],
    &["Ethiopic"],
    &["Georgian"],
    &["Glagolitic"],
    &["Gothic"],
    &["Grantha"],
    &["Greek"],
    &["Gujarati"],
    &["Gujarati v.2"],
    &["Gunjala Gondi"],
    &["Gurmukhi"],
    &["Gurmukhi v.2"],
    &["Hangul"],
    &["Hangul Jamo"],
    &["Hanifi Rohingya"],
    &["Hanunoo"],
    &["Hatran"],
    &["Hebrew"],
    //&["Hiragana"],
    &["Imperial Aramaic"],
    &["Inscriptional Pahlavi"],
    &["Inscriptional Parthian"],
    &["Javanese"],
    &["Kaithi"],
    &["Kannada"],
    &["Kannada v.2"],
    //&["Katakana"],
    &["Kana", "Hiragana", "Katakana"],
    &["Kayah Li"],
    &["Kharosthi"],
    &["Khitan Small Script"],
    &["Khmer"],
    &["Khojki"],
    &["Khudawadi"],
    &["Lao"],
    &["Latin"],
    &["Lepcha"],
    &["Limbu"],
    &["Linear A"],
    &["Linear B"],
    &["Lisu", "Fraser"],
    &["Lycian"],
    &["Lydian"],
    &["Mahajani"],
    &["Makasar"],
    &["Malayalam"],
    &["Malayalam v.2"],
    &["Mandaic", "Mandaean"],
    &["Manichaean"],
    &["Marchen"],
    &["Masaram Gondi"],
    &["Mathematical Alphanumeric Symbols"],
    &["Medefaidrin", "Oberi Okaime", "Oberi Ɔkaimɛ"],
    &["Meitei Mayek", "Meithei", "Meetei"],
    &["Mende Kikakui"],
    &["Meroitic Cursive"],
    &["Meroitic Hieroglyphs"],
    &["Miao"],
    &["Modi"],
    &["Mongolian"],
    &["Mro"],
    &["Multani"],
    &["Musical Symbols"],
    &["Myanmar"],
    &["Myanmar v.2"],
    &["Nabataean"],
    &["Nandinagari"],
    &["Newa"],
    &["New Tai Lue"],
    &["N'Ko"],
    &["Nüshu"],
    &["Nyiakeng Puachue Hmong"],
    &["Odia"],
    &["Odia v.2"],
    &["Ogham"],
    &["Ol Chiki"],
    &["Old Italic"],
    &["Old Hungarian"],
    &["Old North Arabian"],
    &["Old Permic"],
    &["Old Persian Cuneiform"],
    &["Old Sogdian"],
    &["Old South Arabian"],
    &["Old Turkic", "Orkhon Runic"],
    &["Osage"],
    &["Osmanya"],
    &["Pahawh Hmong"],
    &["Palmyrene"],
    &["Pau Cin Hau"],
    &["Phags-pa"],
    &["Phoenician"],
    &["Psalter Pahlavi"],
    &["Rejang"],
    &["Runic"],
    &["Samaritan"],
    &["Saurashtra"],
    &["Sharada"],
    &["Shavian"],
    &["Siddham"],
    &["Sign Writing"],
    &["Sinhala"],
    &["Sogdian"],
    &["Sora Sompeng"],
    &["Soyombo"],
    &["Sumero-Akkadian Cuneiform"],
    &["Sundanese"],
    &["Syloti Nagri"],
    &["Syriac"],
    &["Tagalog"],
    &["Tagbanwa"],
    &["Tai Le"],
    &["Tai Tham", "Lanna"],
    &["Tai Viet"],
    &["Takri"],
    &["Tamil"],
    &["Tamil v.2"],
    &["Tangut"],
    &["Telugu"],
    &["Telugu v.2"],
    &["Thaana"],
    &["Thai"],
    &["Tibetan"],
    &["Tifinagh"],
    &["Tirhuta"],
    &["Ugaritic Cuneiform"],
    &["Vai"],
    &["Wancho"],
    &["Warang Citi"],
    &["Yezidi"],
    &["Yi"],
    &[
        "Zanabazar Square",
        "Zanabazarin Dörböljin Useg",
        "Xewtee Dörböljin Bicig",
        "Horizontal Square Script",
    ],
];

static SCRIPT_ISO: &[&[&str]] = &[
    &["Adlm"],
    &["Ahom"],
    &["Hluw"],
    &["Arab"],
    &["Armn"],
    &["Avst"],
    &["Bali"],
    &["Bamu"],
    &["Bass"],
    &["Batk"],
    &["Beng"],
    &["Beng"],
    &["Bhks"],
    &["Bopo"],
    &["Brah"],
    &["Brai"],
    &["Bugi"],
    &["Buhd"],
    &["Zzzz"],
    &["Cans"],
    &["Cari"],
    &["Aghb"],
    &["Cakm"],
    &["Cham"],
    &["Cher"],
    &["Chrs"],
    &["Hani"],
    &["Copt"],
    &["Cprt"],
    &["Cyrl"],
    &["Zyyy"],
    &["Dsrt"],
    &["Deva"],
    &["Deva"],
    &["Diak"],
    &["Dogr"],
    &["Dupl"],
    &["Egyp"],
    &["Elba"],
    &["Elym"],
    &["Ethi"],
    &["Geok", "Geor"],
    &["Glag"],
    &["Goth"],
    &["Gran"],
    &["Grek"],
    &["Gujr"],
    &["Gujr"],
    &["Gong"],
    &["Guru"],
    &["Guru"],
    &["Hang"],
    &["Hang"],
    &["Rohg"],
    &["Hano"],
    &["Hatr"],
    &["Hebr"],
    &["Armi"],
    &["Phli"],
    &["Prti"],
    &["Java"],
    &["Kthi"],
    &["Knda"],
    &["Knda"],
    &["Hrkt", "Hira", "Kana"],
    &["Kali"],
    &["Khar"],
    &["Kits"],
    &["Khmr"],
    &["Khoj"],
    &["Sind"],
    &["Laoo"],
    &["Latn", "Latf", "Latg"],
    &["Lepc"],
    &["Limb"],
    &["Lina"],
    &["Linb"],
    &["Lisu"],
    &["Lyci"],
    &["Lydi"],
    &["Mahj"],
    &["Maka"],
    &["Mlym"],
    &["Mlym"],
    &["Mand"],
    &["Mani"],
    &["Marc"],
    &["Gonm"],
    &["Zmth"],
    &["Medf"],
    &["Mtei"],
    &["Mend"],
    &["Merc"],
    &["Mero"],
    &["Plrd"],
    &["Modi"],
    &["Mong"],
    &["Mroo"],
    &["Mult"],
    &["Zsym"],
    &["Mymr"],
    &["Mymr"],
    &["Nbat"],
    &["Nand"],
    &["Newa"],
    &["Talu"],
    &["Nkoo"],
    &["Nshu"],
    &["Hmnp"],
    &["Orya"],
    &["Orya"],
    &["Ogam"],
    &["Olck"],
    &["Ital"],
    &["Hung"],
    &["Narb"],
    &["Perm"],
    &["Xpeo"],
    &["Sogo"],
    &["Sarb"],
    &["Orkh"],
    &["Osge"],
    &["Osma"],
    &["Hmng"],
    &["Palm"],
    &["Pauc"],
    &["Phag"],
    &["Phnx"],
    &["Phlp"],
    &["Rjng"],
    &["Runr"],
    &["Samr"],
    &["Saur"],
    &["Shrd"],
    &["Shaw"],
    &["Sidd"],
    &["Sgnw"],
    &["Sinh"],
    &["Sogd"],
    &["Sora"],
    &["Soyo"],
    &["Xsux"],
    &["Sund"],
    &["Sylo"],
    &["Syrc"],
    &["Tglg"],
    &["Tagb"],
    &["Tale"],
    &["Lana"],
    &["Tavt"],
    &["Takr"],
    &["Taml"],
    &["Taml"],
    &["Tang"],
    &["Telu"],
    &["Telu"],
    &["Thaa"],
    &["Thai"],
    &["Tibt"],
    &["Tfng"],
    &["Tirh"],
    &["Ugar"],
    &["Vaii"],
    &["Wcho"],
    &["Wara"],
    &["Yezi"],
    &["Yiii"],
    &["Zanb"],
];

/// An OpenType script identifier.
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(unused, non_camel_case_types)]
pub enum Script {
    adlm,
    ahom,
    hluw,
    arab,
    armn,
    avst,
    bali,
    bamu,
    bass,
    batk,
    beng,
    bng2,
    bhks,
    bopo,
    brah,
    brai,
    bugi,
    buhd,
    byzm,
    cans,
    cari,
    aghb,
    cakm,
    cham,
    cher,
    chrs,
    hani,
    copt,
    cprt,
    cyrl,
    DFLT,
    dsrt,
    deva,
    dev2,
    diak,
    dogr,
    dupl,
    egyp,
    elba,
    elym,
    ethi,
    geor,
    glag,
    goth,
    gran,
    grek,
    gujr,
    gjr2,
    gong,
    guru,
    gur2,
    hang,
    jamo,
    rohg,
    hano,
    hatr,
    hebr,
    //kana,
    armi,
    phli,
    prti,
    java,
    kthi,
    knda,
    knd2,
    kana,
    kali,
    khar,
    kits,
    khmr,
    khoj,
    sind,
    lao,
    latn,
    lepc,
    limb,
    lina,
    linb,
    lisu,
    lyci,
    lydi,
    mahj,
    maka,
    mlym,
    mlm2,
    mand,
    mani,
    marc,
    gonm,
    math,
    medf,
    mtei,
    mend,
    merc,
    mero,
    plrd,
    modi,
    mong,
    mroo,
    mult,
    musc,
    mymr,
    mym2,
    nbat,
    nand,
    newa,
    talu,
    nko,
    nshu,
    hmnp,
    orya,
    ory2,
    ogam,
    olck,
    ital,
    hung,
    narb,
    perm,
    xpeo,
    sogo,
    sarb,
    orkh,
    osge,
    osma,
    hmng,
    palm,
    pauc,
    phag,
    phnx,
    phlp,
    rjng,
    runr,
    samr,
    saur,
    shrd,
    shaw,
    sidd,
    sgnw,
    sinh,
    sogd,
    sora,
    soyo,
    xsux,
    sund,
    sylo,
    syrc,
    tglg,
    tagb,
    tale,
    lana,
    tavt,
    takr,
    taml,
    tml2,
    tang,
    telu,
    tel2,
    thaa,
    thai,
    tibt,
    tfng,
    tirh,
    ugar,
    vai,
    wcho,
    wara,
    yezi,
    yi,
    zanb,
}

/// Whether the OpenType standard defines a shaping engine for this script which will happen in
/// conjunction with any font-defined OpenType Layout rules.
pub enum HasShapingEngine {
    /// Standard OpenType Layout only
    No,
    /// USE rules for complex scripts
    /// (see https://docs.microsoft.com/en-us/typography/script-development/use)
    UniversalShapingEngine,
    /// One of the OpenType Indic shaping engines
    /// (see e.g. https://docs.microsoft.com/en-us/typography/script-development/malayalam)
    IndicShapingEngine,
    /// An OpenType script-specific shaping engine that isn't categorizable
    /// (see table of contents @ https://docs.microsoft.com/en-us/typography/script-development/standard)
    OtherScriptShapingEngine,
}

impl Script {
    /// Whether or not this is the default language
    pub fn is_default(&self) -> bool {
        *self == Script::DFLT
    }

    /// Determine the shaping engine type for the script tag.
    pub fn has_shaping_engine(&self) -> HasShapingEngine {
        if *self == Script::bng2
            || *self == Script::dev2
            || *self == Script::gjr2
            || *self == Script::gur2
            || *self == Script::knd2
            || *self == Script::mlm2
            || *self == Script::ory2
            || *self == Script::tml2
            || *self == Script::tel2
        {
            HasShapingEngine::IndicShapingEngine
        } else if *self == Script::arab
            || *self == Script::bugi
            || *self == Script::hang
            || *self == Script::hebr
            || *self == Script::java
            || *self == Script::khmr
            || *self == Script::lao
            || *self == Script::mymr
            || *self == Script::sinh
            || *self == Script::syrc
            || *self == Script::thaa
            || *self == Script::thai
            || *self == Script::tibt
        {
            HasShapingEngine::OtherScriptShapingEngine
        } else if *self == Script::adlm
            || *self == Script::ahom
            || *self == Script::bhks
            || *self == Script::bali
            || *self == Script::batk
            || *self == Script::brah
            || *self == Script::bugi
            || *self == Script::buhd
            || *self == Script::cakm
            || *self == Script::cham
            || *self == Script::chrs
            || *self == Script::diak
            || *self == Script::dogr
            || *self == Script::dupl
            || *self == Script::egyp
            || *self == Script::elym
            || *self == Script::gran
            || *self == Script::gong
            || *self == Script::rohg
            || *self == Script::hano
            || *self == Script::java
            || *self == Script::kthi
            || *self == Script::kali
            || *self == Script::khar
            || *self == Script::kits
            || *self == Script::khoj
            || *self == Script::sind
            || *self == Script::lepc
            || *self == Script::limb
            || *self == Script::mahj
            || *self == Script::maka
            || *self == Script::mand
            || *self == Script::mani
            || *self == Script::marc
            || *self == Script::gonm
            || *self == Script::medf
            || *self == Script::mtei
            || *self == Script::plrd
            || *self == Script::modi
            || *self == Script::mong
            || *self == Script::mult
            || *self == Script::nand
            || *self == Script::newa
            || *self == Script::nko
            || *self == Script::hmnp
            || *self == Script::sogo
            || *self == Script::hmng
            || *self == Script::phag
            || *self == Script::phlp
            || *self == Script::rjng
            || *self == Script::saur
            || *self == Script::shrd
            || *self == Script::sidd
            || *self == Script::sinh
            || *self == Script::sogd
            || *self == Script::soyo
            || *self == Script::sund
            || *self == Script::sylo
            || *self == Script::tglg
            || *self == Script::tagb
            || *self == Script::tale
            || *self == Script::lana
            || *self == Script::tavt
            || *self == Script::takr
            || *self == Script::tibt
            || *self == Script::tfng
            || *self == Script::tirh
            || *self == Script::wcho
            || *self == Script::yezi
            || *self == Script::zanb
        {
            HasShapingEngine::UniversalShapingEngine
        } else {
            HasShapingEngine::No
        }
    }

    /// Whether or not this script tag is a v2 version of another script tag
    pub fn is_v2(&self) -> bool {
        self.to_otl_tag() != *self
    }

    /// Whether or not this script tag has a v2 script tag
    pub fn has_v2(&self) -> bool {
        self.to_v2_tag() != *self
    }

    /// If script has an OpenType script-specific shaping engine, return tag for it.
    pub fn to_v2_tag(self) -> Self {
        match self {
            Script::beng => Script::bng2,
            Script::deva => Script::dev2,
            Script::gujr => Script::gjr2,
            Script::guru => Script::gur2,
            Script::knda => Script::knd2,
            Script::mlym => Script::mlm2,
            Script::mymr => Script::mym2,
            Script::orya => Script::ory2,
            Script::taml => Script::tml2,
            Script::telu => Script::tel2,
            _ => self,
        }
    }

    /// If script has an OpenType Layout (non Indic shaping engine) script tag, return it.
    pub fn to_otl_tag(self) -> Self {
        match self {
            Script::bng2 => Script::beng,
            Script::dev2 => Script::deva,
            Script::gjr2 => Script::gujr,
            Script::gur2 => Script::guru,
            Script::knd2 => Script::knda,
            Script::mlm2 => Script::mlym,
            Script::mym2 => Script::mymr,
            Script::ory2 => Script::orya,
            Script::tml2 => Script::taml,
            Script::tel2 => Script::telu,
            _ => self,
        }
    }

    /// All the names associated with this script tag
    pub fn names(&self) -> &'static [&'static str] {
        SCRIPT_NAMES[*self as usize]
    }

    /// The most common name associated with this script tag
    pub fn name(&self) -> &'static str {
        SCRIPT_NAMES[*self as usize][0]
    }

    pub fn iso15924_ids(&self) -> &'static [&'static str] {
        SCRIPT_ISO[*self as usize]
    }

    pub fn iso15924_id(&self) -> &'static str {
        SCRIPT_ISO[*self as usize][0]
    }
}

#[cfg(test)]
mod tests {
    use crate::script::Script;

    #[test]
    fn test_scripts() {
        assert!(Script::beng.has_v2());
        assert!(Script::ory2.is_v2());
        assert!(!Script::kana.has_v2() && !Script::kana.is_v2());
        assert_eq!(Script::mlym.to_v2_tag(), Script::mlm2);
        assert_eq!(Script::mlm2.to_v2_tag(), Script::mlm2);
        assert_eq!(Script::mlm2.to_otl_tag(), Script::mlym);
        assert_eq!(Script::kana.name(), "Kana");
        assert_eq!(Script::kana.names()[2], "Katakana");
        assert_eq!(Script::kana.iso15924_ids(), ["Hrkt", "Hira", "Kana"]);
        assert_eq!(Script::zanb.iso15924_id(), "Zanb");
        assert_eq!(Script::ahom.iso15924_id(), "Ahom");
        assert_eq!(Script::nshu.name(), "Nüshu");
        assert!(Script::DFLT.is_default());
    }
}
