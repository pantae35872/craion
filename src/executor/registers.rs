use std::{error::Error, fmt::Display};

use crate::memory::address::Address;

#[derive(Debug)]
pub enum Registers {
    A8,
    A16,
    A32,
    A64,
    B8,
    B16,
    B32,
    B64,
    C8,
    C16,
    C32,
    C64,
    D8,
    D16,
    D32,
    D64,
    IP,
}

impl Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Registers::A8 => write!(f, "a8"),
            Registers::A16 => write!(f, "a16"),
            Registers::A32 => write!(f, "a32"),
            Registers::A64 => write!(f, "a64"),
            Registers::B8 => write!(f, "b8"),
            Registers::B16 => write!(f, "b16"),
            Registers::B32 => write!(f, "b32"),
            Registers::B64 => write!(f, "b64"),
            Registers::C8 => write!(f, "c8"),
            Registers::C16 => write!(f, "c16"),
            Registers::C32 => write!(f, "c32"),
            Registers::C64 => write!(f, "c64"),
            Registers::D8 => write!(f, "d8"),
            Registers::D16 => write!(f, "d16"),
            Registers::D32 => write!(f, "d32"),
            Registers::D64 => write!(f, "d64"),
            Registers::IP => write!(f, "instruction pointer"),
        }
    }
}

#[derive(Debug)]
pub enum RegisterParseError {
    InvalidByteForm(u8),
}

impl Display for RegisterParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidByteForm(byte_form) => write!(
                f,
                "Trying to parse from invalid byte respentation of a register: {}",
                byte_form
            ),
        }
    }
}

impl Error for RegisterParseError {}

impl Registers {
    pub fn from_byte(byte_form: u8) -> Result<Self, RegisterParseError> {
        match byte_form {
            1 => return Ok(Self::A8),
            2 => return Ok(Self::A16),
            3 => return Ok(Self::A32),
            4 => return Ok(Self::A64),
            5 => return Ok(Self::B8),
            6 => return Ok(Self::B16),
            7 => return Ok(Self::B32),
            8 => return Ok(Self::B64),
            9 => return Ok(Self::C8),
            10 => return Ok(Self::C16),
            11 => return Ok(Self::C32),
            12 => return Ok(Self::C64),
            13 => return Ok(Self::D8),
            14 => return Ok(Self::D16),
            15 => return Ok(Self::D32),
            16 => return Ok(Self::D64),
            255 => return Ok(Self::IP),
            e => return Err(RegisterParseError::InvalidByteForm(e)),
        };
    }
}

/// Simple register file
///
/// Pattern:
/// "{register_name}{bits_amount}"
///
/// Registers:
/// 'a8' lower 8 bit of 'a' register
/// 'a16' lower 16 bit of 'a' register
/// 'a32' lower 32 bit of 'a' register
/// 'a64' full 64 bit of 'a' register
/// ...
/// 'ip' instruction pointer. bits amount depends on target arch
///
#[derive(Debug)]
pub struct RegisterFile {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
    ip: Address,
}

#[derive(Debug)]
pub enum RegisterFileError {
    SetError(Registers, u64),
    GeneralUnsupportSet(Registers),
}

impl Display for RegisterFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegisterFileError::SetError(register, value) => write!(
                f,
                "Trying to set a value to a register with oversize value: {}, register type: {}",
                value, register
            ),
            RegisterFileError::GeneralUnsupportSet(register) => write!(
                f,
                "Trying to access a non-general purpose register using general purpose register function: {}",
                register
            ),
        }
    }
}

impl Error for RegisterFileError {}

impl RegisterFile {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            ip: Address::new(0x0),
        }
    }

    pub fn set_ip(&mut self, data: Address) {
        self.ip = data;
    }

    pub fn get_ip(&self) -> Address {
        return self.ip.clone();
    }

    /// Increment 'ip' by perfered value and return increased 'ip'
    pub fn inc_ip(&mut self, amount: usize) -> Address {
        let inc_ip = self.ip.clone() + amount;
        self.ip = inc_ip.clone();
        return inc_ip;
    }

    pub fn set_general(&mut self, register: Registers, data: u64) -> Result<(), RegisterFileError> {
        match register {
            Registers::A8 | Registers::B8 | Registers::C8 | Registers::D8 => {
                match <u8>::try_from(data) {
                    Ok(_) => unsafe { self.set(register, data) },
                    Err(_) => return Err(RegisterFileError::SetError(register, data)),
                }
            }
            Registers::A16 | Registers::B16 | Registers::C16 | Registers::D16 => {
                match <u16>::try_from(data) {
                    Ok(_) => unsafe { self.set(register, data) },
                    Err(_) => return Err(RegisterFileError::SetError(register, data)),
                }
            }
            Registers::A32 | Registers::B32 | Registers::C32 | Registers::D32 => {
                match <u32>::try_from(data) {
                    Ok(_) => unsafe { self.set(register, data) },
                    Err(_) => return Err(RegisterFileError::SetError(register, data)),
                }
            }
            Registers::A64 | Registers::B64 | Registers::C64 | Registers::D64 => {
                match <u64>::try_from(data) {
                    Ok(_) => unsafe { self.set(register, data) },
                    Err(_) => return Err(RegisterFileError::SetError(register, data)),
                }
            }
            ur => return Err(RegisterFileError::GeneralUnsupportSet(ur)),
        };
        return Ok(());
    }

    pub fn get_general(&self, register: Registers) -> Result<u64, RegisterFileError> {
        match register {
            Registers::A8
            | Registers::B8
            | Registers::C8
            | Registers::D8
            | Registers::A16
            | Registers::B16
            | Registers::C16
            | Registers::D16
            | Registers::A32
            | Registers::B32
            | Registers::C32
            | Registers::D32
            | Registers::A64
            | Registers::B64
            | Registers::C64
            | Registers::D64 => return Ok(unsafe { self.get(register) }),
            ur => return Err(RegisterFileError::GeneralUnsupportSet(ur)),
        };
    }

    pub unsafe fn get(&self, register: Registers) -> u64 {
        match register {
            Registers::A8 => self.get_a8().into(),
            Registers::A16 => self.get_a16().into(),
            Registers::A32 => self.get_a32().into(),
            Registers::A64 => self.get_a64(),
            Registers::B8 => self.get_b8().into(),
            Registers::B16 => self.get_b16().into(),
            Registers::B32 => self.get_b32().into(),
            Registers::B64 => self.get_b64(),
            Registers::C8 => self.get_c8().into(),
            Registers::C16 => self.get_c16().into(),
            Registers::C32 => self.get_c32().into(),
            Registers::C64 => self.get_c64(),
            Registers::D8 => self.get_d8().into(),
            Registers::D16 => self.get_d16().into(),
            Registers::D32 => self.get_d32().into(),
            Registers::D64 => self.get_d64(),
            Registers::IP => self.get_ip().get_raw() as u64,
        }
    }

    pub unsafe fn set(&mut self, register: Registers, data: u64) {
        match register {
            Registers::A8 => self.set_a8(data as u8),
            Registers::A16 => self.set_a16(data as u16),
            Registers::A32 => self.set_a32(data as u32),
            Registers::A64 => self.set_a64(data as u64),
            Registers::B8 => self.set_b8(data as u8),
            Registers::B16 => self.set_b16(data as u16),
            Registers::B32 => self.set_b32(data as u32),
            Registers::B64 => self.set_b64(data as u64),
            Registers::C8 => self.set_c8(data as u8),
            Registers::C16 => self.set_c16(data as u16),
            Registers::C32 => self.set_c32(data as u32),
            Registers::C64 => self.set_c64(data as u64),
            Registers::D8 => self.set_d8(data as u8),
            Registers::D16 => self.set_d16(data as u16),
            Registers::D32 => self.set_d32(data as u32),
            Registers::D64 => self.set_d64(data as u64),
            Registers::IP => self.set_ip(Address::new(data as usize)),
        }
    }

    fn set_a8(&mut self, data: u8) {
        self.a = (self.a & 0xFFFFFFFFFFFFFF00) | data as u64;
    }

    fn set_a16(&mut self, data: u16) {
        self.a = (self.a & 0xFFFFFFFFFFFF0000) | data as u64;
    }

    fn set_a32(&mut self, data: u32) {
        self.a = (self.a & 0xFFFFFFFF00000000) | data as u64;
    }

    fn set_a64(&mut self, data: u64) {
        self.a = data;
    }

    fn get_a8(&self) -> u8 {
        return (self.a & 0xFF) as u8;
    }

    fn get_a16(&self) -> u16 {
        return (self.a & 0xFFFF) as u16;
    }

    fn get_a32(&self) -> u32 {
        return (self.a & 0xFFFFFFFF) as u32;
    }

    fn get_a64(&self) -> u64 {
        return self.a;
    }

    fn set_b8(&mut self, data: u8) {
        self.b = (self.b & 0xFFFFFFFFFFFFFF00) | data as u64;
    }

    fn set_b16(&mut self, data: u16) {
        self.b = (self.b & 0xFFFFFFFFFFFF0000) | data as u64;
    }

    fn set_b32(&mut self, data: u32) {
        self.b = (self.b & 0xFFFFFFFF00000000) | data as u64;
    }

    fn set_b64(&mut self, data: u64) {
        self.b = data;
    }

    fn get_b8(&self) -> u8 {
        return (self.b & 0xFF) as u8;
    }

    fn get_b16(&self) -> u16 {
        return (self.b & 0xFFFF) as u16;
    }

    fn get_b32(&self) -> u32 {
        return (self.b & 0xFFFFFFFF) as u32;
    }

    fn get_b64(&self) -> u64 {
        return self.b;
    }

    fn set_c8(&mut self, data: u8) {
        self.c = (self.c & 0xFFFFFFFFFFFFFF00) | data as u64;
    }

    fn set_c16(&mut self, data: u16) {
        self.c = (self.c & 0xFFFFFFFFFFFF0000) | data as u64;
    }

    fn set_c32(&mut self, data: u32) {
        self.c = (self.c & 0xFFFFFFFF00000000) | data as u64;
    }

    fn set_c64(&mut self, data: u64) {
        self.c = data;
    }

    fn get_c8(&self) -> u8 {
        return (self.c & 0xFF) as u8;
    }

    fn get_c16(&self) -> u16 {
        return (self.c & 0xFFFF) as u16;
    }

    fn get_c32(&self) -> u32 {
        return (self.c & 0xFFFFFFFF) as u32;
    }

    fn get_c64(&self) -> u64 {
        return self.c;
    }

    fn set_d8(&mut self, data: u8) {
        self.d = (self.d & 0xFFFFFFFFFFFFFF00) | data as u64;
    }

    fn set_d16(&mut self, data: u16) {
        self.d = (self.d & 0xFFFFFFFFFFFF0000) | data as u64;
    }

    fn set_d32(&mut self, data: u32) {
        self.d = (self.d & 0xFFFFFFFF00000000) | data as u64;
    }

    fn set_d64(&mut self, data: u64) {
        self.d = data;
    }

    fn get_d8(&self) -> u8 {
        return (self.d & 0xFF) as u8;
    }

    fn get_d16(&self) -> u16 {
        return (self.d & 0xFFFF) as u16;
    }

    fn get_d32(&self) -> u32 {
        return (self.d & 0xFFFFFFFF) as u32;
    }

    fn get_d64(&self) -> u64 {
        return self.d;
    }
}
