mod instruction;
pub use instruction::*;

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn decode_UNKNOWN() {
        assert_eq!(
            Instruction::decode(0x01234567),
            Instruction::UNKNOWN(0x01234567)
        );
    }

    #[test]
    fn decode_NOP() {
        assert_eq!(
            Instruction::decode(0b11010101000000110010000000011111),
            Instruction::NOP_HI_hints
        );
    }

    #[test]
    fn encode_UNKNOWN() {
        assert_eq!(Instruction::UNKNOWN(0x01234567).encode(), 0x01234567);
    }

    #[test]
    fn encode_NOP() {
        assert_eq!(
            Instruction::NOP_HI_hints.encode(),
            0b11010101000000110010000000011111
        );
    }
}
