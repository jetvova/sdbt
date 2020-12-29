use crate::InstructionInfo;
use crate::InstructionSection;
use glob::glob;

use std::fmt::Debug;

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
                    if filename != "onebigfile.xml" && filename != "shared_pseudocode.xml" {
                        let contents = std::fs::read_to_string(&path)
                            .expect(&format!("Failed to read: {:?}", path));

                        // Parsing file
                        if contents.contains("<instructionsection") {
                            let xml_instruction =
                                serde_xml_rs::from_str::<InstructionSection>(&contents)
                                    .expect(&format!("Failed to parse: {:?}", path));

                            // Creating instruction info from file
                            for iclass in xml_instruction.classes.iclass.iter() {
                                for encoding in iclass.encoding.iter() {
                                    if encoding.name.is_empty() {
                                        println!(
                                            "WARNING: Empty Instruction name for Isection: {:?}",
                                            xml_instruction
                                        );
                                    } else {
                                        if &instruction_count % 250.0 == 0.0 {
                                            println!(
                                                "Instruction Conversion Progress {}%",
                                                &instruction_count / 25.0
                                            )
                                        }
                                        instructions.push(InstructionInfo::new(
                                            &encoding.name,
                                            &iclass.regdiagram.box_element,
                                            &encoding.optional_box_elements,
                                        ));

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
        println!("Sorting and printing to file...");
        instructions.sort_by_key(|instruction_info| {
            (
                -(instruction_info.immutable_bits as i64),
                instruction_info.identification,
            )
        });
        Self { instructions }
    }
}
