#![allow(dead_code)]
mod bindings;

mod myhtml {
    use crate::bindings as mh;
    use std::ffi;
    use std::ops::Index;

    #[repr(u32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum Encoding {
        Utf8 = mh::myencoding_list::MyENCODING_DEFAULT as u32,
        NotDetermined = mh::myencoding_list::MyENCODING_NOT_DETERMINED as u32,
        Utf16le = mh::myencoding_list::MyENCODING_UTF_16LE as u32,
        Utf16be = mh::myencoding_list::MyENCODING_UTF_16BE as u32,
        XUserDefined = mh::myencoding_list::MyENCODING_X_USER_DEFINED as u32,
        BIG5 = mh::myencoding_list::MyENCODING_BIG5 as u32,
        EucJp = mh::myencoding_list::MyENCODING_EUC_JP as u32,
        EucKr = mh::myencoding_list::MyENCODING_EUC_KR as u32,
        GB18030 = mh::myencoding_list::MyENCODING_GB18030 as u32,
        GBK = mh::myencoding_list::MyENCODING_GBK as u32,
        IBM866 = mh::myencoding_list::MyENCODING_IBM866 as u32,
        Iso2022Jp = mh::myencoding_list::MyENCODING_ISO_2022_JP as u32,
        Iso8859_10 = mh::myencoding_list::MyENCODING_ISO_8859_10 as u32,
        Iso8859_13 = mh::myencoding_list::MyENCODING_ISO_8859_13 as u32,
        Iso8859_14 = mh::myencoding_list::MyENCODING_ISO_8859_14 as u32,
        Iso8859_15 = mh::myencoding_list::MyENCODING_ISO_8859_15 as u32,
        Iso8859_16 = mh::myencoding_list::MyENCODING_ISO_8859_16 as u32,
        Iso8859_2 = mh::myencoding_list::MyENCODING_ISO_8859_2 as u32,
        Iso8859_3 = mh::myencoding_list::MyENCODING_ISO_8859_3 as u32,
        Iso8859_4 = mh::myencoding_list::MyENCODING_ISO_8859_4 as u32,
        Iso8859_5 = mh::myencoding_list::MyENCODING_ISO_8859_5 as u32,
        Iso8859_6 = mh::myencoding_list::MyENCODING_ISO_8859_6 as u32,
        Iso8859_7 = mh::myencoding_list::MyENCODING_ISO_8859_7 as u32,
        Iso8859_8 = mh::myencoding_list::MyENCODING_ISO_8859_8 as u32,
        Iso8859_8I = mh::myencoding_list::MyENCODING_ISO_8859_8_I as u32,
        Koi8R = mh::myencoding_list::MyENCODING_KOI8_R as u32,
        Koi8U = mh::myencoding_list::MyENCODING_KOI8_U as u32,
        MACINTOSH = mh::myencoding_list::MyENCODING_MACINTOSH as u32,
        ShiftJis = mh::myencoding_list::MyENCODING_SHIFT_JIS as u32,
        Windows1250 = mh::myencoding_list::MyENCODING_WINDOWS_1250 as u32,
        Windows1251 = mh::myencoding_list::MyENCODING_WINDOWS_1251 as u32,
        Windows1252 = mh::myencoding_list::MyENCODING_WINDOWS_1252 as u32,
        Windows1253 = mh::myencoding_list::MyENCODING_WINDOWS_1253 as u32,
        Windows1254 = mh::myencoding_list::MyENCODING_WINDOWS_1254 as u32,
        Windows1255 = mh::myencoding_list::MyENCODING_WINDOWS_1255 as u32,
        Windows1256 = mh::myencoding_list::MyENCODING_WINDOWS_1256 as u32,
        Windows1257 = mh::myencoding_list::MyENCODING_WINDOWS_1257 as u32,
        Windows1258 = mh::myencoding_list::MyENCODING_WINDOWS_1258 as u32,
        Windows874 = mh::myencoding_list::MyENCODING_WINDOWS_874 as u32,
        XMacCyrillic = mh::myencoding_list::MyENCODING_X_MAC_CYRILLIC as u32,
        LastEntry = mh::myencoding_list::MyENCODING_LAST_ENTRY as u32,
    }

    #[repr(u32)]
    #[allow(non_camel_case_types)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum TagType {
        _UNDEF = mh::myhtml_tags::MyHTML_TAG__UNDEF as u32,
        _TEXT = mh::myhtml_tags::MyHTML_TAG__TEXT as u32,
        _COMMENT = mh::myhtml_tags::MyHTML_TAG__COMMENT as u32,
        _DOCTYPE = mh::myhtml_tags::MyHTML_TAG__DOCTYPE as u32,
        A = mh::myhtml_tags::MyHTML_TAG_A as u32,
        ABBR = mh::myhtml_tags::MyHTML_TAG_ABBR as u32,
        ACRONYM = mh::myhtml_tags::MyHTML_TAG_ACRONYM as u32,
        ADDRESS = mh::myhtml_tags::MyHTML_TAG_ADDRESS as u32,
        ANNOTATION_XML = mh::myhtml_tags::MyHTML_TAG_ANNOTATION_XML as u32,
        APPLET = mh::myhtml_tags::MyHTML_TAG_APPLET as u32,
        AREA = mh::myhtml_tags::MyHTML_TAG_AREA as u32,
        ARTICLE = mh::myhtml_tags::MyHTML_TAG_ARTICLE as u32,
        ASIDE = mh::myhtml_tags::MyHTML_TAG_ASIDE as u32,
        AUDIO = mh::myhtml_tags::MyHTML_TAG_AUDIO as u32,
        B = mh::myhtml_tags::MyHTML_TAG_B as u32,
        BASE = mh::myhtml_tags::MyHTML_TAG_BASE as u32,
        BASEFONT = mh::myhtml_tags::MyHTML_TAG_BASEFONT as u32,
        BDI = mh::myhtml_tags::MyHTML_TAG_BDI as u32,
        BDO = mh::myhtml_tags::MyHTML_TAG_BDO as u32,
        BGSOUND = mh::myhtml_tags::MyHTML_TAG_BGSOUND as u32,
        BIG = mh::myhtml_tags::MyHTML_TAG_BIG as u32,
        BLINK = mh::myhtml_tags::MyHTML_TAG_BLINK as u32,
        BLOCKQUOTE = mh::myhtml_tags::MyHTML_TAG_BLOCKQUOTE as u32,
        BODY = mh::myhtml_tags::MyHTML_TAG_BODY as u32,
        BR = mh::myhtml_tags::MyHTML_TAG_BR as u32,
        BUTTON = mh::myhtml_tags::MyHTML_TAG_BUTTON as u32,
        CANVAS = mh::myhtml_tags::MyHTML_TAG_CANVAS as u32,
        CAPTION = mh::myhtml_tags::MyHTML_TAG_CAPTION as u32,
        CENTER = mh::myhtml_tags::MyHTML_TAG_CENTER as u32,
        CITE = mh::myhtml_tags::MyHTML_TAG_CITE as u32,
        CODE = mh::myhtml_tags::MyHTML_TAG_CODE as u32,
        COL = mh::myhtml_tags::MyHTML_TAG_COL as u32,
        COLGROUP = mh::myhtml_tags::MyHTML_TAG_COLGROUP as u32,
        COMMAND = mh::myhtml_tags::MyHTML_TAG_COMMAND as u32,
        COMMENT = mh::myhtml_tags::MyHTML_TAG_COMMENT as u32,
        DATALIST = mh::myhtml_tags::MyHTML_TAG_DATALIST as u32,
        DD = mh::myhtml_tags::MyHTML_TAG_DD as u32,
        DEL = mh::myhtml_tags::MyHTML_TAG_DEL as u32,
        DETAILS = mh::myhtml_tags::MyHTML_TAG_DETAILS as u32,
        DFN = mh::myhtml_tags::MyHTML_TAG_DFN as u32,
        DIALOG = mh::myhtml_tags::MyHTML_TAG_DIALOG as u32,
        DIR = mh::myhtml_tags::MyHTML_TAG_DIR as u32,
        DIV = mh::myhtml_tags::MyHTML_TAG_DIV as u32,
        DL = mh::myhtml_tags::MyHTML_TAG_DL as u32,
        DT = mh::myhtml_tags::MyHTML_TAG_DT as u32,
        EM = mh::myhtml_tags::MyHTML_TAG_EM as u32,
        EMBED = mh::myhtml_tags::MyHTML_TAG_EMBED as u32,
        FIELDSET = mh::myhtml_tags::MyHTML_TAG_FIELDSET as u32,
        FIGCAPTION = mh::myhtml_tags::MyHTML_TAG_FIGCAPTION as u32,
        FIGURE = mh::myhtml_tags::MyHTML_TAG_FIGURE as u32,
        FONT = mh::myhtml_tags::MyHTML_TAG_FONT as u32,
        FOOTER = mh::myhtml_tags::MyHTML_TAG_FOOTER as u32,
        FORM = mh::myhtml_tags::MyHTML_TAG_FORM as u32,
        FRAME = mh::myhtml_tags::MyHTML_TAG_FRAME as u32,
        FRAMESET = mh::myhtml_tags::MyHTML_TAG_FRAMESET as u32,
        H1 = mh::myhtml_tags::MyHTML_TAG_H1 as u32,
        H2 = mh::myhtml_tags::MyHTML_TAG_H2 as u32,
        H3 = mh::myhtml_tags::MyHTML_TAG_H3 as u32,
        H4 = mh::myhtml_tags::MyHTML_TAG_H4 as u32,
        H5 = mh::myhtml_tags::MyHTML_TAG_H5 as u32,
        H6 = mh::myhtml_tags::MyHTML_TAG_H6 as u32,
        HEAD = mh::myhtml_tags::MyHTML_TAG_HEAD as u32,
        HEADER = mh::myhtml_tags::MyHTML_TAG_HEADER as u32,
        HGROUP = mh::myhtml_tags::MyHTML_TAG_HGROUP as u32,
        HR = mh::myhtml_tags::MyHTML_TAG_HR as u32,
        HTML = mh::myhtml_tags::MyHTML_TAG_HTML as u32,
        I = mh::myhtml_tags::MyHTML_TAG_I as u32,
        IFRAME = mh::myhtml_tags::MyHTML_TAG_IFRAME as u32,
        IMAGE = mh::myhtml_tags::MyHTML_TAG_IMAGE as u32,
        IMG = mh::myhtml_tags::MyHTML_TAG_IMG as u32,
        INPUT = mh::myhtml_tags::MyHTML_TAG_INPUT as u32,
        INS = mh::myhtml_tags::MyHTML_TAG_INS as u32,
        ISINDEX = mh::myhtml_tags::MyHTML_TAG_ISINDEX as u32,
        KBD = mh::myhtml_tags::MyHTML_TAG_KBD as u32,
        KEYGEN = mh::myhtml_tags::MyHTML_TAG_KEYGEN as u32,
        LABEL = mh::myhtml_tags::MyHTML_TAG_LABEL as u32,
        LEGEND = mh::myhtml_tags::MyHTML_TAG_LEGEND as u32,
        LI = mh::myhtml_tags::MyHTML_TAG_LI as u32,
        LINK = mh::myhtml_tags::MyHTML_TAG_LINK as u32,
        LISTING = mh::myhtml_tags::MyHTML_TAG_LISTING as u32,
        MAIN = mh::myhtml_tags::MyHTML_TAG_MAIN as u32,
        MAP = mh::myhtml_tags::MyHTML_TAG_MAP as u32,
        MARK = mh::myhtml_tags::MyHTML_TAG_MARK as u32,
        MARQUEE = mh::myhtml_tags::MyHTML_TAG_MARQUEE as u32,
        MENU = mh::myhtml_tags::MyHTML_TAG_MENU as u32,
        MENUITEM = mh::myhtml_tags::MyHTML_TAG_MENUITEM as u32,
        META = mh::myhtml_tags::MyHTML_TAG_META as u32,
        METER = mh::myhtml_tags::MyHTML_TAG_METER as u32,
        MTEXT = mh::myhtml_tags::MyHTML_TAG_MTEXT as u32,
        NAV = mh::myhtml_tags::MyHTML_TAG_NAV as u32,
        NOBR = mh::myhtml_tags::MyHTML_TAG_NOBR as u32,
        NOEMBED = mh::myhtml_tags::MyHTML_TAG_NOEMBED as u32,
        NOFRAMES = mh::myhtml_tags::MyHTML_TAG_NOFRAMES as u32,
        NOSCRIPT = mh::myhtml_tags::MyHTML_TAG_NOSCRIPT as u32,
        OBJECT = mh::myhtml_tags::MyHTML_TAG_OBJECT as u32,
        OL = mh::myhtml_tags::MyHTML_TAG_OL as u32,
        OPTGROUP = mh::myhtml_tags::MyHTML_TAG_OPTGROUP as u32,
        OPTION = mh::myhtml_tags::MyHTML_TAG_OPTION as u32,
        OUTPUT = mh::myhtml_tags::MyHTML_TAG_OUTPUT as u32,
        P = mh::myhtml_tags::MyHTML_TAG_P as u32,
        PARAM = mh::myhtml_tags::MyHTML_TAG_PARAM as u32,
        PLAINTEXT = mh::myhtml_tags::MyHTML_TAG_PLAINTEXT as u32,
        PRE = mh::myhtml_tags::MyHTML_TAG_PRE as u32,
        PROGRESS = mh::myhtml_tags::MyHTML_TAG_PROGRESS as u32,
        Q = mh::myhtml_tags::MyHTML_TAG_Q as u32,
        RB = mh::myhtml_tags::MyHTML_TAG_RB as u32,
        RP = mh::myhtml_tags::MyHTML_TAG_RP as u32,
        RT = mh::myhtml_tags::MyHTML_TAG_RT as u32,
        RTC = mh::myhtml_tags::MyHTML_TAG_RTC as u32,
        RUBY = mh::myhtml_tags::MyHTML_TAG_RUBY as u32,
        S = mh::myhtml_tags::MyHTML_TAG_S as u32,
        SAMP = mh::myhtml_tags::MyHTML_TAG_SAMP as u32,
        SCRIPT = mh::myhtml_tags::MyHTML_TAG_SCRIPT as u32,
        SECTION = mh::myhtml_tags::MyHTML_TAG_SECTION as u32,
        SELECT = mh::myhtml_tags::MyHTML_TAG_SELECT as u32,
        SMALL = mh::myhtml_tags::MyHTML_TAG_SMALL as u32,
        SOURCE = mh::myhtml_tags::MyHTML_TAG_SOURCE as u32,
        SPAN = mh::myhtml_tags::MyHTML_TAG_SPAN as u32,
        STRIKE = mh::myhtml_tags::MyHTML_TAG_STRIKE as u32,
        STRONG = mh::myhtml_tags::MyHTML_TAG_STRONG as u32,
        STYLE = mh::myhtml_tags::MyHTML_TAG_STYLE as u32,
        SUB = mh::myhtml_tags::MyHTML_TAG_SUB as u32,
        SUMMARY = mh::myhtml_tags::MyHTML_TAG_SUMMARY as u32,
        SUP = mh::myhtml_tags::MyHTML_TAG_SUP as u32,
        SVG = mh::myhtml_tags::MyHTML_TAG_SVG as u32,
        TABLE = mh::myhtml_tags::MyHTML_TAG_TABLE as u32,
        TBODY = mh::myhtml_tags::MyHTML_TAG_TBODY as u32,
        TD = mh::myhtml_tags::MyHTML_TAG_TD as u32,
        TEMPLATE = mh::myhtml_tags::MyHTML_TAG_TEMPLATE as u32,
        TEXTAREA = mh::myhtml_tags::MyHTML_TAG_TEXTAREA as u32,
        TFOOT = mh::myhtml_tags::MyHTML_TAG_TFOOT as u32,
        TH = mh::myhtml_tags::MyHTML_TAG_TH as u32,
        THEAD = mh::myhtml_tags::MyHTML_TAG_THEAD as u32,
        TIME = mh::myhtml_tags::MyHTML_TAG_TIME as u32,
        TITLE = mh::myhtml_tags::MyHTML_TAG_TITLE as u32,
        TR = mh::myhtml_tags::MyHTML_TAG_TR as u32,
        TRACK = mh::myhtml_tags::MyHTML_TAG_TRACK as u32,
        TT = mh::myhtml_tags::MyHTML_TAG_TT as u32,
        U = mh::myhtml_tags::MyHTML_TAG_U as u32,
        UL = mh::myhtml_tags::MyHTML_TAG_UL as u32,
        VAR = mh::myhtml_tags::MyHTML_TAG_VAR as u32,
        VIDEO = mh::myhtml_tags::MyHTML_TAG_VIDEO as u32,
        WBR = mh::myhtml_tags::MyHTML_TAG_WBR as u32,
        XMP = mh::myhtml_tags::MyHTML_TAG_XMP as u32,
        ALTGLYPH = mh::myhtml_tags::MyHTML_TAG_ALTGLYPH as u32,
        ALTGLYPHDEF = mh::myhtml_tags::MyHTML_TAG_ALTGLYPHDEF as u32,
        ALTGLYPHITEM = mh::myhtml_tags::MyHTML_TAG_ALTGLYPHITEM as u32,
        ANIMATE = mh::myhtml_tags::MyHTML_TAG_ANIMATE as u32,
        ANIMATECOLOR = mh::myhtml_tags::MyHTML_TAG_ANIMATECOLOR as u32,
        ANIMATEMOTION = mh::myhtml_tags::MyHTML_TAG_ANIMATEMOTION as u32,
        ANIMATETRANSFORM = mh::myhtml_tags::MyHTML_TAG_ANIMATETRANSFORM as u32,
        CIRCLE = mh::myhtml_tags::MyHTML_TAG_CIRCLE as u32,
        CLIPPATH = mh::myhtml_tags::MyHTML_TAG_CLIPPATH as u32,
        COLOR_PROFILE = mh::myhtml_tags::MyHTML_TAG_COLOR_PROFILE as u32,
        CURSOR = mh::myhtml_tags::MyHTML_TAG_CURSOR as u32,
        DEFS = mh::myhtml_tags::MyHTML_TAG_DEFS as u32,
        DESC = mh::myhtml_tags::MyHTML_TAG_DESC as u32,
        ELLIPSE = mh::myhtml_tags::MyHTML_TAG_ELLIPSE as u32,
        FEBLEND = mh::myhtml_tags::MyHTML_TAG_FEBLEND as u32,
        FECOLORMATRIX = mh::myhtml_tags::MyHTML_TAG_FECOLORMATRIX as u32,
        FECOMPONENTTRANSFER = mh::myhtml_tags::MyHTML_TAG_FECOMPONENTTRANSFER as u32,
        FECOMPOSITE = mh::myhtml_tags::MyHTML_TAG_FECOMPOSITE as u32,
        FECONVOLVEMATRIX = mh::myhtml_tags::MyHTML_TAG_FECONVOLVEMATRIX as u32,
        FEDIFFUSELIGHTING = mh::myhtml_tags::MyHTML_TAG_FEDIFFUSELIGHTING as u32,
        FEDISPLACEMENTMAP = mh::myhtml_tags::MyHTML_TAG_FEDISPLACEMENTMAP as u32,
        FEDISTANTLIGHT = mh::myhtml_tags::MyHTML_TAG_FEDISTANTLIGHT as u32,
        FEDROPSHADOW = mh::myhtml_tags::MyHTML_TAG_FEDROPSHADOW as u32,
        FEFLOOD = mh::myhtml_tags::MyHTML_TAG_FEFLOOD as u32,
        FEFUNCA = mh::myhtml_tags::MyHTML_TAG_FEFUNCA as u32,
        FEFUNCB = mh::myhtml_tags::MyHTML_TAG_FEFUNCB as u32,
        FEFUNCG = mh::myhtml_tags::MyHTML_TAG_FEFUNCG as u32,
        FEFUNCR = mh::myhtml_tags::MyHTML_TAG_FEFUNCR as u32,
        FEGAUSSIANBLUR = mh::myhtml_tags::MyHTML_TAG_FEGAUSSIANBLUR as u32,
        FEIMAGE = mh::myhtml_tags::MyHTML_TAG_FEIMAGE as u32,
        FEMERGE = mh::myhtml_tags::MyHTML_TAG_FEMERGE as u32,
        FEMERGENODE = mh::myhtml_tags::MyHTML_TAG_FEMERGENODE as u32,
        FEMORPHOLOGY = mh::myhtml_tags::MyHTML_TAG_FEMORPHOLOGY as u32,
        FEOFFSET = mh::myhtml_tags::MyHTML_TAG_FEOFFSET as u32,
        FEPOINTLIGHT = mh::myhtml_tags::MyHTML_TAG_FEPOINTLIGHT as u32,
        FESPECULARLIGHTING = mh::myhtml_tags::MyHTML_TAG_FESPECULARLIGHTING as u32,
        FESPOTLIGHT = mh::myhtml_tags::MyHTML_TAG_FESPOTLIGHT as u32,
        FETILE = mh::myhtml_tags::MyHTML_TAG_FETILE as u32,
        FETURBULENCE = mh::myhtml_tags::MyHTML_TAG_FETURBULENCE as u32,
        FILTER = mh::myhtml_tags::MyHTML_TAG_FILTER as u32,
        FONT_FACE = mh::myhtml_tags::MyHTML_TAG_FONT_FACE as u32,
        FONT_FACE_FORMAT = mh::myhtml_tags::MyHTML_TAG_FONT_FACE_FORMAT as u32,
        FONT_FACE_NAME = mh::myhtml_tags::MyHTML_TAG_FONT_FACE_NAME as u32,
        FONT_FACE_SRC = mh::myhtml_tags::MyHTML_TAG_FONT_FACE_SRC as u32,
        FONT_FACE_URI = mh::myhtml_tags::MyHTML_TAG_FONT_FACE_URI as u32,
        FOREIGNOBJECT = mh::myhtml_tags::MyHTML_TAG_FOREIGNOBJECT as u32,
        G = mh::myhtml_tags::MyHTML_TAG_G as u32,
        GLYPH = mh::myhtml_tags::MyHTML_TAG_GLYPH as u32,
        GLYPHREF = mh::myhtml_tags::MyHTML_TAG_GLYPHREF as u32,
        HKERN = mh::myhtml_tags::MyHTML_TAG_HKERN as u32,
        LINE = mh::myhtml_tags::MyHTML_TAG_LINE as u32,
        LINEARGRADIENT = mh::myhtml_tags::MyHTML_TAG_LINEARGRADIENT as u32,
        MARKER = mh::myhtml_tags::MyHTML_TAG_MARKER as u32,
        MASK = mh::myhtml_tags::MyHTML_TAG_MASK as u32,
        METADATA = mh::myhtml_tags::MyHTML_TAG_METADATA as u32,
        MISSING_GLYPH = mh::myhtml_tags::MyHTML_TAG_MISSING_GLYPH as u32,
        MPATH = mh::myhtml_tags::MyHTML_TAG_MPATH as u32,
        PATH = mh::myhtml_tags::MyHTML_TAG_PATH as u32,
        PATTERN = mh::myhtml_tags::MyHTML_TAG_PATTERN as u32,
        POLYGON = mh::myhtml_tags::MyHTML_TAG_POLYGON as u32,
        POLYLINE = mh::myhtml_tags::MyHTML_TAG_POLYLINE as u32,
        RADIALGRADIENT = mh::myhtml_tags::MyHTML_TAG_RADIALGRADIENT as u32,
        RECT = mh::myhtml_tags::MyHTML_TAG_RECT as u32,
        SET = mh::myhtml_tags::MyHTML_TAG_SET as u32,
        STOP = mh::myhtml_tags::MyHTML_TAG_STOP as u32,
        SWITCH = mh::myhtml_tags::MyHTML_TAG_SWITCH as u32,
        SYMBOL = mh::myhtml_tags::MyHTML_TAG_SYMBOL as u32,
        TEXT = mh::myhtml_tags::MyHTML_TAG_TEXT as u32,
        TEXTPATH = mh::myhtml_tags::MyHTML_TAG_TEXTPATH as u32,
        TREF = mh::myhtml_tags::MyHTML_TAG_TREF as u32,
        TSPAN = mh::myhtml_tags::MyHTML_TAG_TSPAN as u32,
        USE = mh::myhtml_tags::MyHTML_TAG_USE as u32,
        VIEW = mh::myhtml_tags::MyHTML_TAG_VIEW as u32,
        VKERN = mh::myhtml_tags::MyHTML_TAG_VKERN as u32,
        MATH = mh::myhtml_tags::MyHTML_TAG_MATH as u32,
        MACTION = mh::myhtml_tags::MyHTML_TAG_MACTION as u32,
        MALIGNGROUP = mh::myhtml_tags::MyHTML_TAG_MALIGNGROUP as u32,
        MALIGNMARK = mh::myhtml_tags::MyHTML_TAG_MALIGNMARK as u32,
        MENCLOSE = mh::myhtml_tags::MyHTML_TAG_MENCLOSE as u32,
        MERROR = mh::myhtml_tags::MyHTML_TAG_MERROR as u32,
        MFENCED = mh::myhtml_tags::MyHTML_TAG_MFENCED as u32,
        MFRAC = mh::myhtml_tags::MyHTML_TAG_MFRAC as u32,
        MGLYPH = mh::myhtml_tags::MyHTML_TAG_MGLYPH as u32,
        MI = mh::myhtml_tags::MyHTML_TAG_MI as u32,
        MLABELEDTR = mh::myhtml_tags::MyHTML_TAG_MLABELEDTR as u32,
        MLONGDIV = mh::myhtml_tags::MyHTML_TAG_MLONGDIV as u32,
        MMULTISCRIPTS = mh::myhtml_tags::MyHTML_TAG_MMULTISCRIPTS as u32,
        MN = mh::myhtml_tags::MyHTML_TAG_MN as u32,
        MO = mh::myhtml_tags::MyHTML_TAG_MO as u32,
        MOVER = mh::myhtml_tags::MyHTML_TAG_MOVER as u32,
        MPADDED = mh::myhtml_tags::MyHTML_TAG_MPADDED as u32,
        MPHANTOM = mh::myhtml_tags::MyHTML_TAG_MPHANTOM as u32,
        MROOT = mh::myhtml_tags::MyHTML_TAG_MROOT as u32,
        MROW = mh::myhtml_tags::MyHTML_TAG_MROW as u32,
        MS = mh::myhtml_tags::MyHTML_TAG_MS as u32,
        MSCARRIES = mh::myhtml_tags::MyHTML_TAG_MSCARRIES as u32,
        MSCARRY = mh::myhtml_tags::MyHTML_TAG_MSCARRY as u32,
        MSGROUP = mh::myhtml_tags::MyHTML_TAG_MSGROUP as u32,
        MSLINE = mh::myhtml_tags::MyHTML_TAG_MSLINE as u32,
        MSPACE = mh::myhtml_tags::MyHTML_TAG_MSPACE as u32,
        MSQRT = mh::myhtml_tags::MyHTML_TAG_MSQRT as u32,
        MSROW = mh::myhtml_tags::MyHTML_TAG_MSROW as u32,
        MSTACK = mh::myhtml_tags::MyHTML_TAG_MSTACK as u32,
        MSTYLE = mh::myhtml_tags::MyHTML_TAG_MSTYLE as u32,
        MSUB = mh::myhtml_tags::MyHTML_TAG_MSUB as u32,
        MSUP = mh::myhtml_tags::MyHTML_TAG_MSUP as u32,
        MSUBSUP = mh::myhtml_tags::MyHTML_TAG_MSUBSUP as u32,
        _END_OF_FILE = mh::myhtml_tags::MyHTML_TAG__END_OF_FILE as u32,
        LAST_ENTRY = mh::myhtml_tags::MyHTML_TAG_LAST_ENTRY as u32,
    }

    #[derive(Debug, derive_more::From)]
    pub enum ParserError {
        MyHtmlError { status: u32 },
    }

    #[derive(Debug)]
    pub struct Parser {
        mh_parser: *mut mh::myhtml,
    }

    impl Default for Parser {
        fn default() -> Parser {
            unsafe {
                let mhp = mh::myhtml_create();
                mh::myhtml_init(mhp, mh::myhtml_options::MyHTML_OPTIONS_DEFAULT, 1, 0);
                Parser { mh_parser: mhp }
            }
        }
    }

    macro_rules! try_mh {
        ($x:expr) => {{
            let status = $x;
            if status != mh::mycore_status::MyCORE_STATUS_OK as u32 {
                Err(ParserError::MyHtmlError { status })
            } else {
                Ok(())
            }
        }};
    }

    impl Parser {
        pub fn parse(&mut self, html: impl AsRef<str>) -> Result<Tree, ParserError> {
            unsafe {
                let html_ref = html.as_ref();
                let mht = mh::myhtml_tree_create();
                try_mh!(mh::myhtml_tree_init(mht, self.mh_parser))?;
                let enc = std::mem::transmute(Encoding::Utf8);
                let h = html_ref.as_ptr() as *const std::os::raw::c_char;
                mh::myhtml_parse(mht, enc, h, html_ref.len());

                let t = Tree { mh_tree: mht };

                Ok(t)
            }
        }
    }

    impl Drop for Parser {
        fn drop(&mut self) {
            unsafe {
                mh::myhtml_destroy(self.mh_parser);
            }
        }
    }

    #[derive(Debug)]
    pub struct Tree {
        mh_tree: *mut mh::myhtml_tree_t,
    }

    fn ptr_expect<'a, T>(ptr: *mut T, text: &'a str) {
        if ptr == std::ptr::null_mut() {
            print!("{}", text);
        }
    }

    impl Tree {
        pub fn get<'a>(&'a self, idx: usize) -> Option<Node<'a>> {
            unsafe {
                let mut tmp = mh::myhtml_tree_get_document(self.mh_tree);
                if tmp == std::ptr::null_mut() {
                    return None;
                }
                tmp = mh::myhtml_node_child(tmp);
                if tmp == std::ptr::null_mut() {
                    return None;
                }
                for _ in 0..idx {
                    tmp = mh::myhtml_node_next(tmp);
                    if tmp == std::ptr::null_mut() {
                        return None;
                    }
                }
                return Some(Node::from_raw(tmp));
            }
        }

        pub fn render(&self) -> Result<String, ParserError> {
            unsafe {
                let mut mhstring = std::mem::zeroed::<mh::mycore_string_raw_t>();
                let doc = mh::myhtml_tree_get_document(self.mh_tree);
                try_mh!(mh::myhtml_serialization_tree_buffer(doc, &mut mhstring))?;
                Ok(String::from_raw_parts(
                    mhstring.data as *mut u8,
                    mhstring.length,
                    mhstring.size,
                ))
            }
        }

        pub fn iter<'a>(&'a self) -> TreeIterator<'a> {
            TreeIterator {
                tree: self.mh_tree,
                node: std::ptr::null_mut(),
                state: IterState::Start,
                _lifetime: std::marker::PhantomData,
            }
        }
    }

    impl Drop for Tree {
        fn drop(&mut self) {
            unsafe {
                mh::myhtml_tree_destroy(self.mh_tree);
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Node<'a> {
        node: *mut mh::myhtml_tree_node_t,
        _lifetime: std::marker::PhantomData<&'a ()>,
        pub tag_id: TagType,
    }

    impl<'a> Node<'a> {
        fn from_raw(node: *mut mh::myhtml_tree_node_t) -> Node<'a> {
            unsafe {
                Node {
                    node: node,
                    _lifetime: std::marker::PhantomData,
                    tag_id: std::mem::transmute(mh::myhtml_node_tag_id(node) as u32),
                }
            }
        }

        pub fn get(&self, idx: usize) -> Option<Node<'a>> {
            unsafe {
                let mut tmp = mh::myhtml_node_child(self.node);
                if tmp == std::ptr::null_mut() {
                    return None;
                }
                for _ in 0..idx {
                    tmp = mh::myhtml_node_next(tmp);
                    if tmp == std::ptr::null_mut() {
                        return None;
                    }
                }
                return Some(Node::from_raw(tmp));
            }
        }

        pub fn span(&'a self) -> (usize, usize) {
            unsafe {
                let pos = mh::myhtml_node_element_position(self.node);
                return (pos.begin, pos.length);
            }
        }

        pub fn text(&'a self) -> Option<&'a str> {
            unsafe {
                let mut length = 0;
                let chr_ptr = mh::myhtml_node_text(self.node, &mut length) as *const u8;
                let slice = std::slice::from_raw_parts(chr_ptr, length);
                Some(std::str::from_utf8_unchecked(slice))
            }
        }
    }

    #[derive(Debug)]
    enum IterState {
        Start,
        Dig,
        Leave,
        Next,
        Finished,
    }

    #[derive(Debug)]
    pub struct TreeIterator<'a> {
        tree: *mut mh::myhtml_tree_t,
        node: *mut mh::myhtml_tree_node_t,
        state: IterState,
        _lifetime: std::marker::PhantomData<&'a ()>,
    }

    impl<'a> Iterator for TreeIterator<'a> {
        type Item = Node<'a>;

        fn next(&mut self) -> Option<Self::Item> {
            unsafe {
                loop {
                    let (res, next_state) = match self.state {
                        IterState::Start => {
                            let tmp = mh::myhtml_tree_get_node_html(self.tree);
                            if tmp == std::ptr::null_mut() {
                                (None, IterState::Finished)
                            } else {
                                self.node = tmp;
                                (Some(tmp), IterState::Dig)
                            }
                        }
                        IterState::Dig => {
                            let tmp = mh::myhtml_node_child(self.node);
                            if tmp == std::ptr::null_mut() {
                                (None, IterState::Next)
                            } else {
                                self.node = tmp;
                                (Some(tmp), IterState::Dig)
                            }
                        }
                        IterState::Leave => {
                            let tmp = mh::myhtml_node_parent(self.node);
                            if tmp == std::ptr::null_mut() {
                                (None, IterState::Finished)
                            } else {
                                self.node = tmp;
                                (None, IterState::Next)
                            }
                        }
                        IterState::Next => {
                            let tmp = mh::myhtml_node_next(self.node);
                            if tmp == std::ptr::null_mut() {
                                (None, IterState::Leave)
                            } else {
                                self.node = tmp;
                                (Some(tmp), IterState::Dig)
                            }
                        }
                        IterState::Finished => (None, IterState::Finished),
                    };
                    self.state = next_state;
                    if let IterState::Finished = self.state {
                        return None;
                    }
                    if let Some(res) = res {
                        let n = Node::from_raw(res);
                        return Some(n);
                    }
                }
            }
        }
    }

    pub fn parse(html: impl AsRef<str>) -> Result<Tree, ParserError> {
        let mut parser = Parser::default();
        parser.parse(html)
    }
}

mod test {
    use crate::myhtml;
    use crate::myhtml::TagType;

    #[test]
    fn parsing_invalid_html_produces_valid_html() {
        let html = "<div><span>HTML</span></div>";
        let tree = myhtml::parse(html).expect("can parse");
        let actual = tree.render().expect("can render");
        let expect = "<html><head></head><body><div><span>HTML</span></div></body></html>";

        assert_eq!(actual, expect);
    }

    #[test]
    fn raw_position() {
        let html = "<div><span>HTML</span></div>";
        let tree = myhtml::parse(html).expect("can parse");

        let node = tree
            .get(0)
            .expect("has html")
            .get(1)
            .expect("has body")
            .get(0)
            .expect("has div")
            .get(0)
            .expect("has span");
        dbg!(node.span());
        // assert_eq!(actual, expect);
    }

    #[test]
    fn can_iter_html() {
        let html = "
            1
            <div>
                2
                <span>3</span>
                4
                <span>5</span>
                6
                <span/>
                7
                <div>8</div>
                9
            </div>
            10
            <div>11</div>
            12
            <div/>
            13
        ";
        let tree = myhtml::parse(html).expect("can parse");

        let mut actual = Vec::new();
        for node in tree.iter() {
            actual.push((node.tag_id, node.text().map(|x| x.trim().to_string())));
        }
        let expected = vec![
            (TagType::HTML, None),
            (TagType::HEAD, None),
            (TagType::BODY, None),
            (TagType::_TEXT, Some("1")),
            (TagType::DIV, None),
            (TagType::_TEXT, Some("2")),
            (TagType::SPAN, None),
            (TagType::_TEXT, Some("3")),
            (TagType::_TEXT, Some("4")),
            (TagType::SPAN, None),
            (TagType::_TEXT, Some("5")),
            (TagType::_TEXT, Some("6")),
            (TagType::SPAN, None),
            (TagType::_TEXT, Some("7")),
            (TagType::DIV, None),
            (TagType::_TEXT, Some("8")),
            (TagType::_TEXT, Some("9")),
            (TagType::_TEXT, Some("10")),
            (TagType::DIV, None),
            (TagType::_TEXT, Some("11")),
            (TagType::_TEXT, Some("12")),
            (TagType::DIV, None),
            (TagType::_TEXT, Some("13")),
        ];
        let expected: Vec<_> = expected
            .into_iter()
            .map(|(z, x)| (z, x.map(|y| y.to_string())))
            .collect();
        assert_eq!(actual, expected);
    }
}
