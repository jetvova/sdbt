use crate::BitRange;
use crate::BoxElement;
use crate::MergedEncoding;

#[derive(Debug, Serialize)]
pub struct InstructionParameter {
    pub name: String,
    pub width: u32,
    pub data_type: String,
    pub low_bit: u32,
    pub parameter_mask: u32,
}

impl InstructionParameter {
    pub fn try_create(
        bit_range: &BitRange,
        instruction_identification_mask: u32,
    ) -> Option<InstructionParameter> {
        if let Some(ref name) = bit_range.name {
            if bit_range.bits.iter().all(|bit| bit.is_none()) {
                let low_bit = bit_range.hibit - (bit_range.width - 1);
                let parameter_mask = 0xffffffff >> (32 - bit_range.width) << (low_bit);
                if parameter_mask & instruction_identification_mask == 0 {
                    return Some(Self {
                        name: name.to_owned(),
                        width: bit_range.width,
                        data_type: Self::data_type(bit_range.width),
                        low_bit: low_bit,
                        parameter_mask: parameter_mask,
                    });
                }
            }
        }
        None
    }

    fn data_type(width: u32) -> String {
        if width <= 8 {
            "u8"
        } else if width <= 16 {
            "u16"
        } else if width <= 32 {
            "u32"
        } else {
            panic!("Invalid Width")
        }
        .to_owned()
    }
}
