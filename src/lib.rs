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

    #[test]
    fn test_simple_header() {
        let data: &[u8] =
            &hex!("00264bcafcc26080c228c47ac79a017f1c9427886607fcf744c0c284486098bdab000474657374");
        let ((_, left), header) = EmojipackHeader::from_bytes((data, 0)).unwrap();
        assert_eq!(left, 0);
        assert_eq!(header.version, EmojipackVersion::One);
        assert_eq!(
            header.signature,
            hex!("264bcafcc26080c228c47ac79a017f1c9427886607fcf744c0c284486098bdab") // "The great amazing emojipack data"
        );
        assert_eq!(header.name, "test");
    }
}
