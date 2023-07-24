pub trait ToWitcherScript {
    fn ws_type_name(&self) -> String;
    fn ws_code_body(&self) -> String; //TODO rename to ws_type_body, make it Optional, rename trait to ToWitcherScriptType
}