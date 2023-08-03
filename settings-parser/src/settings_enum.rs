use crate::{
    xml::display_type::OptionsArray, 
    cli::CLI, 
    utils::{common_str_prefix, strip_prefixes}, 
    traits::{WitcherScriptType, WitcherScriptTypeDef}
};

#[derive(Clone, PartialEq, Eq)]
pub struct SettingsEnum {
    pub common_prefix: String,
    pub type_name: String,
    pub values: Vec<String>,
}

impl SettingsEnum {
    pub fn from(options_array: &OptionsArray, var_id: &str, cli: &CLI) -> Self {
        // with stripped mod prefix
        let options_array = options_array.iter()
                            .map(|o| strip_prefixes(o, &cli.omit_prefix).trim_start_matches('_').to_string())
                            .collect::<Vec<_>>();

        let mut common_prefix = common_str_prefix(&options_array).to_string();

        let options_array_suffixes = if common_prefix.is_empty() {
            options_array.iter()
            .map(|dn| dn.as_str())
            .collect::<Vec<_>>()
        } else {
            options_array.iter()
            .map(|dn| dn.strip_prefix(&common_prefix).unwrap())
            .collect::<Vec<_>>()
        };

        if common_prefix.is_empty() {
            println!("Warning! OptionsArray for var {} does not have a common prefix. Var id will be used instead.", var_id);
            common_prefix = format!("{}_", strip_prefixes(var_id, &cli.omit_prefix).trim_matches('_'));
        }

        let type_name = format!("{}_{}", cli.settings_master_name, common_prefix.trim_end_matches('_'));
        let values = options_array_suffixes.iter()
                     .map(|suffix| format!("{}_{}{}", cli.settings_master_name, common_prefix, suffix))
                     .collect::<Vec<_>>();

        SettingsEnum { 
            common_prefix,
            type_name,
            values
        }
    }
}


impl WitcherScriptType for SettingsEnum {
    fn ws_type_name(&self) -> String {
        self.type_name.clone()
    }
}

impl WitcherScriptTypeDef for SettingsEnum {
    fn ws_type_definition(&self, buffer: &mut crate::traits::WitcherScript) {
        buffer.push_line(&format!("enum {}", self.type_name))
              .push_indent("{");
        
        for i in 0..self.values.len() {
            buffer.push_line(&format!("{} = {},", self.values[i], i));
        }
    
        buffer.pop_indent("}");
    }
}