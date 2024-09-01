use std::io::{self, Error};

#[cfg(feature = "use-borsh")]
pub use borsh::{BorshSerialize, BorshDeserialize};

#[allow(non_camel_case_types)]
pub struct CompactU16(pub u16);

impl CompactU16 {
    pub fn try_from_bytes(buf: &mut &[u8]) -> io::Result<Self> {
        let mut result: u16 = 0;
        let mut shift = 0;
        loop {
            let byte = u8::deserialize(buf)?;
            result |= ((byte & 0x7f) as u16) << shift;
            if byte & 0x80 == 0 {
                break;
            }
            shift += 7;
            if shift >= 16 {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    "cu16 out of bounds",
                ));
            }
        }
        Ok(Self(result))
    }
}

#[cfg(feature = "use-borsh")]
impl BorshDeserialize for CompactU16 {
    fn deserialize_reader<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let mut result: u16 = 0;
        let mut shift = 0;
        loop {
            let byte = u8::deserialize_reader(reader)?;
            result |= ((byte & 0x7f) as u16) << shift;
            if byte & 0x80 == 0 {
                break;
            }
            shift += 7;
            if shift >= 16 {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    "cu16 out of bounds",
                ));
            }
        }
        Ok(Self(result))
    }
    
    fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        Self::try_from_bytes(buf)
    }
    
    fn try_from_slice(v: &[u8]) -> io::Result<Self> {
        let mut v_mut = v;
        let result = Self::deserialize(&mut v_mut)?;
        if !v_mut.is_empty() {
            return Err(Error::new(io::ErrorKind::InvalidData, "Not all bytes read"));
        }
        Ok(result)
    }
    
    fn try_from_reader<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let result = Self::deserialize_reader(reader)?;
        let mut buf = [0u8; 1];
        match reader.read_exact(&mut buf) {
            Err(f) if f.kind() == io::ErrorKind::UnexpectedEof => Ok(result),
            _ => Err(Error::new(io::ErrorKind::InvalidData, "Not all bytes read")),
        }
    }
}

#[cfg(feature = "use-borsh")]
impl BorshSerialize for CompactU16 {    
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let mut value = self.0;

        while value >= 0x80 {
            writer.write_all(&[(value as u8 & 0x7f) | 0x80])?;
            value >>= 7;
        }
        writer.write_all(&[value as u8])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_cu16() {
        let values = [
            (0x0000, vec![0x00]),
            (0x0001, vec![0x01]),
            (0x007f, vec![0x7f]),
            (0x0080, vec![0x80, 0x01]),
            (0x3fff, vec![0xff, 0x7f]),
            (0x4000, vec![0x80, 0x80, 0x01]),
            (0xc000, vec![0x80, 0x80, 0x03]),
            (0xffff, vec![0xff, 0xff, 0x03])
        ];

        for value in values {
            let mut input = &value.1[..];
            let n = CompactU16::deserialize(&mut input).unwrap();
            assert_eq!(n.0, value.0)
        }
    }
}