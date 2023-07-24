use roxmltree::Node;

use crate::cli::CLI;

// To convert config XML into Rust types
pub trait FromXMLNode {
    fn from_xml_node(node: &Node, cli: &CLI) -> Result<Option<Self>, String> where Self: Sized;
}

// To convert Rust types into WitcherScript types
pub trait ToWitcherScript {
    fn ws_type_name(&self) -> String;
    // Return true something was wrote into the buffer
    fn ws_type_definition(&self, buffer: &mut String) -> bool;
}