use crate::BitRange;

#[derive(Debug, Serialize)]
pub struct InstructionParameter {
    pub name: String,
    pub width: u32,
    pub data_type: String,
    pub low_bit: u32,
    pub parameter_mask: u32,
    pub constraint: Option<MaskAndValue>,
    pub example_value: u32,
}

#[derive(Debug, Serialize)]
pub struct MaskAndValue {
    pub mask: u32,
    pub value: u32,
}

impl InstructionParameter {
    pub fn try_create(
        bit_range: &BitRange,
        instruction_identification_mask: u32,
    ) -> Option<InstructionParameter> {
        if let Some(ref name) = bit_range.name {
            if bit_range.bits.iter().all(|bit| bit.is_none()) {
                let low_bit = bit_range.hibit - (bit_range.width - 1);
                let parameter_mask = 0xffffffff >> (32 - bit_range.width) << low_bit;
                if parameter_mask & instruction_identification_mask == 0 {
                    let constraint = Self::find_constraint(bit_range, low_bit);
                    let example_value = match &constraint {
                        None => parameter_mask, // Unconstrained parameters become equal to their size in ones to help with cathing mistakes during round trip testing.
                        Some(c) => (!c.value & parameter_mask), // Inverting constraint in order to get an example value that would be satisfactory.
                    } >> low_bit;
                    return Some(Self {
                        name: name.to_owned(),
                        width: bit_range.width,
                        data_type: Self::data_type(bit_range.width),
                        low_bit: low_bit,
                        parameter_mask: parameter_mask,
                        constraint: constraint,
                        example_value: example_value,
                    });
                }
            }
        }
        None
    }

    fn find_constraint(bit_range: &BitRange, low_bit: u32) -> Option<MaskAndValue> {
        if let Some(constraint_string) = &bit_range.constraint {
            let constraint_string = &constraint_string[3..];

            let mask_string = constraint_string.replace("0", "1").replace("x", "0");
            let mask = u32::from_str_radix(&mask_string, 2).unwrap() << low_bit;

            let value_string = constraint_string.replace("x", "0");
            let value = u32::from_str_radix(&value_string, 2).unwrap() << low_bit;
            return Some(MaskAndValue { mask, value });
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
