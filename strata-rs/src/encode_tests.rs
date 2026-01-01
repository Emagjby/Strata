#[cfg(test)]
mod tests {
    use crate::encode::{encode_uleb128, encode_sleb128};

    #[test]
    fn uleb128_basic() {
        let mut out = Vec::new();
        encode_uleb128(0, &mut out);
        assert_eq!(out, vec![0x00]);

        out.clear();
        encode_uleb128(1, &mut out);
        assert_eq!(out, vec![0x01]);

        out.clear();
        encode_uleb128(127, &mut out);
        assert_eq!(out, vec![0x7F]);

        out.clear();
        encode_uleb128(128, &mut out);
        assert_eq!(out, vec![0x80, 0x01]);
    }

    #[test]
    fn sleb128_basic() {
        let mut out = Vec::new();
        encode_sleb128(0, &mut out);
        assert_eq!(out, vec![0x00]);

        out.clear();
        encode_sleb128(1, &mut out);
        assert_eq!(out, vec![0x01]);

        out.clear();
        encode_sleb128(-1, &mut out);
        assert_eq!(out, vec![0x7F]);

        out.clear();
        encode_sleb128(127, &mut out);
        assert_eq!(out, vec![0xFF, 0x00]);

        out.clear();
        encode_sleb128(-128, &mut out);
        assert_eq!(out, vec![0x80, 0x7F]);
    }
}
