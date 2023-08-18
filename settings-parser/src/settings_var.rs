use crate::{
    settings_var_type::SettingsVarType, 
    traits::WitcherScriptType, 
    utils::strip_prefixes, 
    xml::var::Var
};

pub struct SettingsVar {
    pub id: String, // id attribute in the Var node
    pub var_name: String, // name of a variable inside a group class in WitcherScript
    pub var_type: SettingsVarType
}

impl SettingsVar {
    pub fn from(xml_var: &Var, master_class_name: &str, prefixes: &Vec<String>) -> Result<Option<Self>, String> {
        let var_name = if let Some(variable_name) = &xml_var.variable_name {
            variable_name.clone()
        } else {
            strip_prefixes(&xml_var.id, prefixes).trim_start_matches('_').into()
        };

        let svt = SettingsVarType::from(xml_var, master_class_name, prefixes)?;

        Ok(svt.and_then(|var_type| Some(SettingsVar {
            id: xml_var.id.clone(),
            var_name,
            var_type
        })))
    }
}



impl WitcherScriptType for SettingsVar {
    fn ws_type_name(&self) -> String {
        match &self.var_type {
            SettingsVarType::Bool => "bool".into(),
            SettingsVarType::Int {..} => "int".into(),
            SettingsVarType::Float {..} => "float".into(),
            SettingsVarType::Enum { val, .. } => val.type_name.clone(),
        }
    }
}