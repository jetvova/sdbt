use failure::Error;
use handlebars::Handlebars;
use std::fs;

mod xml_structure;
pub use xml_structure::*;

mod instruction_collection;
pub use instruction_collection::*;

mod instruction_info;
pub use instruction_info::*;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate handlebars;

handlebars_helper!(hex: |v: i64| format!("0x{:x}", v));
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

    Ok(())
}
