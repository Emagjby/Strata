pub fn encode_uleb128(mut n: u64, out: &mut Vec<u8>) {
    loop {
        let mut byte = (n & 0x7F) as u8;
        n >>= 7;
        if n != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if n == 0 {
            break;
        }
    }
}

pub fn encode_sleb128(mut n: i64, out: &mut Vec<u8>) {
    loop {
        let byte = (n & 0x7F) as u8;
        let sign_bit = byte & 0x40;
        n >>= 7;

        let done = (n == 0 && sign_bit == 0)
            || (n == -1 && sign_bit != 0);

        out.push(if done { byte } else { byte | 0x80 });

        if done {
            break;
        }
    }
}
