// Needed to avoid warnings from inside const_concat_bytes!(...)
#![allow(clippy::transmute_ptr_to_ref)]

use byte_strings::const_concat_bytes;

// TTF/OTF format from https://learn.microsoft.com/en-us/typography/opentype/spec/otff

const NAME: &str = "";
const EM_SIZE: u16 = 1000;
const CAP_HEIGHT: u32 = 714;
const X_HEIGHT: u16 = 523;
const ASCENT: u16 = 952;
const DESCENT: u16 = 213;
const LINE_HEIGHT: u16 = 1165;
// const SLOPE: u32 = 0; // TODO this is only used in the postscript table

// 'name' table
mod name {
    use super::*;

    const VERSION: u16 = 0;
    const COUNT: u16 = 1;
    const STORAGE_OFFSET: u16 = 6 + COUNT * 12; // Offset of the first string

    const PLATFORM_ID: u16 = 0;
    const ENCODING_ID: u16 = 4;
    const LANGUAGE_ID: u16 = 0;
    const NAME_ID: u16 = 1;
    const NAME_LENGTH: u16 = NAME.len() as u16;
    const STRING_OFFSET: u16 = 0;

    pub const TABLE: &[u8] = const_concat_bytes!(
        VERSION.to_be_bytes().as_slice(),
        COUNT.to_be_bytes().as_slice(),
        STORAGE_OFFSET.to_be_bytes().as_slice(),
        PLATFORM_ID.to_be_bytes().as_slice(),
        ENCODING_ID.to_be_bytes().as_slice(),
        LANGUAGE_ID.to_be_bytes().as_slice(),
        NAME_ID.to_be_bytes().as_slice(),
        NAME_LENGTH.to_be_bytes().as_slice(),
        STRING_OFFSET.to_be_bytes().as_slice(),
        NAME.as_bytes(),
    );

    const TABLE_TAG: [u8; 4] = *b"name";
    const CHECKSUM: u32 = 0; // TODO Calculate checksum? It gets ignored by ttf-parser
    pub const OFFSET: u32 = 12 + (NUM_TABLES as u32) * 16; // Size of the table directory
    pub const LENGTH: u32 = TABLE.len() as u32;

    pub const RECORD: &[u8] = const_concat_bytes!(
        TABLE_TAG.as_slice(),
        CHECKSUM.to_be_bytes().as_slice(),
        OFFSET.to_be_bytes().as_slice(),
        LENGTH.to_be_bytes().as_slice(),
    );
}

// 'head' table
mod head {
    use super::*;

    const MAJOR_VERSION: u16 = 1;
    const MINOR_VERSION: u16 = 0;
    const FONT_MAJOR_REVISION: u16 = 1;
    const FONT_MINOR_REVISION: u16 = 0;
    const CHECKSUM_ADJUSTMENT: u32 = 0; // TODO Calculate checksum? It gets ignored by ttf-parser
    const MAGIC_NUMBER: u32 = 0x5F0F3CF5;
    const FLAGS: u16 = 0;
    const UNITS_PER_EM: u16 = EM_SIZE;
    const CREATED: i64 = 0;
    const MODIFIED: i64 = 0;
    const X_MIN: i16 = 0;
    const Y_MIN: i16 = 0;
    const X_MAX: i16 = 0;
    const Y_MAX: i16 = 0;
    const MAC_STYLE: u16 = 0;
    const LOWEST_REC_PPEM: u16 = 0;
    const FONT_DIRECTION_HINT: i16 = 2;
    const INDEX_TO_LOC_FORMAT: i16 = 0;
    const GLYPH_DATA_FORMAT: i16 = 0;

    pub const TABLE: &[u8] = const_concat_bytes!(
        MAJOR_VERSION.to_be_bytes().as_slice(),
        MINOR_VERSION.to_be_bytes().as_slice(),
        FONT_MAJOR_REVISION.to_be_bytes().as_slice(),
        FONT_MINOR_REVISION.to_be_bytes().as_slice(),
        CHECKSUM_ADJUSTMENT.to_be_bytes().as_slice(),
        MAGIC_NUMBER.to_be_bytes().as_slice(),
        FLAGS.to_be_bytes().as_slice(),
        UNITS_PER_EM.to_be_bytes().as_slice(),
        CREATED.to_be_bytes().as_slice(),
        MODIFIED.to_be_bytes().as_slice(),
        X_MIN.to_be_bytes().as_slice(),
        Y_MIN.to_be_bytes().as_slice(),
        X_MAX.to_be_bytes().as_slice(),
        Y_MAX.to_be_bytes().as_slice(),
        MAC_STYLE.to_be_bytes().as_slice(),
        LOWEST_REC_PPEM.to_be_bytes().as_slice(),
        FONT_DIRECTION_HINT.to_be_bytes().as_slice(),
        INDEX_TO_LOC_FORMAT.to_be_bytes().as_slice(),
        GLYPH_DATA_FORMAT.to_be_bytes().as_slice(),
    );

    const TABLE_TAG: [u8; 4] = *b"head";
    const CHECKSUM: u32 = 0; // TODO Calculate checksum? It gets ignored by ttf-parser
    pub const OFFSET: u32 = name::OFFSET + name::LENGTH;
    pub const LENGTH: u32 = TABLE.len() as u32;

    pub const RECORD: &[u8] = const_concat_bytes!(
        TABLE_TAG.as_slice(),
        CHECKSUM.to_be_bytes().as_slice(),
        OFFSET.to_be_bytes().as_slice(),
        LENGTH.to_be_bytes().as_slice(),
    );
}

// 'hhea' table
mod hhea {
    use super::*;

    const MAJOR_VERSION: u16 = 1;
    const MINOR_VERSION: u16 = 0;
    const ASCENDER: i16 = ASCENT as i16;
    const DESCENDER: i16 = -(DESCENT as i16);
    const LINE_GAP: i16 = (LINE_HEIGHT as i16) - (ASCENT as i16) - (DESCENT as i16);
    const ADVANCE_WIDTH_MAX: u16 = 0;
    const MIN_LEFT_SIDE_BEARING: i16 = 0;
    const MIN_RIGHT_SIDE_BEARING: i16 = 0;
    const X_MAX_EXTENT: i16 = 0;
    const CARET_SLOPE_RISE: i16 = 1;
    const CARET_SLOPE_RUN: i16 = 0;
    const CARET_OFFSET: i16 = 0;
    const RESERVED_0: i16 = 0;
    const RESERVED_1: i16 = 0;
    const RESERVED_2: i16 = 0;
    const RESERVED_3: i16 = 0;
    const METRIC_DATA_FORMAT: i16 = 0;
    const NUMBER_OF_H_METRICS: u16 = 0;

    pub const TABLE: &[u8] = const_concat_bytes!(
        MAJOR_VERSION.to_be_bytes().as_slice(),
        MINOR_VERSION.to_be_bytes().as_slice(),
        ASCENDER.to_be_bytes().as_slice(),
        DESCENDER.to_be_bytes().as_slice(),
        LINE_GAP.to_be_bytes().as_slice(),
        ADVANCE_WIDTH_MAX.to_be_bytes().as_slice(),
        MIN_LEFT_SIDE_BEARING.to_be_bytes().as_slice(),
        MIN_RIGHT_SIDE_BEARING.to_be_bytes().as_slice(),
        X_MAX_EXTENT.to_be_bytes().as_slice(),
        CARET_SLOPE_RISE.to_be_bytes().as_slice(),
        CARET_SLOPE_RUN.to_be_bytes().as_slice(),
        CARET_OFFSET.to_be_bytes().as_slice(),
        RESERVED_0.to_be_bytes().as_slice(),
        RESERVED_1.to_be_bytes().as_slice(),
        RESERVED_2.to_be_bytes().as_slice(),
        RESERVED_3.to_be_bytes().as_slice(),
        METRIC_DATA_FORMAT.to_be_bytes().as_slice(),
        NUMBER_OF_H_METRICS.to_be_bytes().as_slice(),
    );

    const TABLE_TAG: [u8; 4] = *b"hhea";
    const CHECKSUM: u32 = 0; // TODO Calculate checksum? It gets ignored by ttf-parser
    pub const OFFSET: u32 = head::OFFSET + head::LENGTH;
    pub const LENGTH: u32 = TABLE.len() as u32;

    pub const RECORD: &[u8] = const_concat_bytes!(
        TABLE_TAG.as_slice(),
        CHECKSUM.to_be_bytes().as_slice(),
        OFFSET.to_be_bytes().as_slice(),
        LENGTH.to_be_bytes().as_slice(),
    );
}

// 'maxp' table
mod maxp {
    use super::*;

    const MAJOR_VERSION: u16 = 1;
    const MINOR_VERSION: u16 = 0;
    const NUM_GLYPHS: u16 = 1; // Should be 0, but ttf-parser doesn't like that
    const MAX_POINTS: u16 = 0;
    const MAX_CONTOURS: u16 = 0;
    const MAX_COMPOSITE_POINTS: u16 = 0;
    const MAX_COMPOSITE_CONTOURS: u16 = 0;
    const MAX_ZONES: u16 = 0;
    const MAX_TWILIGHT_POINTS: u16 = 0;
    const MAX_STORAGE: u16 = 0;
    const MAX_FUNCTION_DEFS: u16 = 0;
    const MAX_INSTRUCTION_DEFS: u16 = 0;
    const MAX_STACK_ELEMENTS: u16 = 0;
    const MAX_SIZE_OF_INSTRUCTIONS: u16 = 0;
    const MAX_COMPONENT_ELEMENTS: u16 = 0;
    const MAX_COMPONENT_DEPTH: u16 = 0;

    pub const TABLE: &[u8] = const_concat_bytes!(
        MAJOR_VERSION.to_be_bytes().as_slice(),
        MINOR_VERSION.to_be_bytes().as_slice(),
        NUM_GLYPHS.to_be_bytes().as_slice(),
        MAX_POINTS.to_be_bytes().as_slice(),
        MAX_CONTOURS.to_be_bytes().as_slice(),
        MAX_COMPOSITE_POINTS.to_be_bytes().as_slice(),
        MAX_COMPOSITE_CONTOURS.to_be_bytes().as_slice(),
        MAX_ZONES.to_be_bytes().as_slice(),
        MAX_TWILIGHT_POINTS.to_be_bytes().as_slice(),
        MAX_STORAGE.to_be_bytes().as_slice(),
        MAX_FUNCTION_DEFS.to_be_bytes().as_slice(),
        MAX_INSTRUCTION_DEFS.to_be_bytes().as_slice(),
        MAX_STACK_ELEMENTS.to_be_bytes().as_slice(),
        MAX_SIZE_OF_INSTRUCTIONS.to_be_bytes().as_slice(),
        MAX_COMPONENT_ELEMENTS.to_be_bytes().as_slice(),
        MAX_COMPONENT_DEPTH.to_be_bytes().as_slice(),
    );

    const TABLE_TAG: [u8; 4] = *b"maxp";
    const CHECKSUM: u32 = 0; // TODO Calculate checksum? It gets ignored by ttf-parser
    pub const OFFSET: u32 = hhea::OFFSET + hhea::LENGTH;
    pub const LENGTH: u32 = TABLE.len() as u32;

    pub const RECORD: &[u8] = const_concat_bytes!(
        TABLE_TAG.as_slice(),
        CHECKSUM.to_be_bytes().as_slice(),
        OFFSET.to_be_bytes().as_slice(),
        LENGTH.to_be_bytes().as_slice(),
    );
}

// 'OS/2' table
mod os2 {
    use super::*;

    const VERSION: u16 = 4;
    const X_AVG_CHAR_WIDTH: i16 = 0;
    const US_WEIGHT_CLASS: u16 = 400;
    const US_WIDTH_CLASS: u16 = 5;
    const FS_TYPE: u16 = 0;
    const Y_SUBSCRIPT_X_SIZE: i16 = 600;
    const Y_SUBSCRIPT_Y_SIZE: i16 = 600;
    const Y_SUBSCRIPT_X_OFFSET: i16 = 0;
    const Y_SUBSCRIPT_Y_OFFSET: i16 = 150;
    const Y_SUPERSCRIPT_X_SIZE: i16 = 600;
    const Y_SUPERSCRIPT_Y_SIZE: i16 = 600;
    const Y_SUPERSCRIPT_X_OFFSET: i16 = 0;
    const Y_SUPERSCRIPT_Y_OFFSET: i16 = 510;
    const Y_STRIKEOUT_SIZE: i16 = 60;
    const Y_STRIKEOUT_POSITION: i16 = 260;
    const S_FAMILY_CLASS: i16 = 0;
    const PANOSE: [u8; 10] = [0; 10];
    const UL_UNICODE_RANGE_1: u32 = 0;
    const UL_UNICODE_RANGE_2: u32 = 0;
    const UL_UNICODE_RANGE_3: u32 = 0;
    const UL_UNICODE_RANGE_4: u32 = 0;
    const ACH_VEND_ID: [u8; 4] = *b"font";
    const FS_SELECTION: u16 = 0x00C0;
    const US_FIRST_CHAR_INDEX: u16 = 0;
    const US_LAST_CHAR_INDEX: u16 = 0;
    const S_TYPO_ASCENDER: i16 = ASCENT as i16;
    const S_TYPO_DESCENDER: i16 = -(DESCENT as i16);
    const S_TYPO_LINE_GAP: i16 = (LINE_HEIGHT as i16) - (ASCENT as i16) - (DESCENT as i16);
    const US_WIN_ASCENT: u16 = ASCENT;
    const US_WIN_DESCENT: u16 = DESCENT;
    const UL_CODE_PAGE_RANGE_1: u32 = 0;
    const UL_CODE_PAGE_RANGE_2: u32 = 0;
    const S_X_HEIGHT: i16 = X_HEIGHT as i16;
    const S_CAP_HEIGHT: i16 = CAP_HEIGHT as i16;
    const US_DEFAULT_CHAR: u16 = 0;
    const US_BREAK_CHAR: u16 = 0;
    const US_MAX_CONTEXT: u16 = 0;
    const US_LOWER_OPTICAL_POINT_SIZE: u16 = 0;
    const US_UPPER_OPTICAL_POINT_SIZE: u16 = 0xffff;

    pub const TABLE: &[u8] = const_concat_bytes!(
        VERSION.to_be_bytes().as_slice(),
        X_AVG_CHAR_WIDTH.to_be_bytes().as_slice(),
        US_WEIGHT_CLASS.to_be_bytes().as_slice(),
        US_WIDTH_CLASS.to_be_bytes().as_slice(),
        FS_TYPE.to_be_bytes().as_slice(),
        Y_SUBSCRIPT_X_SIZE.to_be_bytes().as_slice(),
        Y_SUBSCRIPT_Y_SIZE.to_be_bytes().as_slice(),
        Y_SUBSCRIPT_X_OFFSET.to_be_bytes().as_slice(),
        Y_SUBSCRIPT_Y_OFFSET.to_be_bytes().as_slice(),
        Y_SUPERSCRIPT_X_SIZE.to_be_bytes().as_slice(),
        Y_SUPERSCRIPT_Y_SIZE.to_be_bytes().as_slice(),
        Y_SUPERSCRIPT_X_OFFSET.to_be_bytes().as_slice(),
        Y_SUPERSCRIPT_Y_OFFSET.to_be_bytes().as_slice(),
        Y_STRIKEOUT_SIZE.to_be_bytes().as_slice(),
        Y_STRIKEOUT_POSITION.to_be_bytes().as_slice(),
        S_FAMILY_CLASS.to_be_bytes().as_slice(),
        PANOSE.as_slice(),
        UL_UNICODE_RANGE_1.to_be_bytes().as_slice(),
        UL_UNICODE_RANGE_2.to_be_bytes().as_slice(),
        UL_UNICODE_RANGE_3.to_be_bytes().as_slice(),
        UL_UNICODE_RANGE_4.to_be_bytes().as_slice(),
        ACH_VEND_ID.as_slice(),
        FS_SELECTION.to_be_bytes().as_slice(),
        US_FIRST_CHAR_INDEX.to_be_bytes().as_slice(),
        US_LAST_CHAR_INDEX.to_be_bytes().as_slice(),
        S_TYPO_ASCENDER.to_be_bytes().as_slice(),
        S_TYPO_DESCENDER.to_be_bytes().as_slice(),
        S_TYPO_LINE_GAP.to_be_bytes().as_slice(),
        US_WIN_ASCENT.to_be_bytes().as_slice(),
        US_WIN_DESCENT.to_be_bytes().as_slice(),
        UL_CODE_PAGE_RANGE_1.to_be_bytes().as_slice(),
        UL_CODE_PAGE_RANGE_2.to_be_bytes().as_slice(),
        S_X_HEIGHT.to_be_bytes().as_slice(),
        S_CAP_HEIGHT.to_be_bytes().as_slice(),
        US_DEFAULT_CHAR.to_be_bytes().as_slice(),
        US_BREAK_CHAR.to_be_bytes().as_slice(),
        US_MAX_CONTEXT.to_be_bytes().as_slice(),
        US_LOWER_OPTICAL_POINT_SIZE.to_be_bytes().as_slice(),
        US_UPPER_OPTICAL_POINT_SIZE.to_be_bytes().as_slice(),
    );

    const TABLE_TAG: [u8; 4] = *b"OS/2";
    const CHECKSUM: u32 = 0; // TODO Calculate checksum? It gets ignored by ttf-parser
    pub const OFFSET: u32 = maxp::OFFSET + maxp::LENGTH;
    pub const LENGTH: u32 = TABLE.len() as u32;

    pub const RECORD: &[u8] = const_concat_bytes!(
        TABLE_TAG.as_slice(),
        CHECKSUM.to_be_bytes().as_slice(),
        OFFSET.to_be_bytes().as_slice(),
        LENGTH.to_be_bytes().as_slice(),
    );
}

const SFNT_MAJOR_VERSION: u16 = 1;
const SFNT_MINOR_VERSION: u16 = 0;
const NUM_TABLES: u16 = 5;
const SEARCH_RANGE: u16 = 0x10 << (u16::BITS - NUM_TABLES.leading_zeros() - 1); // 16 * ((2**floor(log2(numTables)))
const ENTRY_SELECTOR: u16 = (u16::BITS - NUM_TABLES.leading_zeros() - 1) as u16; // floor(log2(numTables))
const RANGE_SHIFT: u16 = 0x10 * NUM_TABLES - SEARCH_RANGE; // ((numTables * 16) - searchRange)

pub const FONT: &[u8] = const_concat_bytes!(
    // Table directory
    SFNT_MAJOR_VERSION.to_be_bytes().as_slice(),
    SFNT_MINOR_VERSION.to_be_bytes().as_slice(),
    NUM_TABLES.to_be_bytes().as_slice(),
    SEARCH_RANGE.to_be_bytes().as_slice(),
    ENTRY_SELECTOR.to_be_bytes().as_slice(),
    RANGE_SHIFT.to_be_bytes().as_slice(),
    // Table records
    name::RECORD,
    head::RECORD,
    hhea::RECORD,
    maxp::RECORD,
    os2::RECORD,
    // Tables
    name::TABLE,
    head::TABLE,
    hhea::TABLE,
    maxp::TABLE,
    os2::TABLE,
);
