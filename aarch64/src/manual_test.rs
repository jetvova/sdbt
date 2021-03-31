#![allow(non_snake_case, non_camel_case_types)]
use super::*;

#[test]
fn decode_UNKNOWN() {
    assert_eq!(
        Instruction::decode(0x01234567),
        Instruction::UNKNOWN(0x01234567)
    );
}

#[test]
fn encode_UNKNOWN() {
    assert_eq!(Instruction::UNKNOWN(0x01234567).encode(), 0x01234567);
}

#[test]
fn decode_NOP() {
    assert_eq!(
        Instruction::decode(0b11010101000000110010000000011111),
        Instruction::NOP_HI_hints
    );
}

#[test]
fn encode_NOP() {
    assert_eq!(
        Instruction::NOP_HI_hints.encode(),
        0b11010101000000110010000000011111
    );
}

#[test]
fn decode_EXTR_32() {
    assert_eq!(
        Instruction::decode(0b0_00_100111_0_0_00001_000010_00011_00100),
        Instruction::EXTR_32_extract {
            Rm: 1 as u8,
            imms: 2 as u8,
            Rn: 3 as u8,
            Rd: 4 as u8,
        }
    );
}

#[test]
fn encode_EXTR_32() {
    assert_eq!(
        Instruction::EXTR_32_extract {
            Rm: 1 as u8,
            imms: 2 as u8,
            Rn: 3 as u8,
            Rd: 4 as u8,
        }
        .encode(),
        0b0_00_100111_0_0_00001_000010_00011_00100
    );
}

#[test]
fn decode_EXTR_64() {
    assert_eq!(
        Instruction::decode(0b1_00_100111_1_0_00001_111111_00011_00100),
        Instruction::EXTR_64_extract {
            Rm: 1 as u8,
            imms: 63 as u8,
            Rn: 3 as u8,
            Rd: 4 as u8,
        }
    );
}

#[test]
fn encode_EXTR_64() {
    assert_eq!(
        Instruction::EXTR_64_extract {
            Rm: 1 as u8,
            imms: 63 as u8,
            Rn: 3 as u8,
            Rd: 4 as u8,
        }
        .encode(),
        0b1_00_100111_1_0_00001_111111_00011_00100
    );
}

#[test]
fn decode_CSINV_64_consdel() {
    let left = Instruction::decode(0b_1_1_0_11010100_00001_1110_0_0_00011_00100);
    let right = Instruction::CSINV_64_condsel {
        Rm: 1 as u8,
        cond: 14 as u8,
        Rn: 3 as u8,
        Rd: 4 as u8,
    };

    assert_eq!(left, right);
}