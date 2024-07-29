#![feature(test)]

use executor::{registers::RegisterFile, Executor};
use memory::address::Address;

use crate::memory::Memory;

extern crate test;
use test::Bencher;

pub mod decoder;
pub mod executor;
pub mod memory;

fn main() {
    let mut memory = Memory::new(16);
    let opcode = 16u16.to_le_bytes();
    memory
        .mem_sets(Address::new(0x0), &[6, opcode[0], opcode[1], 1, 4, 8])
        .unwrap();
    let opcode = 32u16.to_le_bytes();
    memory
        .mem_sets(Address::new(0x6), &[5, opcode[0], opcode[1], 4, 8])
        .unwrap();
    let opcode = 65535u16.to_le_bytes();
    memory
        .mem_sets(Address::new(0xb), &[3, opcode[0], opcode[1]])
        .unwrap();
    let mut register = RegisterFile::new();
    register
        .set_general(&executor::registers::Registers::A64, 5)
        .unwrap();
    register
        .set_general(&executor::registers::Registers::B64, 0xFFFFFFFFFFFFFFFF)
        .unwrap();
    let mut executor = Executor::new(&mut memory, &mut register);
    executor.execute();
}
#[bench]
fn bench_simple_execute(b: &mut Bencher) {
    let mut memory = Memory::new(16);
    let mut register = RegisterFile::new();
    b.iter(|| {
        test::black_box({
            let opcode = 16u16.to_le_bytes();
            memory
                .mem_sets(Address::new(0x0), &[6, opcode[0], opcode[1], 1, 4, 8])
                .unwrap();
            let opcode = 65535u16.to_le_bytes();
            memory
                .mem_sets(Address::new(0x6), &[3, opcode[0], opcode[1]])
                .unwrap();
            register
                .set_general(&executor::registers::Registers::A64, 5)
                .unwrap();
            register
                .set_general(&executor::registers::Registers::B64, 0xFFFFFFFFFFFFFFFF)
                .unwrap();
            register.set_halt(false);
            register.set_ip(Address::new(0));
            let mut executor = Executor::new(&mut memory, &mut register);
            executor.execute();
        });
    });
}
