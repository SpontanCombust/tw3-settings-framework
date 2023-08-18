use crate::{
    xml::options_array::OptionsArray, 
    utils::{common_str_prefix, strip_prefixes}, 
    traits::{WitcherScriptType, WitcherScriptTypeDef}
};

#[derive(Clone, PartialEq, Eq)]
pub struct SettingsEnum {
    pub type_name: String,
    pub values: Vec<String>,
}

impl SettingsEnum {
    //TODO rename to try_from, to the same with other types
    pub fn from(options_array: &OptionsArray, var_id: &str, master_class_name: &str, prefixes: &Vec<String>) -> Result<Self, String> {
        if let Some(enum_name) = &options_array.enum_type {
            let mut values = Vec::<String>::new();
            for i in 0..options_array.options.len() {
                let opt = &options_array.options[i];
                if let Some(val) = &opt.enum_value_suffix {
                    values.push(format!("{}{}", enum_name, val));
                } else {
                    //TODO would be good to have a full on validation layer for xml types
                    return Err(format!("Option {} in Var {} does not have msfEnumValue specified, which is needed in this context", i, var_id));
                }
            }

            Ok(SettingsEnum { 
                type_name: enum_name.clone(), 
                values 
            })
        } else {
            // with stripped mod prefix
            let options_array = options_array.options.iter()
                                .map(|o| strip_prefixes(&o.display_name, prefixes).trim_start_matches('_').to_string())
                                .collect::<Vec<_>>();
    
            let common_prefix = common_str_prefix(&options_array).to_string();
    
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
                return Err(format!("OptionsArray for var {} doesn't have common displayName prefix in values, which is needed in this context.", var_id));
            }
    
            let type_name = format!("{}_{}", master_class_name, common_prefix.trim_end_matches('_'));
            let values = options_array_suffixes.iter()
                         .map(|suffix| format!("{}_{}{}", master_class_name, common_prefix, suffix))
                         .collect::<Vec<_>>();
    
            Ok(SettingsEnum {
                type_name,
                values
            })
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
              .push_line("{").push_indent();
        
        for i in 0..self.values.len() {
            buffer.push_line(&format!("{} = {},", self.values[i], i));
        }
    
        buffer.pop_indent().push_line("}");
    }
}


/// Describes the relationship between values in the unified enum and indices in the specific var in UserConfig.
/// This vec should be the same size as the `values` vec in SettingsEnum.
/// For each element in said 'values' vec it attributes the value of element in the unified enum for this enum type.
pub type SettingsEnumValueMapping = Vec<usize>;