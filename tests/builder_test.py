from fontTools.misc.loggingTools import CapturingLogHandler
from fontTools.feaLib.builder import Builder
from fontTools.feaLib.error import FeatureLibError
from fontTools.ttLib import TTFont, newTable
from fontTools.feaLib.parser import Parser
from fontTools.feaLib import ast
from fontTools.feaLib.lexer import Lexer
import difflib
from io import StringIO
import os
import re
import shutil
import sys
import tempfile
import logging
import unittest
import subprocess

def make_post(font):
    post = font["post"] = newTable("post")
    post.formatType = 2.0
    post.italicAngle = post.underlinePosition = post.underlinePosition = 0
    post.underlineThickness=0
    post.isFixedPitch=0
    post.minMemType42=0
    post.maxMemType42=0
    post.minMemType1=0
    post.maxMemType1=0
    post.extraNames=[]
    post.mapping={}
    maxp = font["maxp"] = newTable("maxp")
    maxp.tableVersion=0x00005000
    maxp.numGlyphs = len(font.getGlyphOrder())

def addOpenTypeFeaturesFromString(font, featureString, tables=None):
    _, font_fname = tempfile.mkstemp(suffix=".ttf")
    _, fea_fname = tempfile.mkstemp(suffix=".fea")
    make_post(font)
    font.save(font_fname)
    fea_fh = open(fea_fname, "w")
    fea_fh.write(featureString)
    fea_fh.close()
    print(font_fname, fea_fname, tables)
    subprocess.run(["./target/debug/featiculus", "-o", font_fname, font_fname, fea_fname])
    return TTFont("out-"+font_fname)


def addOpenTypeFeatures(font, fea_fname, tables=None):
    _, font_fname = tempfile.mkstemp(suffix=".ttf")
    make_post(font)
    font.save(font_fname)
    print(font_fname, fea_fname, tables)
    subprocess.run(["./target/debug/featiculus", "-o", font_fname+".out", font_fname, fea_fname])
    return TTFont(font_fname+".out")

def makeTTFont():
    glyphs = """
        .notdef space slash fraction semicolon period comma ampersand
        quotedblleft quotedblright quoteleft quoteright
        zero one two three four five six seven eight nine
        zero.oldstyle one.oldstyle two.oldstyle three.oldstyle
        four.oldstyle five.oldstyle six.oldstyle seven.oldstyle
        eight.oldstyle nine.oldstyle onequarter onehalf threequarters
        onesuperior twosuperior threesuperior ordfeminine ordmasculine
        A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
        a b c d e f g h i j k l m n o p q r s t u v w x y z
        A.sc B.sc C.sc D.sc E.sc F.sc G.sc H.sc I.sc J.sc K.sc L.sc M.sc
        N.sc O.sc P.sc Q.sc R.sc S.sc T.sc U.sc V.sc W.sc X.sc Y.sc Z.sc
        A.alt1 A.alt2 A.alt3 B.alt1 B.alt2 B.alt3 C.alt1 C.alt2 C.alt3
        a.alt1 a.alt2 a.alt3 a.end b.alt c.mid d.alt d.mid
        e.begin e.mid e.end m.begin n.end s.end z.end
        Eng Eng.alt1 Eng.alt2 Eng.alt3
        A.swash B.swash C.swash D.swash E.swash F.swash G.swash H.swash
        I.swash J.swash K.swash L.swash M.swash N.swash O.swash P.swash
        Q.swash R.swash S.swash T.swash U.swash V.swash W.swash X.swash
        Y.swash Z.swash
        f_l c_h c_k c_s c_t f_f f_f_i f_f_l f_i o_f_f_i s_t f_i.begin
        a_n_d T_h T_h.swash germandbls ydieresis yacute breve
        grave acute dieresis macron circumflex cedilla umlaut ogonek caron
        damma hamza sukun kasratan lam_meem_jeem noon.final noon.initial
        by feature lookup sub table uni0327 uni0328 e.fina
    """.split()
    glyphs.extend("cid{:05d}".format(cid) for cid in range(800, 1001 + 1))
    font = TTFont()
    font.setGlyphOrder(glyphs)
    return font


class BuilderTest(unittest.TestCase):
    # Feature files in data/*.fea; output gets compared to data/*.ttx.
    TEST_FEATURE_FILES = """
        Attach cid_range enum markClass language_required
        GlyphClassDef LigatureCaretByIndex LigatureCaretByPos
        lookup lookupflag feature_aalt ignore_pos
        GPOS_1 GPOS_1_zero GPOS_2 GPOS_2b GPOS_3 GPOS_4 GPOS_5 GPOS_6 GPOS_8
        GSUB_2 GSUB_3 GSUB_6 GSUB_8
        spec4h1 spec4h2 spec5d1 spec5d2 spec5fi1 spec5fi2 spec5fi3 spec5fi4
        spec5f_ii_1 spec5f_ii_2 spec5f_ii_3 spec5f_ii_4
        spec5h1 spec6b_ii spec6d2 spec6e spec6f
        spec6h_ii spec6h_iii_1 spec6h_iii_3d spec8a spec8b spec8c spec8d
        spec9a spec9b spec9c1 spec9c2 spec9c3 spec9d spec9e spec9f spec9g
        spec10
        bug453 bug457 bug463 bug501 bug502 bug504 bug505 bug506 bug509
        bug512 bug514 bug568 bug633 bug1307 bug1459 bug2276
        name size size2 multiple_feature_blocks omitted_GlyphClassDef
        ZeroValue_SinglePos_horizontal ZeroValue_SinglePos_vertical
        ZeroValue_PairPos_horizontal ZeroValue_PairPos_vertical
        ZeroValue_ChainSinglePos_horizontal ZeroValue_ChainSinglePos_vertical
        PairPosSubtable ChainSubstSubtable SubstSubtable ChainPosSubtable 
        LigatureSubtable AlternateSubtable MultipleSubstSubtable 
        SingleSubstSubtable aalt_chain_contextual_subst AlternateChained 
        MultipleLookupsPerGlyph MultipleLookupsPerGlyph2 GSUB_6_formats
        GSUB_5_formats delete_glyph STAT_test STAT_test_elidedFallbackNameID
    """.split()

    def __init__(self, methodName):
        unittest.TestCase.__init__(self, methodName)
        # Python 3 renamed assertRaisesRegexp to assertRaisesRegex,
        # and fires deprecation warnings if a program uses the old name.
        if not hasattr(self, "assertRaisesRegex"):
            self.assertRaisesRegex = self.assertRaisesRegexp

    def setUp(self):
        self.tempdir = None
        self.num_tempfiles = 0

    def tearDown(self):
        if self.tempdir:
            shutil.rmtree(self.tempdir)

    @staticmethod
    def getpath(testfile):
        path, _ = os.path.split(__file__)
        return os.path.join(path, "data", testfile)

    def temp_path(self, suffix):
        if not self.tempdir:
            self.tempdir = tempfile.mkdtemp()
        self.num_tempfiles += 1
        return os.path.join(self.tempdir,
                            "tmp%d%s" % (self.num_tempfiles, suffix))

    def read_ttx(self, path):
        lines = []
        with open(path, "r", encoding="utf-8") as ttx:
            for line in ttx.readlines():
                # Elide ttFont attributes because ttLibVersion may change.
                if line.startswith("<ttFont "):
                    lines.append("<ttFont>\n")
                else:
                    lines.append(line.rstrip() + "\n")
        return lines

    def expect_ttx(self, font, expected_ttx, replace=None):
        path = self.temp_path(suffix=".ttx")
        font.saveXML(path, tables=['head', 'name', 'BASE', 'GDEF', 'GSUB',
                                   'GPOS', 'OS/2', 'STAT', 'hhea', 'vhea'])
        actual = self.read_ttx(path)
        expected = self.read_ttx(expected_ttx)
        if replace:
            for i in range(len(expected)):
                for k, v in replace.items():
                    expected[i] = expected[i].replace(k, v)
        if actual != expected:
            for line in difflib.unified_diff(
                    expected, actual, fromfile=expected_ttx, tofile=path):
                sys.stderr.write(line)
            self.fail("TTX output is different from expected")

    def build(self, featureFile, tables=None):
        font = makeTTFont()
        addOpenTypeFeaturesFromString(font, featureFile, tables=tables)
        return font

    def check_feature_file(self, name):
        font = makeTTFont()
        feapath = self.getpath("%s.fea" % name)
        font = addOpenTypeFeatures(font, feapath)
        self.expect_ttx(font, self.getpath("%s.ttx" % name))
        # Check that:
        # 1) tables do compile (only G* tables as long as we have a mock font)
        # 2) dumping after save-reload yields the same TTX dump as before
        for tag in ('GDEF', 'GSUB', 'GPOS'):
            if tag in font:
                data = font[tag].compile(font)
                font[tag].decompile(data, font)
        self.expect_ttx(font, self.getpath("%s.ttx" % name))
        # Optionally check a debug dump.
        debugttx = self.getpath("%s-debug.ttx" % name)
        if os.path.exists(debugttx):
            addOpenTypeFeatures(font, feapath, debug=True)
            self.expect_ttx(font, debugttx, replace = {"__PATH__": feapath})

    def normal_fea(self, lines):
        output = []
        skip = 0
        for l in lines:
            l = l.strip()
            if l.startswith("#test-fea2fea:"):
                if len(l) > 15:
                    output.append(l[15:].strip())
                skip = 1
            x = l.find("#")
            if x >= 0:
                l = l[:x].strip()
            if not len(l):
                continue
            if skip > 0:
                skip = skip - 1
                continue
            output.append(l)
        return output

    def test_alternateSubst_multipleSubstitutionsForSameGlyph(self):
        self.assertRaisesRegex(
            FeatureLibError,
            "Already defined alternates for glyph \"A\"",
            self.build,
            "feature test {"
            "    sub A from [A.alt1 A.alt2];"
            "    sub B from [B.alt1 B.alt2 B.alt3];"
            "    sub A from [A.alt1 A.alt2];"
            "} test;")

    def test_singleSubst_multipleIdenticalSubstitutionsForSameGlyph_info(self):
        logger = logging.getLogger("fontTools.feaLib.builder")
        with CapturingLogHandler(logger, "INFO") as captor:
            self.build(
                "feature test {"
                "    sub A by A.sc;"
                "    sub B by B.sc;"
                "    sub A by A.sc;"
                "} test;")
        captor.assertRegex('Removing duplicate single substitution from glyph "A" to "A.sc"')

    def test_multipleSubst_multipleSubstitutionsForSameGlyph(self):
        self.assertRaisesRegex(
            FeatureLibError,
            "Already defined substitution for glyph \"f_f_i\"",
            self.build,
            "feature test {"
            "    sub f_f_i by f f i;"
            "    sub c_t by c t;"
            "    sub f_f_i by f_f i;"
            "} test;")

    def test_multipleSubst_multipleIdenticalSubstitutionsForSameGlyph_info(self):
        logger = logging.getLogger("fontTools.feaLib.builder")
        with CapturingLogHandler(logger, "INFO") as captor:
            self.build(
                "feature test {"
                "    sub f_f_i by f f i;"
                "    sub c_t by c t;"
                "    sub f_f_i by f f i;"
                "} test;")
        captor.assertRegex(r"Removing duplicate multiple substitution from glyph \"f_f_i\" to \('f', 'f', 'i'\)")

    def test_pairPos_redefinition_warning(self):
        # https://github.com/fonttools/fonttools/issues/1147
        logger = logging.getLogger("fontTools.otlLib.builder")
        with CapturingLogHandler(logger, "DEBUG") as captor:
            # the pair "yacute semicolon" is redefined in the enum pos
            font = self.build(
                "@Y_LC = [y yacute ydieresis];"
                "@SMALL_PUNC = [comma semicolon period];"
                "feature kern {"
                "  pos yacute semicolon -70;"
                "  enum pos @Y_LC semicolon -80;"
                "  pos @Y_LC @SMALL_PUNC -100;"
                "} kern;")

        captor.assertRegex("Already defined position for pair yacute semicolon")

        # the first definition prevails: yacute semicolon -70
        st = font["GPOS"].table.LookupList.Lookup[0].SubTable[0]
        self.assertEqual(st.Coverage.glyphs[2], "yacute")
        self.assertEqual(st.PairSet[2].PairValueRecord[0].SecondGlyph,
                         "semicolon")
        self.assertEqual(vars(st.PairSet[2].PairValueRecord[0].Value1),
                         {"XAdvance": -70})

    def test_singleSubst_multipleSubstitutionsForSameGlyph(self):
        self.assertRaisesRegex(
            FeatureLibError,
            'Already defined rule for replacing glyph "e" by "E.sc"',
            self.build,
            "feature test {"
            "    sub [a-z] by [A.sc-Z.sc];"
            "    sub e by e.fina;"
            "} test;")

    def test_singlePos_redefinition(self):
        self.assertRaisesRegex(
            FeatureLibError,
            "Already defined different position for glyph \"A\"",
            self.build, "feature test { pos A 123; pos A 456; } test;")

    def test_feature_outside_aalt(self):
        self.assertRaisesRegex(
            FeatureLibError,
            'Feature references are only allowed inside "feature aalt"',
            self.build, "feature test { feature test; } test;")

    def test_feature_undefinedReference(self):
        self.assertRaisesRegex(
            FeatureLibError, 'Feature none has not been defined',
            self.build, "feature aalt { feature none; } aalt;")

    def test_GlyphClassDef_conflictingClasses(self):
        self.assertRaisesRegex(
            FeatureLibError, "Glyph X was assigned to a different class",
            self.build,
            "table GDEF {"
            "    GlyphClassDef [a b], [X], , ;"
            "    GlyphClassDef [a b X], , , ;"
            "} GDEF;")

    def test_languagesystem(self):
        builder = Builder(makeTTFont(), (None, None))
        builder.add_language_system(None, 'latn', 'FRA')
        builder.add_language_system(None, 'cyrl', 'RUS')
        builder.start_feature(location=None, name='test')
        self.assertEqual(builder.language_systems,
                         {('latn', 'FRA'), ('cyrl', 'RUS')})

    def test_languagesystem_duplicate(self):
        self.assertRaisesRegex(
            FeatureLibError,
            '"languagesystem cyrl RUS" has already been specified',
            self.build, "languagesystem cyrl RUS; languagesystem cyrl RUS;")

    def test_languagesystem_none_specified(self):
        builder = Builder(makeTTFont(), (None, None))
        builder.start_feature(location=None, name='test')
        self.assertEqual(builder.language_systems, {('DFLT', 'dflt')})

    def test_languagesystem_DFLT_dflt_not_first(self):
        self.assertRaisesRegex(
            FeatureLibError,
            "If \"languagesystem DFLT dflt\" is present, "
            "it must be the first of the languagesystem statements",
            self.build, "languagesystem latn TRK; languagesystem DFLT dflt;")

    def test_languagesystem_DFLT_not_preceding(self):
        self.assertRaisesRegex(
            FeatureLibError,
            "languagesystems using the \"DFLT\" script tag must "
            "precede all other languagesystems",
            self.build,
            "languagesystem DFLT dflt; "
            "languagesystem latn dflt; "
            "languagesystem DFLT fooo; "
        )

    def test_script(self):
        builder = Builder(makeTTFont(), (None, None))
        builder.start_feature(location=None, name='test')
        builder.set_script(location=None, script='cyrl')
        self.assertEqual(builder.language_systems, {('cyrl', 'dflt')})

    def test_script_in_aalt_feature(self):
        self.assertRaisesRegex(
            FeatureLibError,
            "Script statements are not allowed within \"feature aalt\"",
            self.build, "feature aalt { script latn; } aalt;")

    def test_script_in_size_feature(self):
        self.assertRaisesRegex(
            FeatureLibError,
            "Script statements are not allowed within \"feature size\"",
            self.build, "feature size { script latn; } size;")

    def test_script_in_standalone_lookup(self):
        self.assertRaisesRegex(
            FeatureLibError,
            "Script statements are not allowed within standalone lookup blocks",
            self.build, "lookup test { script latn; } test;")

    def test_language(self):
        builder = Builder(makeTTFont(), (None, None))
        builder.add_language_system(None, 'latn', 'FRA ')
        builder.start_feature(location=None, name='test')
        builder.set_script(location=None, script='cyrl')
        builder.set_language(location=None, language='RUS ',
                             include_default=False, required=False)
        self.assertEqual(builder.language_systems, {('cyrl', 'RUS ')})
        builder.set_language(location=None, language='BGR ',
                             include_default=True, required=False)
        self.assertEqual(builder.language_systems,
                         {('cyrl', 'BGR ')})
        builder.start_feature(location=None, name='test2')
        self.assertEqual(builder.language_systems, {('latn', 'FRA ')})

    def test_language_in_aalt_feature(self):
        self.assertRaisesRegex(
            FeatureLibError,
            "Language statements are not allowed within \"feature aalt\"",
            self.build, "feature aalt { language FRA; } aalt;")

    def test_language_in_size_feature(self):
        self.assertRaisesRegex(
            FeatureLibError,
            "Language statements are not allowed within \"feature size\"",
            self.build, "feature size { language FRA; } size;")

    def test_language_in_standalone_lookup(self):
        self.assertRaisesRegex(
            FeatureLibError,
            "Language statements are not allowed within standalone lookup blocks",
            self.build, "lookup test { language FRA; } test;")

    def test_language_required_duplicate(self):
        self.assertRaisesRegex(
            FeatureLibError,
            r"Language FRA \(script latn\) has already specified "
            "feature scmp as its required feature",
            self.build,
            "feature scmp {"
            "    script latn;"
            "    language FRA required;"
            "    language DEU required;"
            "    substitute [a-z] by [A.sc-Z.sc];"
            "} scmp;"
            "feature test {"
            "    script latn;"
            "    language FRA required;"
            "    substitute [a-z] by [A.sc-Z.sc];"
            "} test;")

    def test_lookup_already_defined(self):
        self.assertRaisesRegex(
            FeatureLibError,
            "Lookup \"foo\" has already been defined",
            self.build, "lookup foo {} foo; lookup foo {} foo;")

    def test_lookup_multiple_flags(self):
        self.assertRaisesRegex(
            FeatureLibError,
            "Within a named lookup block, all rules must be "
            "of the same lookup type and flag",
            self.build,
            "lookup foo {"
            "    lookupflag 1;"
            "    sub f i by f_i;"
            "    lookupflag 2;"
            "    sub f f i by f_f_i;"
            "} foo;")

    def test_lookup_multiple_types(self):
        self.assertRaisesRegex(
            FeatureLibError,
            "Within a named lookup block, all rules must be "
            "of the same lookup type and flag",
            self.build,
            "lookup foo {"
            "    sub f f i by f_f_i;"
            "    sub A from [A.alt1 A.alt2];"
            "} foo;")

    def test_lookup_inside_feature_aalt(self):
        self.assertRaisesRegex(
            FeatureLibError,
            "Lookup blocks cannot be placed inside 'aalt' features",
            self.build, "feature aalt {lookup L {} L;} aalt;")

    def test_chain_subst_refrences_GPOS_looup(self):
        self.assertRaisesRegex(
            FeatureLibError,
            "Missing index of the specified lookup, might be a positioning lookup",
            self.build,
            "lookup dummy { pos a 50; } dummy;"
            "feature test {"
            "    sub a' lookup dummy b;"
            "} test;"
        )

    def test_chain_pos_refrences_GSUB_looup(self):
        self.assertRaisesRegex(
            FeatureLibError,
            "Missing index of the specified lookup, might be a substitution lookup",
            self.build,
            "lookup dummy { sub a by A; } dummy;"
            "feature test {"
            "    pos a' lookup dummy b;"
            "} test;"
        )

    def test_STAT_elidedfallbackname_already_defined(self):
        self.assertRaisesRegex(
            FeatureLibError,
            'ElidedFallbackName is already set.',
            self.build,
            'table name {'
            '   nameid 256 "Roman"; '
            '} name;'
            'table STAT {'
            '    ElidedFallbackName { name "Roman"; };'
            '    ElidedFallbackNameID 256;'
            '} STAT;')

    def test_STAT_elidedfallbackname_set_twice(self):
        self.assertRaisesRegex(
            FeatureLibError,
            'ElidedFallbackName is already set.',
            self.build,
            'table name {'
            '   nameid 256 "Roman"; '
            '} name;'
            'table STAT {'
            '    ElidedFallbackName { name "Roman"; };'
            '    ElidedFallbackName { name "Italic"; };'
            '} STAT;')

    def test_STAT_elidedfallbacknameID_already_defined(self):
        self.assertRaisesRegex(
            FeatureLibError,
            'ElidedFallbackNameID is already set.',
            self.build,
            'table name {'
            '   nameid 256 "Roman"; '
            '} name;'
            'table STAT {'
            '    ElidedFallbackNameID 256;'
            '    ElidedFallbackName { name "Roman"; };'
            '} STAT;')

    def test_STAT_elidedfallbacknameID_not_in_name_table(self):
        self.assertRaisesRegex(
            FeatureLibError,
            'ElidedFallbackNameID 256 points to a nameID that does not '
            'exist in the "name" table',
            self.build,
            'table name {'
            '   nameid 257 "Roman"; '
            '} name;'
            'table STAT {'
            '    ElidedFallbackNameID 256;'
            '    DesignAxis opsz 1 { name "Optical Size"; };'
            '} STAT;')

    def test_STAT_design_axis_name(self):
        self.assertRaisesRegex(
            FeatureLibError,
            'Expected "name"',
            self.build,
            'table name {'
            '   nameid 256 "Roman"; '
            '} name;'
            'table STAT {'
            '    ElidedFallbackName { name "Roman"; };'
            '    DesignAxis opsz 0 { badtag "Optical Size"; };'
            '} STAT;')

    def test_STAT_duplicate_design_axis_name(self):
        self.assertRaisesRegex(
            FeatureLibError,
            'DesignAxis already defined for tag "opsz".',
            self.build,
            'table name {'
            '   nameid 256 "Roman"; '
            '} name;'
            'table STAT {'
            '    ElidedFallbackName { name "Roman"; };'
            '    DesignAxis opsz 0 { name "Optical Size"; };'
            '    DesignAxis opsz 1 { name "Optical Size"; };'
            '} STAT;')

    def test_STAT_design_axis_duplicate_order(self):
        self.assertRaisesRegex(
            FeatureLibError,
            "DesignAxis already defined for axis number 0.",
            self.build,
            'table name {'
            '   nameid 256 "Roman"; '
            '} name;'
            'table STAT {'
            '    ElidedFallbackName { name "Roman"; };'
            '    DesignAxis opsz 0 { name "Optical Size"; };'
            '    DesignAxis wdth 0 { name "Width"; };'
            '    AxisValue {'
            '         location opsz 8;'
            '         location wdth 400;'
            '         name "Caption";'
            '     };'
            '} STAT;')

    def test_STAT_undefined_tag(self):
        self.assertRaisesRegex(
            FeatureLibError,
            'DesignAxis not defined for wdth.',
            self.build,
            'table name {'
            '   nameid 256 "Roman"; '
            '} name;'
            'table STAT {'
            '    ElidedFallbackName { name "Roman"; };'
            '    DesignAxis opsz 0 { name "Optical Size"; };'
            '    AxisValue { '
            '        location wdth 125; '
            '        name "Wide"; '
            '    };'
            '} STAT;')

    def test_STAT_axis_value_format4(self):
        self.assertRaisesRegex(
            FeatureLibError,
            'Axis tag wdth already defined.',
            self.build,
            'table name {'
            '   nameid 256 "Roman"; '
            '} name;'
            'table STAT {'
            '    ElidedFallbackName { name "Roman"; };'
            '    DesignAxis opsz 0 { name "Optical Size"; };'
            '    DesignAxis wdth 1 { name "Width"; };'
            '    DesignAxis wght 2 { name "Weight"; };'
            '    AxisValue { '
            '        location opsz 8; '
            '        location wdth 125; '
            '        location wdth 125; '
            '        location wght 500; '
            '        name "Caption Medium Wide"; '
            '    };'
            '} STAT;')

    def test_STAT_duplicate_axis_value_record(self):
        # Test for Duplicate AxisValueRecords even when the definition order
        # is different.
        self.assertRaisesRegex(
            FeatureLibError,
            'An AxisValueRecord with these values is already defined.',
            self.build,
            'table name {'
            '   nameid 256 "Roman"; '
            '} name;'
            'table STAT {'
            '    ElidedFallbackName { name "Roman"; };'
            '    DesignAxis opsz 0 { name "Optical Size"; };'
            '    DesignAxis wdth 1 { name "Width"; };'
            '    AxisValue {'
            '         location opsz 8;'
            '         location wdth 400;'
            '         name "Caption";'
            '     };'
            '    AxisValue {'
            '         location wdth 400;'
            '         location opsz 8;'
            '         name "Caption";'
            '     };'
            '} STAT;')

    def test_STAT_axis_value_missing_location(self):
        self.assertRaisesRegex(
            FeatureLibError,
            'Expected "Axis location"',
            self.build,
            'table name {'
            '   nameid 256 "Roman"; '
            '} name;'
            'table STAT {'
            '    ElidedFallbackName {   name "Roman"; '
            '};'
            '    DesignAxis opsz 0 { name "Optical Size"; };'
            '    AxisValue { '
            '        name "Wide"; '
            '    };'
            '} STAT;')

    def test_STAT_invalid_location_tag(self):
        self.assertRaisesRegex(
            FeatureLibError,
            'Tags cannot be longer than 4 characters',
            self.build,
            'table name {'
            '   nameid 256 "Roman"; '
            '} name;'
            'table STAT {'
            '    ElidedFallbackName { name "Roman"; '
            '                         name 3 1 0x0411 "ローマン"; }; '
            '    DesignAxis width 0 { name "Width"; };'
            '} STAT;')

    def test_extensions(self):
        class ast_BaseClass(ast.MarkClass):
            def asFea(self, indent=""):
                return ""

        class ast_BaseClassDefinition(ast.MarkClassDefinition):
            def asFea(self, indent=""):
                return ""

        class ast_MarkBasePosStatement(ast.MarkBasePosStatement):
            def asFea(self, indent=""):
                if isinstance(self.base, ast.MarkClassName):
                    res = ""
                    for bcd in self.base.markClass.definitions:
                        if res != "":
                            res += "\n{}".format(indent)
                        res += "pos base {} {}".format(bcd.glyphs.asFea(), bcd.anchor.asFea())
                        for m in self.marks:
                            res += " mark @{}".format(m.name)
                        res += ";"
                else:
                    res = "pos base {}".format(self.base.asFea())
                    for a, m in self.marks:
                        res += " {} mark @{}".format(a.asFea(), m.name)
                    res += ";"
                return res

        class testAst(object):
            MarkBasePosStatement = ast_MarkBasePosStatement
            def __getattr__(self, name):
                return getattr(ast, name)

    def test_markClass_same_glyph_redefined(self):
        self.assertRaisesRegex(
            FeatureLibError,
            "Glyph acute already defined",
            self.build,
            "markClass [acute] <anchor 350 0> @TOP_MARKS;"*2)

    def test_markClass_same_glyph_multiple_classes(self):
        self.assertRaisesRegex(
            FeatureLibError,
            'Glyph uni0327 cannot be in both @ogonek and @cedilla',
            self.build,
            "feature mark {"
            "    markClass [uni0327 uni0328] <anchor 0 0> @ogonek;"
            "    pos base [a] <anchor 399 0> mark @ogonek;"
            "    markClass [uni0327] <anchor 0 0> @cedilla;"
            "    pos base [a] <anchor 244 0> mark @cedilla;"
            "} mark;")

    def test_build_specific_tables(self):
        features = "feature liga {sub f i by f_i;} liga;"
        font = self.build(features)
        assert "GSUB" in font

        font2 = self.build(features, tables=set())
        assert "GSUB" not in font2

    def test_build_unsupported_tables(self):
        self.assertRaises(NotImplementedError, self.build, "", tables={"FOO"})

    def test_unsupported_subtable_break(self):
        logger = logging.getLogger("fontTools.otlLib.builder")
        with CapturingLogHandler(logger, level='WARNING') as captor:
            self.build(
                "feature test {"
                "    pos a 10;"
                "    subtable;"
                "    pos b 10;"
                "} test;"
            )

        captor.assertRegex(
            '<features>:1:32: unsupported "subtable" statement for lookup type'
        )

    def test_skip_featureNames_if_no_name_table(self):
        features = (
            "feature ss01 {"
            "    featureNames {"
            '        name "ignored as we request to skip name table";'
            "    };"
            "    sub A by A.alt1;"
            "} ss01;"
        )
        font = self.build(features, tables=["GSUB"])
        self.assertIn("GSUB", font)
        self.assertNotIn("name", font)

    def test_singlePos_multiplePositionsForSameGlyph(self):
        self.assertRaisesRegex(
            FeatureLibError,
            "Already defined different position for glyph",
            self.build,
            "lookup foo {"
            "    pos A -45; "
            "    pos A 45; "
            "} foo;")

    def test_pairPos_enumRuleOverridenBySinglePair_DEBUG(self):
        logger = logging.getLogger("fontTools.otlLib.builder")
        with CapturingLogHandler(logger, "DEBUG") as captor:
            self.build(
                "feature test {"
                "    enum pos A [V Y] -80;"
                "    pos A V -75;"
                "} test;")
        captor.assertRegex('Already defined position for pair A V at')

    def test_ignore_empty_lookup_block(self):
        # https://github.com/fonttools/fonttools/pull/2277
        font = self.build(
            "lookup EMPTY { ; } EMPTY;"
            "feature ss01 { lookup EMPTY; } ss01;"
        )
        assert "GPOS" not in font
        assert "GSUB" not in font


def generate_feature_file_test(name):
    return lambda self: self.check_feature_file(name)


for name in BuilderTest.TEST_FEATURE_FILES:
    setattr(BuilderTest, "test_FeatureFile_%s" % name,
            generate_feature_file_test(name))



if __name__ == "__main__":
    # Check we have built the latest
    import subprocess
    subprocess.run(["cargo", "build", "--features", "build-binary"])

    sys.exit(unittest.main())
