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
    pub fn from(options_array: &OptionsArray, cli: &CLI) -> Self {
        let display_names_omit_prefix = options_array.iter()
                                                     .map(|dn| strip_prefixes(&dn, &cli.omit_prefix))
                                                     .collect::<Vec<_>>();

        //TODO if there is no prefix use Var id and print warning
        let common_prefix = common_str_prefix(&display_names_omit_prefix).trim_matches('_').to_string();

        let type_name = format!("{}_{}", cli.settings_master_name, common_prefix);
        let values = display_names_omit_prefix.iter()
                            .map(|dn| format!("{}_{}", cli.settings_master_name, dn))
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