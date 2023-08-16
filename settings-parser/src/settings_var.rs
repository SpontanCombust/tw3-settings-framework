use crate::{
    settings_var_type::SettingsVarType, 
    traits::WitcherScriptType, 
    cli::CLI, 
    utils::strip_prefixes, 
    xml::var::Var
};

pub struct SettingsVar {
    pub id: String, // id attribute in the Var node
    pub var_name: String, // name of a variable inside a group class in WitcherScript
    pub var_type: SettingsVarType
}

impl SettingsVar {
    pub fn from(xml_var: &Var, master_class_name: &str, cli: &CLI) -> Option<Self> {
        SettingsVarType::from(xml_var, master_class_name, cli)
        .and_then(|var_type| Some(SettingsVar {
            id: xml_var.id.clone(),
            var_name: strip_prefixes(&xml_var.id, &cli.omit_prefix).trim_start_matches('_').into(),
            var_type
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