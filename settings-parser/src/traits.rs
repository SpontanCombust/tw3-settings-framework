use roxmltree::Node;

use crate::{cli::CLI, indented_document::IndentedDocument};

// To convert config XML into Rust types
pub trait FromXmlNode {
    fn from_xml_node(node: &Node, cli: &CLI) -> Result<Option<Self>, String> where Self: Sized;
}


pub type WitcherScript = IndentedDocument;

// To convert Rust types into WitcherScript types
pub trait ToWitcherScriptType {
    fn ws_type_name(&self) -> String;
    // Return true something was wrote into the buffer
    fn ws_type_definition(&self, buffer: &mut WitcherScript) -> bool;
}