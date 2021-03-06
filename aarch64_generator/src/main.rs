use failure::Error;
use handlebars::Handlebars;
use std::fs;

mod xml_structure;
pub use xml_structure::*;

mod instruction_collection;
pub use instruction_collection::*;

mod instruction_info;
pub use instruction_info::*;

mod instruction_parameter;
pub use instruction_parameter::*;

mod merged_encoding;
pub use merged_encoding::*;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate handlebars;

handlebars_helper!(hex: |v: i64| format!("0x{:08x}", v));
fn main() -> Result<(), Error> {
    let collection = InstructionColletion::new(
        &std::env::args()
            .nth(1)
            .expect("Documentation path not provided"),
    );
    let src_directory = "aarch64_generator/src";
    let output_directory = "aarch64/src";

    // Loading Handlebars template
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string(
            "instruction",
            fs::read_to_string(format!("{}/instruction.hbs", src_directory)).unwrap(),
        )
        .unwrap();
    handlebars.register_helper("hex", Box::new(hex));

    // Writing .rs file using template
    fs::write(
        format!("{}/instruction.rs", output_directory),
        handlebars.render("instruction", &collection).unwrap(),
    )
    .unwrap();

    handlebars = Handlebars::new();
    handlebars
        .register_template_string(
            "test",
            fs::read_to_string(format!("{}/test.hbs", src_directory)).unwrap(),
        )
        .unwrap();
    handlebars.register_helper("hex", Box::new(hex));

    fs::write(
        format!("{}/test.rs", output_directory),
        handlebars.render("test", &collection).unwrap(),
    )
    .unwrap();

    Ok(())
}
