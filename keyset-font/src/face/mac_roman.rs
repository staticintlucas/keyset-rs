type PlatformId = rustybuzz::ttf_parser::PlatformId;

// References:
// https://github.com/fonttools/fonttools/issues/236
// https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6name.html

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum MacEncodingId {
    Roman = 0,
    Japanese = 1,
    ChineseTraditional = 2,
    Korean = 3,
    Arabic = 4,
    Hebrew = 5,
    Greek = 6,
    Cyrillic = 7,
    Devanagari = 9,
    Gurmukhi = 10,
    Gujarati = 11,
    Oriya = 12,
    Bengali = 13,
    Tamil = 14,
    Telugu = 15,
    Kannada = 16,
    Malayalam = 17,
    Sinhalese = 18,
    Burmese = 19,
    Khmer = 20,
    Thai = 21,
    Laotian = 22,
    Georgian = 23,
    Armenian = 24,
    ChineseSimplified = 25,
    Tibetan = 26,
    Mongolian = 27,
    Ethiopic = 28,
    CentralEuropeanRoman = 29,
    Vietnamese = 30,
    ExtArabic = 31,
    Turkish = 35,
    Croatian = 36,
    Icelandic = 37,
    Romanian = 38,
}

impl From<MacEncodingId> for u16 {
    fn from(value: MacEncodingId) -> Self {
        value as Self
    }
}

impl TryFrom<u16> for MacEncodingId {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            // GRCOV_EXCL_START // We cover some of these lines but not all
            0 => Ok(Self::Roman),
            1 => Ok(Self::Japanese),
            2 => Ok(Self::ChineseTraditional),
            3 => Ok(Self::Korean),
            4 => Ok(Self::Arabic),
            5 => Ok(Self::Hebrew),
            6 => Ok(Self::Greek),
            7 => Ok(Self::Cyrillic),
            9 => Ok(Self::Devanagari),
            10 => Ok(Self::Gurmukhi),
            11 => Ok(Self::Gujarati),
            12 => Ok(Self::Oriya),
            13 => Ok(Self::Bengali),
            14 => Ok(Self::Tamil),
            15 => Ok(Self::Telugu),
            16 => Ok(Self::Kannada),
            17 => Ok(Self::Malayalam),
            18 => Ok(Self::Sinhalese),
            19 => Ok(Self::Burmese),
            20 => Ok(Self::Khmer),
            21 => Ok(Self::Thai),
            22 => Ok(Self::Laotian),
            23 => Ok(Self::Georgian),
            24 => Ok(Self::Armenian),
            25 => Ok(Self::ChineseSimplified),
            26 => Ok(Self::Tibetan),
            27 => Ok(Self::Mongolian),
            28 => Ok(Self::Ethiopic),
            29 => Ok(Self::CentralEuropeanRoman),
            30 => Ok(Self::Vietnamese),
            31 => Ok(Self::ExtArabic),
            35 => Ok(Self::Turkish),
            36 => Ok(Self::Croatian),
            37 => Ok(Self::Icelandic),
            38 => Ok(Self::Romanian),
            _ => Err(()),
            // GRCOV_EXCL_STOP
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum MacLanguageId {
    English = 0,
    French = 1,
    German = 2,
    Italian = 3,
    Dutch = 4,
    Swedish = 5,
    Spanish = 6,
    Danish = 7,
    Portuguese = 8,
    Norwegian = 9,
    Hebrew = 10,
    Japanese = 11,
    Arabic = 12,
    Finnish = 13,
    Greek = 14,
    Icelandic = 15,
    Maltese = 16,
    Turkish = 17,
    Croatian = 18,
    ChineseTraditional = 19,
    Urdu = 20,
    Hindi = 21,
    Thai = 22,
    Korean = 23,
    Lithuanian = 24,
    Polish = 25,
    Hungarian = 26,
    Estonian = 27,
    Latvian = 28,
    Sami = 29,
    Faroese = 30,
    FarsiPersian = 31,
    Russian = 32,
    ChineseSimplified = 33,
    Flemish = 34,
    IrishGaelic = 35,
    Albanian = 36,
    Romanian = 37,
    Czech = 38,
    Slovak = 39,
    Slovenian = 40,
    Yiddish = 41,
    Serbian = 42,
    Macedonian = 43,
    Bulgarian = 44,
    Ukrainian = 45,
    Byelorussian = 46,
    Uzbek = 47,
    Kazakh = 48,
    AzerbaijaniCyrillic = 49,
    AzerbaijaniArabic = 50,
    Armenian = 51,
    Georgian = 52,
    Moldavian = 53,
    Kirghiz = 54,
    Tajiki = 55,
    Turkmen = 56,
    MongolianMongolian = 57,
    MongolianCyrillic = 58,
    Pashto = 59,
    Kurdish = 60,
    Kashmiri = 61,
    Sindhi = 62,
    Tibetan = 63,
    Nepali = 64,
    Sanskrit = 65,
    Marathi = 66,
    Bengali = 67,
    Assamese = 68,
    Gujarati = 69,
    Punjabi = 70,
    Oriya = 71,
    Malayalam = 72,
    Kannada = 73,
    Tamil = 74,
    Telugu = 75,
    Sinhalese = 76,
    Burmese = 77,
    Khmer = 78,
    Lao = 79,
    Vietnamese = 80,
    Indonesian = 81,
    Tagalog = 82,
    MalayRoman = 83,
    MalayArabic = 84,
    Amharic = 85,
    Tigrinya = 86,
    Galla = 87,
    Somali = 88,
    Swahili = 89,
    KinyarwandaRuanda = 90,
    Rundi = 91,
    NyanjaChewa = 92,
    Malagasy = 93,
    Esperanto = 94,
    Welsh = 128,
    Basque = 129,
    Catalan = 130,
    Latin = 131,
    Quechua = 132,
    Guarani = 133,
    Aymara = 134,
    Tatar = 135,
    Uighur = 136,
    Dzongkha = 137,
    JavaneseRoman = 138,
    SundaneseRoman = 139,
    Galician = 140,
    Afrikaans = 141,
    Breton = 142,
    Inuktitut = 143,
    ScottishGaelic = 144,
    ManxGaelic = 145,
    IrishGaelicDot = 146,
    Tongan = 147,
    GreekPolytonic = 148,
    Greenlandic = 149,
    AzerbaijaniRoman = 150,
}

impl From<MacLanguageId> for u16 {
    fn from(value: MacLanguageId) -> Self {
        value as Self
    }
}

impl TryFrom<u16> for MacLanguageId {
    type Error = ();

    #[allow(clippy::too_many_lines)]
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            // GRCOV_EXCL_START // We cover some of these lines but not all
            0 => Ok(Self::English),
            1 => Ok(Self::French),
            2 => Ok(Self::German),
            3 => Ok(Self::Italian),
            4 => Ok(Self::Dutch),
            5 => Ok(Self::Swedish),
            6 => Ok(Self::Spanish),
            7 => Ok(Self::Danish),
            8 => Ok(Self::Portuguese),
            9 => Ok(Self::Norwegian),
            10 => Ok(Self::Hebrew),
            11 => Ok(Self::Japanese),
            12 => Ok(Self::Arabic),
            13 => Ok(Self::Finnish),
            14 => Ok(Self::Greek),
            15 => Ok(Self::Icelandic),
            16 => Ok(Self::Maltese),
            17 => Ok(Self::Turkish),
            18 => Ok(Self::Croatian),
            19 => Ok(Self::ChineseTraditional),
            20 => Ok(Self::Urdu),
            21 => Ok(Self::Hindi),
            22 => Ok(Self::Thai),
            23 => Ok(Self::Korean),
            24 => Ok(Self::Lithuanian),
            25 => Ok(Self::Polish),
            26 => Ok(Self::Hungarian),
            27 => Ok(Self::Estonian),
            28 => Ok(Self::Latvian),
            29 => Ok(Self::Sami),
            30 => Ok(Self::Faroese),
            31 => Ok(Self::FarsiPersian),
            32 => Ok(Self::Russian),
            33 => Ok(Self::ChineseSimplified),
            34 => Ok(Self::Flemish),
            35 => Ok(Self::IrishGaelic),
            36 => Ok(Self::Albanian),
            37 => Ok(Self::Romanian),
            38 => Ok(Self::Czech),
            39 => Ok(Self::Slovak),
            40 => Ok(Self::Slovenian),
            41 => Ok(Self::Yiddish),
            42 => Ok(Self::Serbian),
            43 => Ok(Self::Macedonian),
            44 => Ok(Self::Bulgarian),
            45 => Ok(Self::Ukrainian),
            46 => Ok(Self::Byelorussian),
            47 => Ok(Self::Uzbek),
            48 => Ok(Self::Kazakh),
            49 => Ok(Self::AzerbaijaniCyrillic),
            50 => Ok(Self::AzerbaijaniArabic),
            51 => Ok(Self::Armenian),
            52 => Ok(Self::Georgian),
            53 => Ok(Self::Moldavian),
            54 => Ok(Self::Kirghiz),
            55 => Ok(Self::Tajiki),
            56 => Ok(Self::Turkmen),
            57 => Ok(Self::MongolianMongolian),
            58 => Ok(Self::MongolianCyrillic),
            59 => Ok(Self::Pashto),
            60 => Ok(Self::Kurdish),
            61 => Ok(Self::Kashmiri),
            62 => Ok(Self::Sindhi),
            63 => Ok(Self::Tibetan),
            64 => Ok(Self::Nepali),
            65 => Ok(Self::Sanskrit),
            66 => Ok(Self::Marathi),
            67 => Ok(Self::Bengali),
            68 => Ok(Self::Assamese),
            69 => Ok(Self::Gujarati),
            70 => Ok(Self::Punjabi),
            71 => Ok(Self::Oriya),
            72 => Ok(Self::Malayalam),
            73 => Ok(Self::Kannada),
            74 => Ok(Self::Tamil),
            75 => Ok(Self::Telugu),
            76 => Ok(Self::Sinhalese),
            77 => Ok(Self::Burmese),
            78 => Ok(Self::Khmer),
            79 => Ok(Self::Lao),
            80 => Ok(Self::Vietnamese),
            81 => Ok(Self::Indonesian),
            82 => Ok(Self::Tagalog),
            83 => Ok(Self::MalayRoman),
            84 => Ok(Self::MalayArabic),
            85 => Ok(Self::Amharic),
            86 => Ok(Self::Tigrinya),
            87 => Ok(Self::Galla),
            88 => Ok(Self::Somali),
            89 => Ok(Self::Swahili),
            90 => Ok(Self::KinyarwandaRuanda),
            91 => Ok(Self::Rundi),
            92 => Ok(Self::NyanjaChewa),
            93 => Ok(Self::Malagasy),
            94 => Ok(Self::Esperanto),
            128 => Ok(Self::Welsh),
            129 => Ok(Self::Basque),
            130 => Ok(Self::Catalan),
            131 => Ok(Self::Latin),
            132 => Ok(Self::Quechua),
            133 => Ok(Self::Guarani),
            134 => Ok(Self::Aymara),
            135 => Ok(Self::Tatar),
            136 => Ok(Self::Uighur),
            137 => Ok(Self::Dzongkha),
            138 => Ok(Self::JavaneseRoman),
            139 => Ok(Self::SundaneseRoman),
            140 => Ok(Self::Galician),
            141 => Ok(Self::Afrikaans),
            142 => Ok(Self::Breton),
            143 => Ok(Self::Inuktitut),
            144 => Ok(Self::ScottishGaelic),
            145 => Ok(Self::ManxGaelic),
            146 => Ok(Self::IrishGaelicDot),
            147 => Ok(Self::Tongan),
            148 => Ok(Self::GreekPolytonic),
            149 => Ok(Self::Greenlandic),
            150 => Ok(Self::AzerbaijaniRoman),
            _ => Err(()),
            // GRCOV_EXCL_STOP
        }
    }
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
    let Ok(encoding_id) = encoding_id.try_into() else {
        return false;
    };
    let Ok(language_id) = language_id.try_into() else {
        return false;
    };

    matches!(platform_id, PlatformId::Macintosh)
        && matches!(encoding_id, MacEncodingId::Roman)
        && !matches!(
            language_id,
            MacLanguageId::Icelandic
                | MacLanguageId::Turkish
                | MacLanguageId::Croatian
                | MacLanguageId::Lithuanian
                | MacLanguageId::Polish
                | MacLanguageId::Hungarian
                | MacLanguageId::Estonian
                | MacLanguageId::Latvian
                | MacLanguageId::Albanian
                | MacLanguageId::Romanian
                | MacLanguageId::Czech
                | MacLanguageId::Slovak
                | MacLanguageId::Slovenian
        )
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
    fn mac_encoding_id_into() {
        use MacEncodingId::*;

        assert_eq!(u16::from(Roman), 0);
        assert_eq!(u16::from(Japanese), 1);
        assert_eq!(u16::from(Thai), 21);
        assert_eq!(u16::from(CentralEuropeanRoman), 29);
    }

    #[test]
    fn mac_encoding_id_from() {
        use MacEncodingId::*;

        assert_eq!(MacEncodingId::try_from(0).unwrap(), Roman);
        assert_eq!(MacEncodingId::try_from(1).unwrap(), Japanese);
        assert_eq!(MacEncodingId::try_from(21).unwrap(), Thai);
        assert_eq!(MacEncodingId::try_from(29).unwrap(), CentralEuropeanRoman);
        assert!(MacEncodingId::try_from(8).is_err());
        assert!(MacEncodingId::try_from(33).is_err());
        assert!(MacEncodingId::try_from(40).is_err());
    }

    #[test]
    fn mac_language_id_into() {
        use MacLanguageId::*;

        assert_eq!(u16::from(English), 0);
        assert_eq!(u16::from(Japanese), 11);
        assert_eq!(u16::from(Thai), 22);
        assert_eq!(u16::from(IrishGaelic), 35);
    }

    #[test]
    fn mac_language_id_from() {
        use MacLanguageId::*;

        assert_eq!(MacLanguageId::try_from(0).unwrap(), English);
        assert_eq!(MacLanguageId::try_from(11).unwrap(), Japanese);
        assert_eq!(MacLanguageId::try_from(22).unwrap(), Thai);
        assert_eq!(MacLanguageId::try_from(35).unwrap(), IrishGaelic);
        assert!(MacLanguageId::try_from(99).is_err());
        assert!(MacLanguageId::try_from(120).is_err());
        assert!(MacLanguageId::try_from(155).is_err());
    }

    #[test]
    fn test_is_mac_roman_encoding() {
        use rustybuzz::ttf_parser::PlatformId as Plat;
        use MacEncodingId as Enc;
        use MacLanguageId as Lang;

        assert!(!is_mac_roman_encoding(
            Plat::Unicode, // Wrong PlatformId
            Enc::Roman.into(),
            Lang::English.into(),
        ));
        assert!(!is_mac_roman_encoding(
            Plat::Macintosh,
            Enc::Japanese.into(), // Wrong EncodingId
            Lang::English.into(),
        ));
        assert!(!is_mac_roman_encoding(
            Plat::Macintosh,
            u16::MAX, // Invalid EncodingId
            Lang::English.into(),
        ));
        assert!(!is_mac_roman_encoding(
            Plat::Macintosh,
            Enc::Roman.into(),
            Lang::Icelandic.into(), // Wrong LanguageId
        ));
        assert!(!is_mac_roman_encoding(
            Plat::Macintosh,
            Enc::Roman.into(),
            u16::MAX, // Invalid LanguageId
        ));
        assert!(is_mac_roman_encoding(
            Plat::Macintosh,
            Enc::Roman.into(),
            Lang::English.into(),
        ));
    }

    #[test]
    fn test_mac_roman_decode() {
        // Test data borrowed from https://github.com/SolraBizna/macroman-encode

        let encoded: Vec<_> = (0..=u8::MAX).collect();
        let expected = "\
            \0\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0b\x0c\r\x0e\x0f\
            \x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f\
            \x20!\"#$%&'()*+,-./\
            0123456789:;<=>?\
            @ABCDEFGHIJKLMNO\
            PQRSTUVWXYZ[\\]^_\
            `abcdefghijklmno\
            pqrstuvwxyz{|}~\x7f\
            ÄÅÇÉÑÖÜáàâäãåçéè\
            êëíìîïñóòôöõúùûü\
            †°¢£§•¶ß®©™´¨≠ÆØ\
            ∞±≤≥¥µ∂∑∏π∫ªºΩæø\
            ¿¡¬√ƒ≈∆«»…\u{A0}ÀÃÕŒœ\
            –—“”‘’÷◊ÿŸ⁄€‹›ﬁﬂ\
            ‡·‚„‰ÂÊÁËÈÍÎÏÌÓÔ\
            ÒÚÛÙıˆ˜¯˘˙˚¸˝˛ˇ";

        assert_eq!(mac_roman_decode(&encoded), expected);
    }
}
