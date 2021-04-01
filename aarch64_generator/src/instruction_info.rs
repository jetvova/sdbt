use crate::BitRange;
use crate::InstructionParameter;
use crate::MaskAndValue;
use crate::MergedEncoding;

#[derive(Debug, Serialize)]
pub struct InstructionInfo {
    pub name: String,
    pub identification: MaskAndValue,
    pub parameters: Option<Vec<InstructionParameter>>,
}

impl InstructionInfo {
    pub fn new(merged_encoding: MergedEncoding) -> Self {
        let name = Self::standardize_name(merged_encoding.encoding_name);
        let identification = MaskAndValue {
            mask: Self::create_bits(&merged_encoding.bit_ranges, false),
            value: Self::create_bits(&merged_encoding.bit_ranges, true),
        };
        let parameters = Self::create_parameters(&merged_encoding.bit_ranges, identification.mask);

        Self {
            name,
            identification,
            parameters,
        }
    }

    fn standardize_name(instruction_name: String) -> String {
        let first_underscore = instruction_name
            .find('_')
            .expect("Instruction name does not contain _");
        let name_first_part = instruction_name[0..first_underscore].to_uppercase();
        let name_addon = &instruction_name[first_underscore..instruction_name.len()];

        if name_addon.chars().last() != Some('_') {
            name_first_part + name_addon
        } else {
            name_first_part + &name_addon[0..name_addon.len() - 1]
        }
    }

    fn create_bits(bit_ranges: &Vec<BitRange>, preserve_zeroes: bool) -> u32 {
        let mut result: u32 = 0x00000000;
        for range in bit_ranges.iter() {
            for index in 0..range.bits.len() {
                let bit = match range.bits[index] {
                    Some(value) => {
                        if preserve_zeroes {
                            value
                        } else {
                            1
                        }
                    }
                    None => 0,
                };
                result |= bit << (range.hibit - index as u32);
            }
        }
        result
    }

    fn create_parameters(
        bit_ranges: &Vec<BitRange>,
        identification_mask: u32,
    ) -> Option<Vec<InstructionParameter>> {
        let result: Vec<InstructionParameter> = bit_ranges
            .iter()
            .filter_map(|range| InstructionParameter::try_create(range, identification_mask))
            .collect();

        if !result.is_empty() {
            Option::Some(result)
        } else {
            None
        }
    }
}
