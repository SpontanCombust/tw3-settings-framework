use std::collections::HashMap;

use crate::{
    settings_group::SettingsGroup, 
    traits::{WitcherScriptType, WitcherScript, WitcherScriptTypeDef}, 
    settings_var_type::SettingsVarType, 
    cli::{CLI, OptionParsingMode}, xml::user_config::UserConfig, 
    settings_enum::{SettingsEnum, SettingsEnumValueMapping}
};

pub struct SettingsMaster {
    pub class_name: String, // name of the class in the WitcherScript
    pub mod_version: String,
    pub groups: Vec<SettingsGroup>,
    pub enums: Vec<SettingsEnum>,
    pub validate_values: bool,
    pub generate_getter: bool
}

impl SettingsMaster {
    pub fn from(xml_user_config: &UserConfig, cli: &CLI) -> Result<Self, String> {  
        let class_name = cli.settings_master_name.clone();
        let mod_version = cli.mod_version.clone();
        let validate_values = !cli.no_var_validation;
        let generate_getter = !cli.no_getter;
        
        let mut settings_groups = Vec::new();
        for group in xml_user_config.groups.iter() {
            settings_groups.push(SettingsGroup::from(&group, cli));
        }

        let mut settings_master = SettingsMaster {
            class_name,
            mod_version,
            groups: settings_groups,
            enums: Vec::new(),
            validate_values,
            generate_getter
        };

        if cli.option_parsing_mode != OptionParsingMode::Ints {
            let needs_enum_value_mappings = settings_master.fetch_enums(cli)?;

            if needs_enum_value_mappings {
                settings_master.create_enum_value_mappings();
            }
        }
  
        Ok(settings_master)
    }

    pub fn has_enum_value_mappings(&self) -> bool {
        self.groups.iter()
        .any(|g| g.has_enum_value_mappings())
    }

    // Returns whether enum value mapping is needed
    fn fetch_enums(&mut self, cli: &CLI) -> Result<bool, String> {
        // convenience struct that also contains group id and var id of the options var that enum was made from
        struct EnumData<'a> {
            group_id: &'a str,
            var_id: &'a str,
            val: &'a SettingsEnum
        }

        // map where the key is common enum prefix i.e. its type 
        let mut enum_prefix_map: HashMap<&str, Vec<EnumData>> = HashMap::new();
        for sg in self.groups.iter() {
            for sv in sg.vars.iter() {
                if let SettingsVarType::Enum { val, .. } = &sv.var_type {
                    let prefix = val.common_prefix.as_str();
                    let data = EnumData {
                        group_id: sg.id.as_str(),
                        var_id: sv.id.as_str(),
                        val
                    };

                    if let Some(data_vec) = enum_prefix_map.get_mut(prefix) {
                        data_vec.push(data);
                    } else {
                        enum_prefix_map.insert(prefix, vec![data]);
                    }
                }
            }
        }

        if enum_prefix_map.is_empty() {
            return Ok(false);
        }

        let mut enum_value_mapping_needed = false;
        for (_, data_vec) in enum_prefix_map.iter() {
            if data_vec.is_empty() {
                continue;
            } else if data_vec.len() == 1 {
                self.enums.push(data_vec[0].val.clone());
                continue;
            }
            
            // an enum type that will contain all possible values from OptionsArrays with the same common prefix 
            let mut unified_enum = data_vec[0].val.clone();
            for data in data_vec.iter().skip(1) {
                // comparison concerns not only the values inside, but how they are ordered
                if unified_enum.values != data.val.values {
                    if cli.option_parsing_mode == OptionParsingMode::EnumsStrict {
                        return Err(format!("Some OptionArrays have the same common prefix, but different sets of values. See {}::{} and {}::{}",
                                            data_vec[0].group_id, data_vec[0].var_id, data.group_id, data.var_id));
                    }

                    enum_value_mapping_needed = true;

                    for value in &data.val.values {
                        if !unified_enum.values.contains(value) {
                            unified_enum.values.push(value.clone());
                        }
                    }
                }
            }

            self.enums.push(unified_enum);
        }

        // enum definitions kept being parsed in different order between program runs
        self.enums.sort_by(|e1, e2| e1.type_name.cmp(&e2.type_name));

        Ok(enum_value_mapping_needed)
    }

    fn create_enum_value_mappings(&mut self) {
        let it = self.groups.iter_mut()
        .flat_map(|g| &mut g.vars)
        .map(|v| &mut v.var_type)
        .filter_map(|vt| {
            if let SettingsVarType::Enum { val, val_mapping } = vt {
                Some((val, val_mapping))
            } else {
                None
            }
        });
        
        for (val, val_mapping) in it {
            let mut mapping = SettingsEnumValueMapping::default();
            mapping.resize(val.values.len(), 0);

            let unified_enum = self.enums.iter()
                               .find(|e| e.common_prefix == val.common_prefix)
                               .expect(&format!("Unified enum type not found for enum {}", val.type_name));

            // no need for any specialized mapping if those types are exactly the same
            if val == unified_enum {
                *val_mapping = None;
                continue;
            }

            for i in 0..val.values.len() {
                for j in 0..unified_enum.values.len() {
                    if val.values[i] == unified_enum.values[j] {
                        mapping[i] = j;
                        break;
                    }
                }
            }

            *val_mapping = Some(mapping);
        }
    }
}




const MASTER_BASE_CLASS_NAME: &str = "ISettingsMaster";
const MASTER_MOD_VERSION_VAR_NAME: &str = "modVersion";
const MASTER_INIT_FUNC_NAME: &str = "Init";
const MASTER_VALIDATE_VALUES_FUNC_NAME: &str = "ValidateSettings";
const MASTER_READ_SETTINGS_FUNC_NAME: &str = "ReadSettings";
const MASTER_READ_SETTING_VALUE_FUNC_NAME: &str = "ReadSettingValue";
const MASTER_WRITE_SETTINGS_FUNC_NAME: &str = "WriteSettings";
const MASTER_WRITE_SETTING_VALUE_FUNC_NAME: &str = "WriteSettingValue";
const MASTER_RESET_SETTINGS_TO_DEFAULT_FUNC_NAME: &str = "ResetSettingsToDefault";
const MASTER_SHOULD_RESET_TO_DEFAULT_ON_INIT_FUNC_NAME: &str = "ShouldResetSettingsToDefaultOnInit";
const MASTER_ENUM_MAPPING_CONFIG_TO_UNIFIED_FUNC_NAME: &str = "EnumValueMappingConfigToUnified";
const MASTER_ENUM_MAPPING_UNIFIED_TO_CONFIG_FUNC_NAME: &str = "EnumValueMappingUnifiedToConfig";
const MASTER_ENUM_MAPPING_VALIDATE_FUNC_NAME: &str = "EnumValueMappingValidateUnified";
const GROUP_RESET_SETTINGS_TO_DEFAULT_FUNC_NAME: &str = "ResetToDefault";

impl WitcherScriptType for SettingsMaster {
    fn ws_type_name(&self) -> String {
        self.class_name.clone()
    }
}

impl WitcherScriptTypeDef for SettingsMaster {
    fn ws_type_definition(&self, buffer: &mut WitcherScript) {
        buffer.push_line(&format!("class {} extends {}", self.class_name, MASTER_BASE_CLASS_NAME));
        buffer.push_line("{").push_indent();
    
        default_variable_values(self, buffer);
        
        buffer.new_line();
        settings_class_variables(self, buffer);
    
        buffer.new_line();
        init_function(self, buffer);
    
        if self.validate_values {
            buffer.new_line();
            validate_values_function(self, buffer);
        }
        
        buffer.new_line();
        read_settings_function(self, buffer);
    
        buffer.new_line();
        write_settings_function(self, buffer);
    
        buffer.new_line();
        reset_settings_to_default_function(self, buffer);
    
        buffer.new_line();
        should_reset_to_default_on_init_function(self, buffer);

        if self.has_enum_value_mappings() {
            buffer.new_line();
            enum_mapping_config_to_unified_function(self, buffer);

            buffer.new_line();
            enum_mapping_unified_to_config_function(self, buffer);

            buffer.new_line();
            enum_mapping_validate_function(self, buffer);
        }
    
        buffer.pop_indent().push_line("}");
        buffer.new_line();

        for g in &self.groups {
            g.ws_type_definition(buffer);
            buffer.new_line();
        }

        for e in &self.enums {
            e.ws_type_definition(buffer);
            buffer.new_line();
        }

        if self.generate_getter {
            buffer.new_line();
            getter_convenience_function(self, buffer);
        }
    }

}

fn settings_class_variables(master: &SettingsMaster, buffer: &mut WitcherScript) {
    for group in &master.groups {
        buffer.push_line(&format!("public var {} : {};", group.var_name, group.ws_type_name()));
    }
}

fn default_variable_values(master: &SettingsMaster, buffer: &mut WitcherScript) {
    buffer.push_line(&format!("default {} = \"{}\";", MASTER_MOD_VERSION_VAR_NAME, master.mod_version));
}

fn init_function(master: &SettingsMaster, buffer: &mut WitcherScript) {
    buffer.push_line(&format!("public /* override */ function {}() : void", MASTER_INIT_FUNC_NAME));
    buffer.push_line("{").push_indent();

    for group in &master.groups {
        buffer.push_line(&format!("{n} = new {t} in this; {n}.Init(this);", n = group.var_name, t = group.ws_type_name()));
    }

    buffer.new_line();
    buffer.push_line(&format!("super.{}();", MASTER_INIT_FUNC_NAME));

    buffer.pop_indent().push_line("}");
}

fn validate_values_function(master: &SettingsMaster, buffer: &mut WitcherScript) {
    buffer.push_line(&format!("public /* override */ function {}() : void", MASTER_VALIDATE_VALUES_FUNC_NAME));
    buffer.push_line("{").push_indent();

    for group in &master.groups {
        let mut group_has_validation = false;
        for var in &group.vars {
            let validator = match &var.var_type {
                SettingsVarType::Int { min, max } => Some(format!("{g}.{v} = Clamp({g}.{v}, {min}, {max});", 
                                                                    g = group.var_name, v = var.var_name)),
                SettingsVarType::Float { min, max } => Some(format!("{g}.{v} = ClampF({g}.{v}, {min}, {max});", 
                                                                      g = group.var_name, v = var.var_name)),
                SettingsVarType::Enum { val, val_mapping } => {
                    if let Some(_) = val_mapping {
                        Some(format!("{g}.{v} = ({t}){f}('{gid}', '{vid}', (int){g}.{v});",
                                       g = group.var_name, v = var.var_name, 
                                       gid = group.id, vid = var.id,
                                       t = val.type_name,
                                       f = MASTER_ENUM_MAPPING_VALIDATE_FUNC_NAME))
                    } else {
                        Some(format!("{g}.{v} = ({t})Clamp((int){g}.{v}, {min}, {max});",
                                       g = group.var_name, v = var.var_name, 
                                       t = val.type_name,
                                       min = 0, max = val.values.len() - 1))
                    }
                } 
                
                _ => None,
            };

            if let Some(validator) = validator {
                buffer.push_line(&validator);
                group_has_validation = true;
            }
        }

        if group_has_validation {
            buffer.new_line();
        }
    }

    buffer.push_line(&format!("super.{}();", MASTER_VALIDATE_VALUES_FUNC_NAME));

    buffer.pop_indent().push_line("}");
}

fn read_settings_function(master: &SettingsMaster, buffer: &mut WitcherScript) {
    buffer.push_line(&format!("public /* override */ function {}() : void", MASTER_READ_SETTINGS_FUNC_NAME));
    buffer.push_line("{").push_indent();

    buffer.push_line("var config : CInGameConfigWrapper;");
    buffer.push_line("config = theGame.GetInGameConfigWrapper();");
    buffer.new_line();

    for group in &master.groups {
        for var in &group.vars {
            let mut get_var_value = format!("{}(config, '{}', '{}')", MASTER_READ_SETTING_VALUE_FUNC_NAME, group.id, var.id);

            // surround with type cast if necessary
            get_var_value = match &var.var_type {
                SettingsVarType::Bool => format!("StringToBool({})", get_var_value),
                SettingsVarType::Int {..} => format!("StringToInt({}, 0)", get_var_value),
                SettingsVarType::Float {..} => format!("StringToFloat({}, 0.0)", get_var_value),
                SettingsVarType::Enum { val, val_mapping } => {
                    get_var_value = format!("StringToInt({}, 0)", get_var_value);
                    if val_mapping.is_some() {
                        get_var_value = format!("{}('{}', '{}', {})", MASTER_ENUM_MAPPING_CONFIG_TO_UNIFIED_FUNC_NAME, group.id, var.id, get_var_value);
                    }
                    format!("({}){}", val.type_name, get_var_value)
                }
            };

            buffer.push_line(&format!("{}.{} = {};", group.var_name, var.var_name, get_var_value));
        }
        buffer.new_line();
    }

    if master.validate_values {
        buffer.push_line(&format!("this.{}();", MASTER_VALIDATE_VALUES_FUNC_NAME));
    }
    buffer.push_line(&format!("super.{}();", MASTER_READ_SETTINGS_FUNC_NAME));

    buffer.pop_indent().push_line("}");
}

fn write_settings_function(master: &SettingsMaster, buffer: &mut WitcherScript) {
    buffer.push_line(&format!("public /* override */ function {}() : void", MASTER_WRITE_SETTINGS_FUNC_NAME))
          .push_line("{").push_indent();

    buffer.push_line("var config : CInGameConfigWrapper;")
          .push_line("config = theGame.GetInGameConfigWrapper();")
          .new_line();

    if master.validate_values {
        buffer.push_line(&format!("this.{}();", MASTER_VALIDATE_VALUES_FUNC_NAME))
              .new_line();
    }

    for group in &master.groups {
        for var in &group.vars {
            let var_value_str = match &var.var_type {
                SettingsVarType::Bool => format!("BoolToString({}.{})", group.var_name, var.var_name),
                SettingsVarType::Int {..} => format!("IntToString({}.{})", group.var_name, var.var_name),
                SettingsVarType::Float {..} => format!("FloatToString({}.{})", group.var_name, var.var_name),
                SettingsVarType::Enum {val_mapping, ..} => { 
                    let mut var_value_str = format!("(int){}.{}", group.var_name, var.var_name);
                    if val_mapping.is_some() {
                        var_value_str = format!("{}('{}', '{}', {})", MASTER_ENUM_MAPPING_UNIFIED_TO_CONFIG_FUNC_NAME, group.id, var.id, var_value_str);
                    }
                    format!("IntToString({})", var_value_str)
                }
            };

            buffer.push_line(&format!("{}(config, '{}', '{}', {});", MASTER_WRITE_SETTING_VALUE_FUNC_NAME, group.id, var.id, var_value_str));
        }
        buffer.new_line();
    }

    buffer.push_line(&format!("super.{}();", MASTER_WRITE_SETTINGS_FUNC_NAME));
          
    buffer.pop_indent().push_line("}");
}

fn reset_settings_to_default_function(master: &SettingsMaster, buffer: &mut WitcherScript) {
    buffer.push_line(&format!("public /* override */ function {}() : void", MASTER_RESET_SETTINGS_TO_DEFAULT_FUNC_NAME))
          .push_line("{").push_indent();

    for group in &master.groups {
        buffer.push_line(&format!("{}.{}();", group.var_name, GROUP_RESET_SETTINGS_TO_DEFAULT_FUNC_NAME));
    }

    buffer.pop_indent().push_line("}");
}

fn should_reset_to_default_on_init_function(master: &SettingsMaster, buffer: &mut WitcherScript) {
    let group_id = &master.groups[0].id;
    let var_id = &master.groups[0].vars[0].id;

    buffer.push_line(&format!("public /* override */ function {}() : bool", MASTER_SHOULD_RESET_TO_DEFAULT_ON_INIT_FUNC_NAME))
          .push_line("{").push_indent();
    
    buffer.push_line("var config : CInGameConfigWrapper;")
          .push_line("config = theGame.GetInGameConfigWrapper();")
          .new_line()
          .push_line(&format!("return config.GetVarValue('{}','{}') == \"\";", group_id, var_id));
    
    buffer.pop_indent().push_line("}");
}

fn enum_mapping_function(master: &SettingsMaster, buffer: &mut WitcherScript, config_to_unified: bool) {
    buffer.push_line(&format!("public /* override */ function {}(gId: name, vId: name, val: int) : int", 
                                if config_to_unified {
                                    MASTER_ENUM_MAPPING_CONFIG_TO_UNIFIED_FUNC_NAME
                                } else {
                                    MASTER_ENUM_MAPPING_UNIFIED_TO_CONFIG_FUNC_NAME
                                }))
          .push_line("{").push_indent();


    buffer.push_line("switch(gId)")
          .push_line("{");

    for group in &master.groups {
        if group.has_enum_value_mappings() {
            buffer.push_line(&format!("case '{}':", group.id)).push_indent();
    
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

            buffer.pop_indent();
        }
    }

    buffer.push_line("}");

    buffer.new_line();
    if config_to_unified {
        buffer.push_line("return val;");
    } else {
        buffer.push_line("return 0;");
    }

    buffer.pop_indent().push_line("}");
}

fn enum_mapping_config_to_unified_function(master: &SettingsMaster, buffer: &mut WitcherScript) {
    enum_mapping_function(master, buffer, true);
}

fn enum_mapping_unified_to_config_function(master: &SettingsMaster, buffer: &mut WitcherScript) {
    enum_mapping_function(master, buffer, false);
}

fn enum_mapping_validate_function(master: &SettingsMaster, buffer: &mut WitcherScript) {
    buffer.push_line(&format!("public /* override */ function {}(gId: name, vId: name, val: int) : int", MASTER_ENUM_MAPPING_VALIDATE_FUNC_NAME))
          .push_line("{").push_indent();
    
    buffer.push_line("switch(gId)")
          .push_line("{");

    for group in &master.groups {
        if group.has_enum_value_mappings() {
            buffer.push_line(&format!("case '{}':", group.id)).push_indent();
    
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

            buffer.pop_indent();
        }
    }
    buffer.push_line("}");

    buffer.new_line()
          .push_line("return 0;");
    
    buffer.pop_indent().push_line("}");
}

fn getter_convenience_function(master: &SettingsMaster, buffer: &mut WitcherScript) {
    buffer.push_line(&format!("function Get{m}() : {m}", m=master.class_name))
          .push_line("{").push_indent()
          .push_line(&format!("var settings: {};", master.class_name))
          .new_line()
          .push_line(&format!("settings = ({m})GetSettingsMasterRegistry().GetSettings('{m}');", m=master.class_name))
          .push_line("if(!settings)")
          .push_line("{").push_indent()
          .push_line(&format!("settings = new {} in theGame;", master.class_name))
          .push_line(&format!("GetSettingsMasterRegistry().AddSettings(settings, '{}');", master.class_name))
          .pop_indent().push_line("}")
          .new_line()
          .push_line("return settings;")  
          .pop_indent().push_line("}");  
}  