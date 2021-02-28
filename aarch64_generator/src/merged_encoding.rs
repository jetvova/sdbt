use crate::BoxElement;

#[derive(Debug)]
pub struct MergedEncoding {
    pub encoding_name: String,
    pub bit_ranges: Vec<BitRange>,
}

impl MergedEncoding {
    pub fn new(
        encoding_name: &str,
        default_encoding: &Vec<BoxElement>,
        specific_encoding: &Option<Vec<BoxElement>>,
    ) -> Self {
        let mut bit_ranges: Vec<BitRange> = Vec::new();
        for default_box in default_encoding.iter() {
            bit_ranges.push(BitRange::new(default_box));
        }

        if let Some(specific_encoding) = specific_encoding {
            for specific_box in specific_encoding.iter() {
                for index_inside_box in 0..specific_box.c.len() {
                    match specific_box.c[index_inside_box].parse::<u32>() {
                        Ok(bit) => {
                            BitRange::set_bit(
                                &mut bit_ranges,
                                specific_box.hibit - index_inside_box as u32,
                                Some(bit),
                            );
                        }
                        Err(_) => (),
                    };
                }
            }
        }

        bit_ranges = bit_ranges
            .iter()
            .flat_map(|range| range.split_if_bits_reserved())
            .collect();

        Self {
            encoding_name: encoding_name.to_owned(),
            bit_ranges,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BitRange {
    pub hibit: u32,
    pub width: u32,
    pub name: Option<String>,
    pub constraint: Option<String>,
    pub bits: Vec<Option<u32>>,
}

impl BitRange {
    pub fn new(box_input: &BoxElement) -> Self {
        let bits: Vec<Option<u32>> = if box_input.c.len() == 1 && box_input.c[0] == "" {
            vec![None; box_input.width.unwrap_or(1) as usize]
        } else {
            box_input
                .c
                .iter()
                .map(|value| value.parse::<u32>().ok())
                .collect()
        };
        Self {
            hibit: box_input.hibit,
            width: box_input.width.unwrap_or(1),
            name: box_input.name.to_owned(),
            constraint: box_input.constraint.to_owned(),
            bits,
        }
    }

    pub fn set_bit(target_vector: &mut Vec<BitRange>, index: u32, new_value: Option<u32>) {
        for bit_range in target_vector.iter_mut() {
            if bit_range.in_range(index) {
                let old_value = bit_range.bits[(bit_range.hibit - index) as usize];
                if old_value.is_some() && old_value != new_value {
                    panic!();
                } else {
                    bit_range.bits[(bit_range.hibit - index) as usize] = new_value;
                    return;
                }
            }
        }
        panic!();
    }

    fn in_range(&self, index: u32) -> bool {
        self.hibit >= index && (self.hibit - (self.width - 1)) <= index
    }

    pub fn split_if_bits_reserved(&self) -> Vec<Self> {
        if self.bits.iter().all(|bit| bit.is_some()) || self.bits.iter().all(|bit| bit.is_none()) {
            vec![self.clone()]
        } else {
            let mut split_result: Vec<Self> = Vec::new();
            for index in 0..self.bits.len() {
                let new_hibit = self.hibit - index as u32;
                if self.bits[index].is_some() {
                    split_result.push(Self::new_single_width(new_hibit, self.bits[index], None));
                } else {
                    if let Some(range) = split_result
                        .iter_mut()
                        .last()
                        .filter(|range| range.name.is_some())
                    {
                        range.width += 1;
                        range.bits.push(None);
                    } else {
                        split_result.push(Self::new_single_width(
                            new_hibit,
                            None,
                            self.name.clone(),
                        ));
                    }
                }
            }
            split_result
        }
    }

    fn new_single_width(hibit: u32, value: Option<u32>, name: Option<String>) -> Self {
        Self {
            hibit,
            width: 1,
            name,
            constraint: None,
            bits: vec![value],
        }
    }
}
