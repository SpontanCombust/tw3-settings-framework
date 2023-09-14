use crate::{
    settings_var::SettingsVar, 
    traits::{WitcherScriptType, WitcherScript, WitcherScriptTypeDef}, 
    xml::group::Group, 
    utils::strip_prefixes, 
    settings_var_type::SettingsVarType, 
    constants::{
        GROUP_ID_VAR_NAME, 
        GROUP_DEFAULT_PRESET_VAR_NAME, 
        GROUP_VALIDATE_VALUES_PARSER_FUNC_NAME,
        GROUP_PARENT_CLASS, GROUP_READ_SETTINGS_PARSER_FUNC_NAME, 
        GROUP_WRITE_SETTINGS_PARSER_FUNC_NAME, 
        ReadSettingValueFnName, 
        WriteSettingValueFnName, 
        GROUP_ENUM_MAPPING_CONFIG_TO_UNIFIED_PARSER_FUNC_NAME, 
        GROUP_ENUM_MAPPING_UNIFIED_TO_CONFIG_PARSER_FUNC_NAME, 
        GROUP_ENUM_MAPPING_VALIDATE_PARSER_FUNC_NAME, 
        GROUP_ENUM_MAPPING_VALIDATE_FUNC_NAME,
    }
};

pub struct SettingsGroup {
    pub id: String, // id attribute in the Var node
    pub class_name: String, // name of the class for this group in WitcherScript
    pub var_name: String, // name of an instance of the class for this group in WitcherScript
    pub default_preset_index: u8,
    pub vars: Vec<SettingsVar>,
    pub validate_values: bool
}

impl SettingsGroup {
    pub fn try_from(xml_group: &Group, master_class_name: &str, prefixes: &Vec<String>, master_validate_values: bool) -> Result<Self, String> {    
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
        let validate_values = xml_group.validate.unwrap_or(master_validate_values);

        let mut setting_vars = Vec::<SettingsVar>::new();
        for xml_var in &xml_group.visible_vars {
            if !xml_var.ignore.unwrap_or(false) {
                if let Some(setting_var) = SettingsVar::try_from(xml_var, master_class_name, prefixes, validate_values)? {
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
            validate_values
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

        if self.validate_values {
            buffer.new_line();
            group_validate_values_function(self, buffer);
        }

        buffer.new_line();
        group_read_settings_function(self, buffer);

        buffer.new_line();
        group_write_settings_function(self, buffer);

        if self.has_enum_value_mappings() {
            buffer.new_line();
            enum_mapping_validate_function(self, buffer);

            buffer.new_line();
            enum_mapping_config_to_unified_function(self, buffer);

            buffer.new_line();
            enum_mapping_unified_to_config_function(self, buffer);
        }
    
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
    buffer.push_line(&format!("protected /* override */ function {}() : void", GROUP_VALIDATE_VALUES_PARSER_FUNC_NAME));
    buffer.push_line("{").push_indent();

    for var in &group.vars {
        if !var.validate_value {
            continue;
        }

        let validator = match &var.var_type {
            SettingsVarType::Int { min, max } => Some(format!("{v} = Clamp({v}, {min}, {max});", 
                                                                v = var.var_name)),
            SettingsVarType::Float { min, max } => Some(format!("{v} = ClampF({v}, {min}, {max});", 
                                                                  v = var.var_name)),
            SettingsVarType::Enum { val, val_mapping } => {
                if let Some(_) = val_mapping {
                    Some(format!("{v} = ({t}){f}('{vid}', (int){v});",
                                   v = var.var_name, 
                                   vid = var.id,
                                   t = val.type_name,
                                   f = GROUP_ENUM_MAPPING_VALIDATE_FUNC_NAME))
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

    buffer.pop_indent().push_line("}");
}

fn group_read_settings_function(group: &SettingsGroup, buffer: &mut WitcherScript) {
    buffer.push_line(&format!("protected /* override */ function {}(config: CInGameConfigWrapper) : void", GROUP_READ_SETTINGS_PARSER_FUNC_NAME));
    buffer.push_line("{").push_indent();

    for var in &group.vars {
        // add type cast if it's an enum
        let type_cast = if let SettingsVarType::Enum { val, .. } = &var.var_type {
            format!("({})", val.type_name)
        } else {
            "".into()
        };

        let read_setting_value = format!("{vn} = {tc}{func}(config, '{vid}');",
                                        vn = var.var_name,
                                        tc = type_cast,
                                        func = var.var_type.read_setting_value_fn(),
                                        vid = var.id);

        buffer.push_line(&read_setting_value);
    }

    buffer.pop_indent().push_line("}");
}

fn group_write_settings_function(group: &SettingsGroup, buffer: &mut WitcherScript) {
    buffer.push_line(&format!("protected /* override */ function {}(config: CInGameConfigWrapper) : void", GROUP_WRITE_SETTINGS_PARSER_FUNC_NAME))
          .push_line("{").push_indent();

    for var in &group.vars {
        // add type cast if it's an enum
        let type_cast = if let SettingsVarType::Enum {..} = &var.var_type {
            "(int)"
        } else {
            ""
        };

        let write_setting_value = format!("{func}(config, '{vid}', {tc}{vn});",
                                        func = var.var_type.write_setting_value_fn(),
                                        vid = var.id,
                                        tc = type_cast,
                                        vn = var.var_name);

        buffer.push_line(&write_setting_value);
    }
    
    buffer.pop_indent().push_line("}");
}

fn enum_mapping_function(group: &SettingsGroup, buffer: &mut WitcherScript, config_to_unified: bool) {
    buffer.push_line(&format!("protected /* override */ function {}(vId: name, val: int) : int", 
                                if config_to_unified {
                                    GROUP_ENUM_MAPPING_CONFIG_TO_UNIFIED_PARSER_FUNC_NAME
                                } else {
                                    GROUP_ENUM_MAPPING_UNIFIED_TO_CONFIG_PARSER_FUNC_NAME
                                }));

    buffer.push_line("{").push_indent();

    buffer.push_line("switch(vId)")
          .push_line("{");
    for var in &group.vars {
        if let Some(mapping) = if let SettingsVarType::Enum { val_mapping, .. } = &var.var_type { val_mapping } else { &None } {
            buffer.push_line(&format!("case '{}':", var.id)).push_indent();

            buffer.push_line("switch(val)")
                  .push_line("{");

            for i in 0..mapping.len() {
                let (k, v) = if config_to_unified { 
                    (i, mapping[i]) 
                } else { 
                    (mapping[i], i) 
                };
                buffer.push_line(&format!("case {}: return {};", k, v));
            }

            buffer.push_line("}");

            buffer.pop_indent();
        }
    }
    buffer.push_line("}");

    buffer.new_line();
    buffer.push_line("return -1;");

    buffer.pop_indent().push_line("}");
}

fn enum_mapping_config_to_unified_function(master: &SettingsGroup, buffer: &mut WitcherScript) {
    enum_mapping_function(master, buffer, true);
}

fn enum_mapping_unified_to_config_function(master: &SettingsGroup, buffer: &mut WitcherScript) {
    enum_mapping_function(master, buffer, false);
}

fn enum_mapping_validate_function(group: &SettingsGroup, buffer: &mut WitcherScript) {
    buffer.push_line(&format!("protected /* override */ function {}(vId: name, val: int) : int", GROUP_ENUM_MAPPING_VALIDATE_PARSER_FUNC_NAME))
          .push_line("{").push_indent();
    
    buffer.push_line("switch(vId)")
          .push_line("{");
    for var in &group.vars {
        if let Some(mapping) = if let SettingsVarType::Enum { val_mapping, .. } = &var.var_type { val_mapping } else { &None } {
            buffer.push_line(&format!("case '{}':", var.id)).push_indent();

            buffer.push_line("switch(val)")
                    .push_line("{");

            for i in 0..mapping.len() {
                buffer.push_line(&format!("case {}: ", mapping[i]));
            }

            buffer.push_indent()
                    .push_line("return val;")
                    .pop_indent()
                    .push_line("default:")
                    .push_indent()
                    .push_line(&format!("return {};", mapping[0]))
                    .pop_indent();

            buffer.push_line("}");

            buffer.pop_indent();
        }
    }
    buffer.push_line("}");

    buffer.new_line()
          .push_line("return 0;");
    
    buffer.pop_indent().push_line("}");
}