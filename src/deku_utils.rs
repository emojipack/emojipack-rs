use deku::bitvec::{BitSlice, BitVec, Msb0};
use deku::prelude::*;
use deku::DekuError;

pub(crate) fn read_string(
    rest: &BitSlice<Msb0, u8>,
) -> Result<(&BitSlice<Msb0, u8>, String), DekuError> {
    use deku::ctx::{Endian, Limit};

    let (rest, len) = u16::read(rest, Endian::Big)?;
    let (rest, bytes) = Vec::read(rest, Limit::new_count(len as _))?;
    Ok((
        rest,
        String::from_utf8(bytes).map_err(|e| DekuError::Parse(e.to_string()))?,
    ))
}

pub(crate) fn write_string(output: &mut BitVec<Msb0, u8>, s: &str) -> Result<(), DekuError> {
    use deku::ctx::Endian;

    (s.len() as u16).write(output, Endian::Big)?;
    s.as_bytes().write(output, ())?;

    Ok(())
}
