use crate::value::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    UnexpectedEOF,
    InvalidTag(u8),
    InvalidVarint,
    InvalidUtf8,
}

pub struct Decoder<'a> {
    input: &'a [u8],
    position: usize,
}

impl<'a> Decoder<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self { input, position: 0 }
    }

    fn remaining(&self) -> usize {
        self.input.len() - self.position
    }

    fn read_byte(&mut self) -> Result<u8, DecodeError> {
        if self.position >= self.input.len() {
            return Err(DecodeError::UnexpectedEOF);
        }
        let byte = self.input[self.position];
        self.position += 1;
        Ok(byte)
    }

    fn read_slice(&mut self, len: usize) -> Result<&'a [u8], DecodeError> {
        if self.remaining() < len {
            return Err(DecodeError::UnexpectedEOF);
        }
        let slice = &self.input[self.position..self.position + len];
        self.position += len;
        Ok(slice)
    }

    fn decode_uleb128(&mut self) -> Result<u64, DecodeError> {
        let mut result = 0u64;
        let mut shift = 0;

        loop {
            let byte = self.read_byte()?;
            let value = (byte & 0x7F) as u64;

            if shift >= 64 {
                return Err(DecodeError::InvalidVarint);
            }

            result |= value << shift;

            if (byte & 0x80) == 0 {
                return Ok(result as u64);
            }

            shift += 7;
        }
    }

    fn decode_sleb128(&mut self) -> Result<i64, DecodeError> {
        let mut result = 0i64;
        let mut shift = 0;
        let mut byte;

        loop {
            byte = self.read_byte()? as i64;
            let value = byte & 0x7F;

            result |= value << shift;
            shift += 7;

            if (byte & 0x80) == 0 {
                break;
            }

            if shift >= 64 {
                return Err(DecodeError::InvalidVarint);
            }
        }

        if (shift < 64) && ((byte & 0x40) != 0) {
            result |= (!0i64) << shift;
        }

        Ok(result)
    }

    pub fn decode_value(&mut self) -> Result<Value, DecodeError> {
        let tag = self.read_byte()?;

        match tag {
            0x00 => Ok(Value::Null),
            0x01 => Ok(Value::Bool(false)),
            0x02 => Ok(Value::Bool(true)),

            0x10 => {
                let n = self.decode_sleb128()?;
                Ok(Value::Int(n))
            }

            0x20 => {
                let len = self.decode_uleb128()? as usize;
                let bytes = self.read_slice(len)?;
                let s = std::str::from_utf8(bytes).map_err(|_| DecodeError::InvalidUtf8)?;

                Ok(Value::String(s.to_string()))
            }

            0x21 => {
                let len = self.decode_uleb128()? as usize;
                let bytes = self.read_slice(len)?;
                Ok(Value::Bytes(bytes.to_vec()))
            }

            0x30 => {
                let count = self.decode_uleb128()? as usize;
                let mut items = Vec::with_capacity(count);
                for _ in 0..count {
                    items.push(self.decode_value()?);
                }
                Ok(Value::List(items))
            }

            0x40 => {
                let count = self.decode_uleb128()? as usize;
                let mut map = std::collections::BTreeMap::new();

                for _ in 0..count {
                    let key = match self.decode_value()? {
                        Value::String(s) => s,
                        _ => return Err(DecodeError::InvalidTag(0)),
                    };
                    let value = self.decode_value()?;
                    map.insert(key, value);
                }
                Ok(Value::Map(map))
            }

            other => Err(DecodeError::InvalidTag(other)),
        }
    }
}

pub fn decode(input: &[u8]) -> Result<Value, DecodeError> {
    let mut decoder = Decoder::new(input);
    let value = decoder.decode_value()?;

    if decoder.remaining() != 0 {
        return Err(DecodeError::InvalidTag(0)); // Extra data after valid value
    }

    Ok(value)
}
