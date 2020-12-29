#[derive(Debug, Deserialize)]
pub struct InstructionSection {
    pub classes: Classes,
}

#[derive(Debug, Deserialize)]
pub struct Classes {
    pub iclass: Vec<Iclass>,
}

#[derive(Debug, Deserialize)]
pub struct Iclass {
    pub regdiagram: RegDiagram,
    pub encoding: Vec<Encoding>,
}

#[derive(Debug, Deserialize)]
pub struct RegDiagram {
    #[serde(rename = "box")]
    pub box_element: Vec<BoxElement>,
}

#[derive(Debug, Deserialize)]
pub struct Encoding {
    pub name: String,
    #[serde(rename = "box")]
    pub optional_box_elements: Option<Vec<BoxElement>>,
}

#[derive(Debug, Deserialize)]
pub struct BoxElement {
    pub hibit: i32,
    pub c: Vec<String>,
}