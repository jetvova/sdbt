use crate::InstructionInfo;
use crate::InstructionParameter;
use crate::InstructionSection;
use glob::glob;

use std::fmt::Debug;

use crate::MergedEncoding;
#[derive(Debug, Serialize)]
pub struct InstructionColletion {
    pub instructions: Vec<InstructionInfo>,
}

impl InstructionColletion {
    pub fn new(documentation_path: &str) -> Self {
        println!("Current directory: {:?}", std::env::current_dir().unwrap());
        println!("Documentation path: {:?}", documentation_path);
        let mut instructions = Vec::new();
        let mut instruction_count = 0.0;

        let pattern = &format!("{}/*.xml", documentation_path);
        for entry in glob(pattern).expect("Failed to read glob pattern") {
            match entry {
                Err(e) => panic!("{:?}", e),
                Ok(path) => {
                    // Opening file
                    let filename = path.file_name().unwrap();
                    if filename != "onebigfile.xml"
                        && filename != "shared_pseudocode.xml"
                        && filename != "enumerated-symbol-accounts.xml"
                    {
                        let contents = std::fs::read_to_string(&path)
                            .expect(&format!("Failed to read: {:?}", path));

                        // Parsing file
                        if contents.contains("<instructionsection") {
                            let xml_instruction =
                                serde_xml_rs::from_str::<InstructionSection>(&contents)
                                    .expect(&format!("Failed to parse: {:?}", path));

                            if xml_instruction.aliasto.is_none() {
                                // Skipping instruction if it is an alias.
                                // Creating instruction info from file
                                for iclass in xml_instruction.classes.iclass.iter() {
                                    for encoding in iclass.encoding.iter() {
                                        if encoding.name.is_empty() {
                                            println!(
                                            "WARNING: Empty Instruction name for Isection: {:?}",
                                            xml_instruction
                                        );
                                        } else {
                                            let merged_encoding = MergedEncoding::new(
                                                &encoding.name,
                                                &iclass.regdiagram.box_elements,
                                                &encoding.optional_box_elements,
                                            );

                                            instructions
                                                .push(InstructionInfo::new(merged_encoding));

                                            if &instruction_count % 250.0 == 0.0 {
                                                println!(
                                                    "Instruction Conversion Progress {}%",
                                                    &instruction_count / 25.0
                                                )
                                            }
                                            instruction_count += 1.0;
                                            // Workaround for an apparant error in MOV documentation
                                            if encoding.name == "MOV_dup_z_zi_" {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("Sorting and printing to file...");
        // Using instruction identification and constraint hamming weights to ensure the most specific encodings
        // are checked first.
        instructions.sort_by_key(|instruction_info| {
            (
                -(instruction_info.identification.mask.count_ones() as i8),
                -(Self::constraint_mask_hamming_weight(&instruction_info.parameters)),
                instruction_info.identification.value,
            )
        });
        Self { instructions }
    }

    pub fn constraint_mask_hamming_weight(parameters: &Option<Vec<InstructionParameter>>) -> i8 {
        let mut combined_constraints_mask: u32 = 0;
        if let Some(parameters) = parameters {
            for parameter in parameters {
                if let Some(constraint) = &parameter.constraint {
                    combined_constraints_mask |= constraint.mask;
                }
            }
        }
        combined_constraints_mask.count_ones() as i8
    }
}
