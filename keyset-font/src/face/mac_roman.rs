type PlatformId = rustybuzz::ttf_parser::PlatformId;

// References:
// https://github.com/fonttools/fonttools/issues/236
// https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6name.html

#[allow(dead_code)]
mod mac_encoding_id {
    pub const ROMAN: u16 = 0;
    pub const JAPANESE: u16 = 1;
    pub const CHINESE_TRADITIONAL: u16 = 2;
    pub const KOREAN: u16 = 3;
    pub const ARABIC: u16 = 4;
    pub const HEBREW: u16 = 5;
    pub const GREEK: u16 = 6;
    pub const CYRILLIC: u16 = 7;
    pub const DEVANAGARI: u16 = 9;
    pub const GURMUKHI: u16 = 10;
    pub const GUJARATI: u16 = 11;
    pub const ORIYA: u16 = 12;
    pub const BENGALI: u16 = 13;
    pub const TAMIL: u16 = 14;
    pub const TELUGU: u16 = 15;
    pub const KANNADA: u16 = 16;
    pub const MALAYALAM: u16 = 17;
    pub const SINHALESE: u16 = 18;
    pub const BURMESE: u16 = 19;
    pub const KHMER: u16 = 20;
    pub const THAI: u16 = 21;
    pub const LAOTIAN: u16 = 22;
    pub const GEORGIAN: u16 = 23;
    pub const ARMENIAN: u16 = 24;
    pub const CHINESE_SIMPLIFIED: u16 = 25;
    pub const TIBETAN: u16 = 26;
    pub const MONGOLIAN: u16 = 27;
    pub const ETHIOPIC: u16 = 28;
    pub const CENTRAL_EUROPEAN_ROMAN: u16 = 29;
    pub const VIETNAMESE: u16 = 30;
    pub const EXT_ARABIC: u16 = 31;
    pub const TURKISH: u16 = 35;
    pub const CROATIAN: u16 = 36;
    pub const ICELANDIC: u16 = 37;
    pub const ROMANIAN: u16 = 38;
}

#[allow(dead_code)]
mod mac_language_id {
    pub const ENGLISH: u16 = 0;
    pub const FRENCH: u16 = 1;
    pub const GERMAN: u16 = 2;
    pub const ITALIAN: u16 = 3;
    pub const DUTCH: u16 = 4;
    pub const SWEDISH: u16 = 5;
    pub const SPANISH: u16 = 6;
    pub const DANISH: u16 = 7;
    pub const PORTUGUESE: u16 = 8;
    pub const NORWEGIAN: u16 = 9;
    pub const HEBREW: u16 = 10;
    pub const JAPANESE: u16 = 11;
    pub const ARABIC: u16 = 12;
    pub const FINNISH: u16 = 13;
    pub const GREEK: u16 = 14;
    pub const ICELANDIC: u16 = 15;
    pub const MALTESE: u16 = 16;
    pub const TURKISH: u16 = 17;
    pub const CROATIAN: u16 = 18;
    pub const CHINESE_TRADITIONAL: u16 = 19;
    pub const URDU: u16 = 20;
    pub const HINDI: u16 = 21;
    pub const THAI: u16 = 22;
    pub const KOREAN: u16 = 23;
    pub const LITHUANIAN: u16 = 24;
    pub const POLISH: u16 = 25;
    pub const HUNGARIAN: u16 = 26;
    pub const ESTONIAN: u16 = 27;
    pub const LATVIAN: u16 = 28;
    pub const SAMI: u16 = 29;
    pub const FAROESE: u16 = 30;
    pub const FARSI_PERSIAN: u16 = 31;
    pub const RUSSIAN: u16 = 32;
    pub const CHINESE_SIMPLIFIED: u16 = 33;
    pub const FLEMISH: u16 = 34;
    pub const IRISH_GAELIC: u16 = 35;
    pub const ALBANIAN: u16 = 36;
    pub const ROMANIAN: u16 = 37;
    pub const CZECH: u16 = 38;
    pub const SLOVAK: u16 = 39;
    pub const SLOVENIAN: u16 = 40;
    pub const MALAY_ROMAN: u16 = 83;
    pub const MALAY_ARABIC: u16 = 84;
    pub const AMHARIC: u16 = 85;
    pub const TIGRINYA: u16 = 86;
    pub const GALLA: u16 = 87;
    pub const SOMALI: u16 = 88;
    pub const SWAHILI: u16 = 89;
    pub const KINYARWANDA_RUANDA: u16 = 90;
    pub const RUNDI: u16 = 91;
    pub const NYANJA_CHEWA: u16 = 92;
    pub const MALAGASY: u16 = 93;
    pub const ESPERANTO: u16 = 94;
    pub const WELSH: u16 = 128;
    pub const BASQUE: u16 = 129;
    pub const CATALAN: u16 = 130;
    pub const LATIN: u16 = 131;
    pub const QUECHUA: u16 = 132;
    pub const GUARANI: u16 = 133;
    pub const AYMARA: u16 = 134;
    pub const TATAR: u16 = 135;
    pub const UIGHUR: u16 = 136;
    pub const DZONGKHA: u16 = 137;
    pub const JAVANESE_ROMAN: u16 = 138;
    pub const SUNDANESE_ROMAN: u16 = 139;
    pub const GALICIAN: u16 = 140;
    pub const AFRIKAANS: u16 = 141;
    pub const BRETON: u16 = 142;
    pub const INUKTITUT: u16 = 143;
    pub const SCOTTISH_GAELIC: u16 = 144;
    pub const MANX_GAELIC: u16 = 145;
    pub const IRISH_GAELIC_DOT: u16 = 146;
    pub const TONGAN: u16 = 147;
    pub const GREEK_POLYTONIC: u16 = 148;
    pub const GREENLANDIC: u16 = 149;
    pub const AZERBAIJANI_ROMAN: u16 = 150;
}

const ASCII_MAX: u8 = 0x7F;

const MAC_ROMAN_MAPPING: [char; (u8::MAX - ASCII_MAX) as usize] = [
    '\u{00C4}', '\u{00C5}', '\u{00C7}', '\u{00C9}', '\u{00D1}', '\u{00D6}', '\u{00DC}', '\u{00E1}',
    '\u{00E0}', '\u{00E2}', '\u{00E4}', '\u{00E3}', '\u{00E5}', '\u{00E7}', '\u{00E9}', '\u{00E8}',
    '\u{00EA}', '\u{00EB}', '\u{00ED}', '\u{00EC}', '\u{00EE}', '\u{00EF}', '\u{00F1}', '\u{00F3}',
    '\u{00F2}', '\u{00F4}', '\u{00F6}', '\u{00F5}', '\u{00FA}', '\u{00F9}', '\u{00FB}', '\u{00FC}',
    '\u{2020}', '\u{00B0}', '\u{00A2}', '\u{00A3}', '\u{00A7}', '\u{2022}', '\u{00B6}', '\u{00DF}',
    '\u{00AE}', '\u{00A9}', '\u{2122}', '\u{00B4}', '\u{00A8}', '\u{2260}', '\u{00C6}', '\u{00D8}',
    '\u{221E}', '\u{00B1}', '\u{2264}', '\u{2265}', '\u{00A5}', '\u{00B5}', '\u{2202}', '\u{2211}',
    '\u{220F}', '\u{03C0}', '\u{222B}', '\u{00AA}', '\u{00BA}', '\u{03A9}', '\u{00E6}', '\u{00F8}',
    '\u{00BF}', '\u{00A1}', '\u{00AC}', '\u{221A}', '\u{0192}', '\u{2248}', '\u{2206}', '\u{00AB}',
    '\u{00BB}', '\u{2026}', '\u{00A0}', '\u{00C0}', '\u{00C3}', '\u{00D5}', '\u{0152}', '\u{0153}',
    '\u{2013}', '\u{2014}', '\u{201C}', '\u{201D}', '\u{2018}', '\u{2019}', '\u{00F7}', '\u{25CA}',
    '\u{00FF}', '\u{0178}', '\u{2044}', '\u{20AC}', '\u{2039}', '\u{203A}', '\u{FB01}', '\u{FB02}',
    '\u{2021}', '\u{00B7}', '\u{201A}', '\u{201E}', '\u{2030}', '\u{00C2}', '\u{00CA}', '\u{00C1}',
    '\u{00CB}', '\u{00C8}', '\u{00CD}', '\u{00CE}', '\u{00CF}', '\u{00CC}', '\u{00D3}', '\u{00D4}',
    '\u{F8FF}', '\u{00D2}', '\u{00DA}', '\u{00DB}', '\u{00D9}', '\u{0131}', '\u{02C6}', '\u{02DC}',
    '\u{00AF}', '\u{02D8}', '\u{02D9}', '\u{02DA}', '\u{00B8}', '\u{02DD}', '\u{02DB}', '\u{02C7}',
];

pub fn is_mac_roman_encoding(platform_id: PlatformId, encoding_id: u16, language_id: u16) -> bool {
    use {mac_encoding_id as enc, mac_language_id as lang};

    // If encoding_id == roman, the below languages use modified mac-roman encodings. All others
    // languages use either standard mac-roman or encoding_id != roman (in which case we default to
    // mac-roman).
    platform_id == PlatformId::Macintosh
        && encoding_id == enc::ROMAN
        && ![
            lang::ICELANDIC,
            lang::TURKISH,
            lang::CROATIAN,
            lang::LITHUANIAN,
            lang::POLISH,
            lang::HUNGARIAN,
            lang::ESTONIAN,
            lang::LATVIAN,
            lang::ALBANIAN,
            lang::ROMANIAN,
            lang::CZECH,
            lang::SLOVAK,
            lang::SLOVENIAN,
        ]
        .contains(&language_id)
}

pub fn mac_roman_decode(chars: &[u8]) -> String {
    chars
        .iter()
        .map(|&b| match b {
            0..=ASCII_MAX => b as char,
            _ => MAC_ROMAN_MAPPING[usize::from(b - (ASCII_MAX + 1))],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_mac_roman_encoding() {
        use rustybuzz::ttf_parser::PlatformId as Plat;
        use {mac_encoding_id as enc, mac_language_id as lang};

        assert!(!is_mac_roman_encoding(
            Plat::Unicode, // Wrong PlatformId
            enc::ROMAN,
            lang::ENGLISH,
        ));
        assert!(!is_mac_roman_encoding(
            Plat::Macintosh,
            enc::JAPANESE, // Wrong EncodingId
            lang::ENGLISH,
        ));
        assert!(!is_mac_roman_encoding(
            Plat::Macintosh,
            u16::MAX, // Invalid EncodingId
            lang::ENGLISH,
        ));
        assert!(!is_mac_roman_encoding(
            Plat::Macintosh,
            enc::ROMAN,
            lang::ICELANDIC, // Wrong LanguageId
        ));
        assert!(is_mac_roman_encoding(
            Plat::Macintosh,
            enc::ROMAN,
            u16::MAX, // Invalid LanguageId => default to mac-roman
        ));
        assert!(is_mac_roman_encoding(
            Plat::Macintosh,
            enc::ROMAN,
            lang::ENGLISH,
        ));
    }

    #[test]
    fn test_mac_roman_decode() {
        // Test data borrowed from https://www.unicode.org/Public/MAPPINGS/VENDORS/APPLE/ROMAN.TXT

        let encoded: Vec<_> = (0..=u8::MAX).collect();
        let expected = "\
            \u{0000}\u{0001}\u{0002}\u{0003}\u{0004}\u{0005}\u{0006}\u{0007}\
            \u{0008}\u{0009}\u{000A}\u{000B}\u{000C}\u{000D}\u{000E}\u{000F}\
            \u{0010}\u{0011}\u{0012}\u{0013}\u{0014}\u{0015}\u{0016}\u{0017}\
            \u{0018}\u{0019}\u{001A}\u{001B}\u{001C}\u{001D}\u{001E}\u{001F}\
            \u{0020}\u{0021}\u{0022}\u{0023}\u{0024}\u{0025}\u{0026}\u{0027}\
            \u{0028}\u{0029}\u{002A}\u{002B}\u{002C}\u{002D}\u{002E}\u{002F}\
            \u{0030}\u{0031}\u{0032}\u{0033}\u{0034}\u{0035}\u{0036}\u{0037}\
            \u{0038}\u{0039}\u{003A}\u{003B}\u{003C}\u{003D}\u{003E}\u{003F}\
            \u{0040}\u{0041}\u{0042}\u{0043}\u{0044}\u{0045}\u{0046}\u{0047}\
            \u{0048}\u{0049}\u{004A}\u{004B}\u{004C}\u{004D}\u{004E}\u{004F}\
            \u{0050}\u{0051}\u{0052}\u{0053}\u{0054}\u{0055}\u{0056}\u{0057}\
            \u{0058}\u{0059}\u{005A}\u{005B}\u{005C}\u{005D}\u{005E}\u{005F}\
            \u{0060}\u{0061}\u{0062}\u{0063}\u{0064}\u{0065}\u{0066}\u{0067}\
            \u{0068}\u{0069}\u{006A}\u{006B}\u{006C}\u{006D}\u{006E}\u{006F}\
            \u{0070}\u{0071}\u{0072}\u{0073}\u{0074}\u{0075}\u{0076}\u{0077}\
            \u{0078}\u{0079}\u{007A}\u{007B}\u{007C}\u{007D}\u{007E}\u{007F}\
            \u{00C4}\u{00C5}\u{00C7}\u{00C9}\u{00D1}\u{00D6}\u{00DC}\u{00E1}\
            \u{00E0}\u{00E2}\u{00E4}\u{00E3}\u{00E5}\u{00E7}\u{00E9}\u{00E8}\
            \u{00EA}\u{00EB}\u{00ED}\u{00EC}\u{00EE}\u{00EF}\u{00F1}\u{00F3}\
            \u{00F2}\u{00F4}\u{00F6}\u{00F5}\u{00FA}\u{00F9}\u{00FB}\u{00FC}\
            \u{2020}\u{00B0}\u{00A2}\u{00A3}\u{00A7}\u{2022}\u{00B6}\u{00DF}\
            \u{00AE}\u{00A9}\u{2122}\u{00B4}\u{00A8}\u{2260}\u{00C6}\u{00D8}\
            \u{221E}\u{00B1}\u{2264}\u{2265}\u{00A5}\u{00B5}\u{2202}\u{2211}\
            \u{220F}\u{03C0}\u{222B}\u{00AA}\u{00BA}\u{03A9}\u{00E6}\u{00F8}\
            \u{00BF}\u{00A1}\u{00AC}\u{221A}\u{0192}\u{2248}\u{2206}\u{00AB}\
            \u{00BB}\u{2026}\u{00A0}\u{00C0}\u{00C3}\u{00D5}\u{0152}\u{0153}\
            \u{2013}\u{2014}\u{201C}\u{201D}\u{2018}\u{2019}\u{00F7}\u{25CA}\
            \u{00FF}\u{0178}\u{2044}\u{20AC}\u{2039}\u{203A}\u{FB01}\u{FB02}\
            \u{2021}\u{00B7}\u{201A}\u{201E}\u{2030}\u{00C2}\u{00CA}\u{00C1}\
            \u{00CB}\u{00C8}\u{00CD}\u{00CE}\u{00CF}\u{00CC}\u{00D3}\u{00D4}\
            \u{F8FF}\u{00D2}\u{00DA}\u{00DB}\u{00D9}\u{0131}\u{02C6}\u{02DC}\
            \u{00AF}\u{02D8}\u{02D9}\u{02DA}\u{00B8}\u{02DD}\u{02DB}\u{02C7}\
        ";

        assert_eq!(mac_roman_decode(&encoded), expected);
    }
}
