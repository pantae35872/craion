use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::{
    executor::registers::{RegisterParseError, Registers},
    memory::{address::Address, buffer_reader::BufferReader},
};

#[derive(Debug)]
pub enum ArgumentParseError {
    OutOfRange(usize),
    RegisterParseError(RegisterParseError),
}

impl Display for ArgumentParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfRange(readpos) => {
                write!(f, "Trying to read with invalid readpos: {}", readpos)
            }
            Self::RegisterParseError(parse_error) => write!(f, "{}", parse_error),
        }
    }
}

impl From<RegisterParseError> for ArgumentParseError {
    fn from(value: RegisterParseError) -> Self {
        Self::RegisterParseError(value)
    }
}

impl Error for ArgumentParseError {}

pub struct Argument<'a> {
    reader: BufferReader<'a>,
}

impl<'a> Argument<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            reader: BufferReader::new(data),
        }
    }

    pub fn parse_register(&mut self) -> Result<Registers, ArgumentParseError> {
        let byte_read = match self.reader.read_bytes(1) {
            Some(byte) => byte,
            None => return Err(ArgumentParseError::OutOfRange(self.reader.get_read_pos())),
        };
        return Ok(Registers::from_byte(byte_read[0])?);
    }

    pub fn parse_i64(&mut self) -> Result<i64, ArgumentParseError> {
        let number_read = match self.reader.read_i64() {
            Some(number) => number,
            None => return Err(ArgumentParseError::OutOfRange(self.reader.get_read_pos())),
        };
        return Ok(number_read);
    }

    pub fn parse_address(&mut self) -> Result<Address, ArgumentParseError> {
        let number_read = match self.reader.read_u64() {
            Some(number) => number,
            None => return Err(ArgumentParseError::OutOfRange(self.reader.get_read_pos())),
        };
        return Ok(Address::new(number_read as usize));
    }
}
