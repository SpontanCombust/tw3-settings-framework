use crate::indented_document::IndentedDocument;



pub trait WitcherScriptType {
    fn ws_type_name(&self) -> String;
}

pub type WitcherScript = IndentedDocument;

pub trait WitcherScriptTypeDef : WitcherScriptType {
    fn ws_type_definition(&self, buffer: &mut WitcherScript);
}