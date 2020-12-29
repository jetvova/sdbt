use crate::BoxElement;

#[derive(Debug, Serialize)]
pub struct InstructionInfo {
    pub name: String,
    pub immutable_bits: u32,
    pub identification: u32,
}

impl InstructionInfo {
    pub fn new(
        instruction_name: &str,
        iclass_bits: &Vec<BoxElement>,
        encoding_specific_bits: &Option<Vec<BoxElement>>,
    ) -> Self {
        let name = Self::standardize_name(instruction_name);
        let immutable_bits = Self::create_bits(iclass_bits, encoding_specific_bits, false);
        let identification = Self::create_bits(iclass_bits, encoding_specific_bits, true);

        Self {
            name,
            immutable_bits,
            identification,
        }
    }

    fn standardize_name(instruction_name: &str) -> String {
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

    fn create_bits(
        iclass_bits: &Vec<BoxElement>,
        encoding_specific_bits: &Option<Vec<BoxElement>>,
        preserve_zeroes: bool,
    ) -> u32 {
        let mut result = Self::compute_boxes(iclass_bits, preserve_zeroes);
        if let Some(box_vector) = encoding_specific_bits {
            result |= Self::compute_boxes(box_vector, preserve_zeroes);
        }
        result
    }

    fn compute_boxes(box_vector: &Vec<BoxElement>, preserve_zeroes: bool) -> u32 {
        let mut result: u32 = 0x00000000;
        for box_element in box_vector.iter() {
            for index in 0..box_element.c.len() {
                let bit = match box_element.c[index].parse::<u32>() {
                    Ok(number) => {
                        if preserve_zeroes {
                            number
                        } else {
                            1
                        }
                    }
                    Err(_unparsable_value) => 0,
                };
                result |= bit << (box_element.hibit - index as i32);
            }
        }
        result
    }
}
