mod deku_utils;

use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct EmojipackHeader {
    version: EmojipackVersion,
    signature: [u8; blake3::OUT_LEN],
    #[deku(
        reader = "deku_utils::read_string(deku::rest)",
        writer = "deku_utils::write_string(deku::output, &self.name)"
    )]
    name: String,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum EmojipackVersion {
    One = 0, // confusing, I know
}

#[cfg(test)]
mod tests {
    use super::*;
    use hexlit::hex;

    fn get_simple_header() -> EmojipackHeader {
        EmojipackHeader {
            version: EmojipackVersion::One,
            signature: hex!("264bcafcc26080c228c47ac79a017f1c9427886607fcf744c0c284486098bdab"), // "The great amazing emojipack data"
            name: "test".to_string(),
        }
    }
    const SIMPLE_HEADER_DATA: &[u8] =
        &hex!("00264bcafcc26080c228c47ac79a017f1c9427886607fcf744c0c284486098bdab000474657374");

    #[test]
    fn test_read_simple_header() {
        let ((_, left), header) = EmojipackHeader::from_bytes((SIMPLE_HEADER_DATA, 0)).unwrap();
        assert_eq!(left, 0);
        assert_eq!(header, get_simple_header());
    }
    #[test]
    fn test_write_simple_header() {
        let out = get_simple_header().to_bytes().unwrap();
        assert_eq!(out, SIMPLE_HEADER_DATA);
    }
}
