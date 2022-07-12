use crate::types::IconPair;

#[allow(dead_code)]
pub const ICON_DIR: &str = "\u{F74A}";
pub const ICON_GEAR: &str = "\u{F013}";

pub const ICON_APK: &str = "\u{E70E}";
pub const ICON_EXEC: &str = &ICON_GEAR;
pub const ICON_SHELL: &str = "\u{F489}";
pub const ICON_C: &str = "\u{E61E}";
pub const ICON_CPP: &str = "\u{E61D}";
pub const ICON_CS: &str = "\u{F81A}";
pub const ICON_CSS: &str = "\u{E749}";
pub const ICON_FSHARP: &str = "\u{E7A7}";
pub const ICON_XML: &str = "\u{F81A}";
pub const ICON_RB: &str = "\u{E739}";
pub const ICON_LUA: &str = "\u{E620}";
pub const ICON_GIT: &str = "\u{E702}";
pub const ICON_GO: &str = "\u{E626}";
pub const ICON_HTML: &str = "\u{E736}";
pub const ICON_JAVA: &str = "\u{E738}";
pub const ICON_JS: &str = "\u{E781}";
pub const ICON_JSON: &str = "\u{E60B}";
pub const ICON_PY: &str = "\u{E73C}";
pub const ICON_SCALA: &str = "\u{E737}";
pub const ICON_MD: &str = "\u{E609}";
pub const ICON_VIM: &str = "\u{E62B}";

pub const ICON_AUDIO: &str = "\u{F722}";
pub const ICON_VIDEO: &str = "\u{F72A}";
pub const ICON_IMAGE: &str = "\u{F1C5}";
pub const ICON_GENERIC: &str = "\u{F15B}";
pub const ICON_ARCHIVE: &str = "\u{F1C6}";

const ICONS: [IconPair; 162] = [
    IconPair { ext: "1", icon: ICON_ARCHIVE },
    IconPair { ext: "3g2", icon: ICON_VIDEO },
    IconPair { ext: "3gp", icon: ICON_VIDEO },
    IconPair { ext: "7z", icon: ICON_ARCHIVE },
    IconPair { ext: "7zip", icon: ICON_ARCHIVE },
    /* A */
    IconPair { ext: "a", icon: ICON_ARCHIVE },
    IconPair { ext: "aac", icon: ICON_AUDIO },
    IconPair { ext: "ac3", icon: ICON_AUDIO },
    IconPair { ext: "ai", icon: ICON_IMAGE },
    IconPair { ext: "aif", icon: ICON_AUDIO },
    IconPair { ext: "alz", icon: ICON_ARCHIVE },
    IconPair { ext: "amv", icon: ICON_VIDEO },
    IconPair { ext: "apk", icon: ICON_APK },
    IconPair { ext: "asec", icon: ICON_APK },
    IconPair { ext: "asf", icon: ICON_VIDEO },
    IconPair { ext: "asm", icon: ICON_GEAR },
    IconPair { ext: "avi", icon: ICON_VIDEO },
    /* B */
    IconPair { ext: "bash", icon: ICON_SHELL },
    IconPair { ext: "bin", icon: ICON_EXEC },
    IconPair { ext: "bmp", icon: ICON_IMAGE },
    IconPair { ext: "bz2", icon: ICON_ARCHIVE },
    /* C */
    IconPair { ext: "c", icon: ICON_C },
    IconPair { ext: "c++", icon: ICON_CPP },
    IconPair { ext: "cc", icon: ICON_CPP },
    IconPair { ext: "cbr", icon: ICON_ARCHIVE },
    IconPair { ext: "cda", icon: ICON_AUDIO },
    IconPair { ext: "class", icon: ICON_JAVA },
    IconPair { ext: "cpgz", icon: ICON_ARCHIVE },
    IconPair { ext: "cs", icon: ICON_CS },
    IconPair { ext: "cso", icon: ICON_ARCHIVE },
    IconPair { ext: "css", icon: ICON_CSS },
    /* D */
    IconPair { ext: "dar", icon: ICON_ARCHIVE },
    IconPair { ext: "dbz", icon: ICON_ARCHIVE },
    IconPair { ext: "deb", icon: ICON_ARCHIVE },
    IconPair { ext: "drc", icon: ICON_VIDEO },
    IconPair { ext: "dz", icon: ICON_ARCHIVE },
    /* E */
    IconPair { ext: "ear", icon: ICON_ARCHIVE },
    IconPair { ext: "eps", icon: ICON_ARCHIVE },
    /* F */
    IconPair { ext: "f#", icon: ICON_FSHARP },
    IconPair { ext: "f4a", icon: ICON_AUDIO },
    IconPair { ext: "f4b", icon: ICON_AUDIO },
    IconPair { ext: "f4p", icon: ICON_VIDEO },
    IconPair { ext: "f4v", icon: ICON_VIDEO },
    IconPair { ext: "flv", icon: ICON_VIDEO },
    IconPair { ext: "fish", icon: ICON_SHELL },
    /* G */
    IconPair { ext: "gif", icon: ICON_IMAGE },
    IconPair { ext: "gifv", icon: ICON_VIDEO },
    IconPair { ext: "gip", icon: ICON_ARCHIVE },
    IconPair { ext: "git", icon: ICON_GIT },
    IconPair { ext: "go", icon: ICON_GO },
    IconPair { ext: "gz", icon: ICON_ARCHIVE },
    /* H */
    IconPair { ext: "h", icon: ICON_C },
    IconPair { ext: "h264", icon: ICON_VIDEO },
    IconPair { ext: "heif", icon: ICON_IMAGE },
    IconPair { ext: "hh", icon: ICON_CPP },
    IconPair { ext: "hpp", icon: ICON_CPP },
    IconPair { ext: "htm", icon: ICON_HTML },
    IconPair { ext: "html", icon: ICON_HTML },
    IconPair { ext: "htmlz", icon: ICON_ARCHIVE },
    /* I */
    IconPair { ext: "ico", icon: ICON_IMAGE },
    IconPair { ext: "igz", icon: ICON_ARCHIVE },
    IconPair { ext: "ipa", icon: ICON_ARCHIVE },
    /* J */
    IconPair { ext: "jar", icon: ICON_JAVA },
    IconPair { ext: "java", icon: ICON_JAVA },
    IconPair { ext: "jpeg", icon: ICON_IMAGE },
    IconPair { ext: "jpg", icon: ICON_IMAGE },
    IconPair { ext: "js", icon: ICON_JS },
    IconPair { ext: "json", icon: ICON_JSON },
    /* K */
    /* L */
    IconPair { ext: "lua", icon: ICON_LUA },
    IconPair { ext: "lz", icon: ICON_ARCHIVE },
    IconPair { ext: "lz4", icon: ICON_ARCHIVE },
    /* M */
    IconPair { ext: "m2ts", icon: ICON_VIDEO },
    IconPair { ext: "m2v", icon: ICON_VIDEO },
    IconPair { ext: "m4a", icon: ICON_AUDIO },
    IconPair { ext: "m4p", icon: ICON_AUDIO },
    IconPair { ext: "m4v", icon: ICON_VIDEO },
    IconPair { ext: "maff", icon: ICON_ARCHIVE },
    IconPair { ext: "md", icon: ICON_MD },
    IconPair { ext: "mid", icon: ICON_AUDIO },
    IconPair { ext: "midi", icon: ICON_AUDIO },
    IconPair { ext: "mkv", icon: ICON_VIDEO },
    IconPair { ext: "mng", icon: ICON_IMAGE },
    IconPair { ext: "mog", icon: ICON_VIDEO },
    IconPair { ext: "mp3", icon: ICON_AUDIO },
    IconPair { ext: "mp4", icon: ICON_VIDEO },
    IconPair { ext: "mpeg", icon: ICON_VIDEO },
    IconPair { ext: "mpg", icon: ICON_VIDEO },
    IconPair { ext: "mpq", icon: ICON_ARCHIVE },
    IconPair { ext: "mts", icon: ICON_VIDEO },
    IconPair { ext: "mxf", icon: ICON_VIDEO },
    /* N */
    IconPair {},
    IconPair {},
    IconPair {},
    /* O */
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    /* P */
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    /* Q */
    IconPair {},
    /* R */
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    /* S */
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    /* T */
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    /* U */
    IconPair {},
    /* V */
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    /* W */
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    /* X */
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    /* Y */
    IconPair {},
    /* Z */
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
    IconPair {},
];