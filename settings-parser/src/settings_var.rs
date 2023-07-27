use crate::{var_type::VarType, traits::{ToWitcherScriptType, WitcherScript}, cli::CLI, utils::id_to_script_name, xml::var::Var};

pub struct SettingsVar {
    pub id: String, // id attribute in the Var node
    pub var_name: String, // name of a variable inside a group class in WitcherScript
    pub var_type: VarType
}

impl SettingsVar {
    pub fn from(xml_var: &Var, cli: &CLI) -> Option<Self> {
        if let Some(var_type) = VarType::from(xml_var, cli) {
            Some(SettingsVar {
                id: xml_var.id.clone(),
                var_name: id_to_script_name(&xml_var.id, &cli.omit_prefix),
                var_type
            })
        } else {
            None
        }        
    }
}



impl ToWitcherScriptType for SettingsVar {
    fn ws_type_name(&self) -> String {
        match &self.var_type {
            VarType::Bool => "bool".into(),
            VarType::Int {..} => "int".into(),
            VarType::Float {..} => "float".into(),
            VarType::Enum {name, ..} => name.clone(),
        }
    }

    fn ws_type_definition(&self, buffer: &mut WitcherScript) -> bool {
        match &self.var_type {
            VarType::Enum {name, values} => {
                buffer.push_line(&format!("enum {}", name))
                      .push_indent("{");
                
                for i in 0..values.len() {
                    buffer.push_line(&format!("{} = {},", values[i], i));
                }

                buffer.pop_indent("}");
                true
            },
            _ => false
        }
    }
}