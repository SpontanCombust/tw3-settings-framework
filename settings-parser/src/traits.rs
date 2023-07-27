use crate::indented_document::IndentedDocument;


pub type WitcherScript = IndentedDocument;

// To convert Rust types into WitcherScript types
pub trait ToWitcherScriptType {
    fn ws_type_name(&self) -> String;
    // Return true something was wrote into the buffer
    fn ws_type_definition(&self, buffer: &mut WitcherScript) -> bool;
}