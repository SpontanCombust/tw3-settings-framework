use crate::{
    settings_var_type::SettingsVarType, 
    traits::WitcherScriptType, 
    utils::strip_prefixes, 
    xml::{var::Var, display_type::DisplayType}
};

pub struct SettingsVar {
    pub id: String, // id attribute in the Var node
    pub var_name: String, // name of a variable inside a group class in WitcherScript
    pub var_type: SettingsVarType,
    pub var_not_found_value: String, // value that gets read if a given setting does not yet exist in user.settings
    pub validate_value: bool
}

impl SettingsVar {
    pub fn try_from(xml_var: &Var, master_class_name: &str, prefixes: &Vec<String>, group_validate_values: bool) -> Result<Option<Self>, String> {
        let svt;
        match SettingsVarType::from(xml_var, master_class_name, prefixes)? {
            Some(v) => svt = v,
            None => return Ok(None),
        }

        let var_name = if let Some(variable_name) = &xml_var.variable_name {
            variable_name.clone()
        } else {
            strip_prefixes(&xml_var.id, prefixes).trim_start_matches('_').into()
        };

        let validate_value = xml_var.validate.unwrap_or(group_validate_values);

        let var_not_found_value = match xml_var.display_type {
            DisplayType::Options(_) => "-1".to_string(),
            _ => "".to_string()
        };

        Ok(Some(SettingsVar {
            id: xml_var.id.clone(),
            var_name,
            var_type: svt,
            var_not_found_value,
            validate_value
        }))
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