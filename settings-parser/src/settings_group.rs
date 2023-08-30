use crate::{
    settings_var::SettingsVar, 
    traits::{WitcherScriptType, WitcherScript, WitcherScriptTypeDef}, 
    xml::group::Group, 
    utils::strip_prefixes, settings_var_type::SettingsVarType
};

pub struct SettingsGroup {
    pub id: String, // id attribute in the Var node
    pub class_name: String, // name of the class for this group in WitcherScript
    pub var_name: String, // name of an instance of the class for this group in WitcherScript
    pub default_preset_index: u8,
    pub vars: Vec<SettingsVar>
}

impl SettingsGroup {
    pub fn try_from(xml_group: &Group, master_class_name: &str, prefixes: &Vec<String>) -> Result<Self, String> {    
        let default_preset_index = xml_group.default_preset_index.unwrap_or(0);
        if xml_group.presets_array.len() > 0 && default_preset_index as usize >= xml_group.presets_array.len() {
            return Err(format!("Invalid default preset index in Group {}", xml_group.id));
        }

        let id = xml_group.id.clone();
        let var_name = if let Some(variable_name) = &xml_group.variable_name {
            variable_name.clone()
        } else {
            strip_prefixes(&id, prefixes).trim_start_matches('_').into()
        };
        let class_name = format!("{}_{}", master_class_name, var_name); //xml_group.class_name.as_ref().unwrap_or(&format!("{}_{}", master_class_name, var_name)).to_string();
        let mut setting_vars = Vec::<SettingsVar>::new();

        for xml_var in &xml_group.visible_vars {
            if !xml_var.ignore.unwrap_or(false) {
                if let Some(setting_var) = SettingsVar::try_from(xml_var, master_class_name, prefixes)? {
                    setting_vars.push(setting_var);
                }
            }
        }

        Ok(SettingsGroup {
            id,
            class_name,
            var_name,
            default_preset_index,
            vars: setting_vars,
        })
    }

    pub fn has_enum_value_mappings(&self) -> bool {
        self.vars.iter()
        .filter_map(|v| {
            if let SettingsVarType::Enum { val_mapping, .. } = &v.var_type {
                Some(val_mapping)
            } else {
                None
            }
        }).any(|m| m.is_some())
    }
}


//TODO move these to some seperate 'constants' module
const GROUP_PARENT_CLASS: &str = "ISettingsGroup";
const GROUP_PARENT_MASTER_VAR_NAME: &str = "m_parentMaster";
const GROUP_ID_VAR_NAME: &str = "id";
const GROUP_DEFAULT_PRESET_VAR_NAME: &str = "defaultPresetIndex";
const GROUP_VALIDATE_VALUES_FUNC_NAME: &str = "Validate";
const MASTER_ENUM_MAPPING_VALIDATE_FUNC_NAME: &str = "EnumValueMappingValidateUnified";

impl WitcherScriptType for SettingsGroup {
    fn ws_type_name(&self) -> String {
        self.class_name.clone()
    }

}

impl WitcherScriptTypeDef for SettingsGroup {
    fn ws_type_definition(&self, buffer: &mut WitcherScript) {
        buffer.push_line(&format!("class {} extends {}", self.ws_type_name(), GROUP_PARENT_CLASS));
        buffer.push_line("{").push_indent();
    
        group_class_variables(self, buffer);
        
        buffer.new_line();
        group_default_variable_values(self, buffer);

        buffer.new_line();
        group_validate_values_function(self, buffer);
    
        buffer.pop_indent().push_line("}");
    }
}

fn group_class_variables(group: &SettingsGroup, buffer: &mut WitcherScript) {
    for var in &group.vars {
        buffer.push_line(&format!("public var {} : {};", var.var_name, var.ws_type_name()));
    }
}

fn group_default_variable_values(group: &SettingsGroup, buffer: &mut WitcherScript) {
    buffer.push_line(&format!("default {} = '{}';", GROUP_ID_VAR_NAME, group.id))
          .push_line(&format!("default {} = {};", GROUP_DEFAULT_PRESET_VAR_NAME, group.default_preset_index));
}

fn group_validate_values_function(group: &SettingsGroup, buffer: &mut WitcherScript) {
    buffer.push_line(&format!("public /* override */ function {}() : void", GROUP_VALIDATE_VALUES_FUNC_NAME));
    buffer.push_line("{").push_indent();

    for var in &group.vars {
        let validator = match &var.var_type {
            SettingsVarType::Int { min, max } => Some(format!("{v} = Clamp({v}, {min}, {max});", 
                                                                v = var.var_name)),
            SettingsVarType::Float { min, max } => Some(format!("{v} = ClampF({v}, {min}, {max});", 
                                                                  v = var.var_name)),
            SettingsVarType::Enum { val, val_mapping } => {
                if let Some(_) = val_mapping {
                    Some(format!("{v} = ({t}){p}.{f}('{gid}', '{vid}', (int){v});",
                                   v = var.var_name, 
                                   gid = group.id, vid = var.id,
                                   t = val.type_name,
                                   p = GROUP_PARENT_MASTER_VAR_NAME,
                                   f = MASTER_ENUM_MAPPING_VALIDATE_FUNC_NAME))
                } else {
                    Some(format!("{v} = ({t})Clamp((int){v}, {min}, {max});",
                                   v = var.var_name, 
                                   t = val.type_name,
                                   min = 0, max = val.values.len() - 1))
                }
            } 
            
            _ => None,
        };

        if let Some(validator) = validator {
            buffer.push_line(&validator);
        }
    }

    buffer.new_line();
    buffer.push_line(&format!("super.{}();", GROUP_VALIDATE_VALUES_FUNC_NAME));

    buffer.pop_indent().push_line("}");
}