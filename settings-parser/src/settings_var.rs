use crate::{var_type::VarType, to_witcher_script::ToWitcherScript};

pub struct SettingsVar {
    pub id: String,
    pub name: String,
    pub var_type: VarType
}


impl ToWitcherScript for SettingsVar {
    fn ws_type_name(&self) -> String {
        match &self.var_type {
            VarType::Toggle => String::from("bool"),
            VarType::Options => String::from("int"),
            VarType::SliderInt => String::from("int"),
            VarType::SliderFloat => String::from("float"),
        }
    }

    fn ws_code_body(&self) -> String {
        format!("var {} : {}", self.name, self.ws_type_name())
    }
}