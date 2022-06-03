pub trait ToWitcherScript {
    fn ws_type_name(&self) -> String;
    fn ws_code_body(&self) -> String;
}