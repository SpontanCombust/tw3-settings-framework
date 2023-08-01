use std::ops::Deref;

use crate::{
    settings_group::SettingsGroup, 
    traits::{WitcherScriptType, WitcherScript, WitcherScriptTypeDef}, 
    settings_var_type::SettingsVarType, 
    cli::CLI, xml::user_config::UserConfig, 
    settings_enum::SettingsEnum
};

pub struct SettingsMaster {
    pub class_name: String, // name of the class in the WitcherScript
    pub mod_version: String,
    pub groups: Vec<SettingsGroup>,
    pub enums: Vec<SettingsEnum>,
    pub validate_values: bool
}

impl SettingsMaster {
    pub fn from(xml_user_config: &UserConfig, cli: &CLI) -> Result<Self, String> {  
        
        let class_name = cli.settings_master_name.clone();
        let mod_version = cli.mod_version.clone();
        let validate_values = !cli.no_var_validation;
        
        let mut settings_groups = Vec::new();
        for group in xml_user_config.groups.iter() {
            settings_groups.push(SettingsGroup::from(&group, cli));
        }

        let settings_enums = Self::fetch_enums(&settings_groups)?;
  
        Ok(SettingsMaster {
            class_name,
            mod_version,
            groups: settings_groups,
            enums: settings_enums,
            validate_values,
        })
    }

    fn fetch_enums(settings_groups: &Vec<SettingsGroup>) -> Result<Vec<SettingsEnum>, String> {
        let mut enums_hierarchy = Vec::new();
        for sg in settings_groups.iter() {
            for sv in sg.vars.iter() {
                if let SettingsVarType::Enum(se) = &sv.var_type {
                    enums_hierarchy.push((sg, sv, se));
                }
            }
        }

        // make the ones having the same common prefix be next to each other
        enums_hierarchy.sort_by(|(_, _, e1), (_, _, e2)| e1.common_prefix.cmp(&e2.common_prefix));

        // check whether there are any enums without common prefix (and thus invalid type name)
        for (sg, sv, se) in enums_hierarchy.iter() {
            if se.common_prefix.is_empty() {
                return Err(format!("OptionsArray for var {} in group {} does not have a common prefix", sv.id, sg.id));
            }
        }  

        // check whether same type enums have the same sets of values
        // different sets of values are not supported (yet)
        for i in 0..enums_hierarchy.len() - 1 {
            let h1 = enums_hierarchy[i];
            let h2 = enums_hierarchy[i + 1];
            if h1.2.common_prefix == h2.2.common_prefix && h1.2.values != h2.2.values {
                return Err(format!("Some OptionArrays have the same common prefix, but different sets of values. See {}::{} and {}::{}",
                                    h1.0.id, h1.1.id, h2.0.id, h2.1.id));
            }
        }

        // vec is sorted so this will leave only one ref to each enum type
        enums_hierarchy.dedup_by(|(_, _, e1), (_, _, e2)| e1.common_prefix == e2.common_prefix);

        let fetched_enums = enums_hierarchy.iter()
                            .map(|(_, _, e)| e.deref().clone())
                            .collect();

        Ok(fetched_enums)
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
const GROUP_RESET_SETTINGS_TO_DEFAULT_FUNC_NAME: &str = "ResetToDefault";

impl WitcherScriptType for SettingsMaster {
    fn ws_type_name(&self) -> String {
        self.class_name.clone()
    }
}

impl WitcherScriptTypeDef for SettingsMaster {
    fn ws_type_definition(&self, buffer: &mut WitcherScript) {
        buffer.push_line(&format!("class {} extends {}", self.class_name, MASTER_BASE_CLASS_NAME));
        buffer.push_indent("{");
    
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
    
        buffer.pop_indent("}");
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
    buffer.push_indent("{");

    for group in &master.groups {
        buffer.push_line(&format!("{n} = new {t} in this; {n}.Init(this);", n = group.var_name, t = group.ws_type_name()));
    }

    buffer.new_line();
    buffer.push_line(&format!("super.{}();", MASTER_INIT_FUNC_NAME));

    buffer.pop_indent("}");
}

fn validate_values_function(master: &SettingsMaster, buffer: &mut WitcherScript) {
    buffer.push_line(&format!("public /* override */ function {}() : void", MASTER_VALIDATE_VALUES_FUNC_NAME));
    buffer.push_indent("{");

    for group in &master.groups {
        let mut group_has_validation = false;
        for var in &group.vars {
            let validator = match &var.var_type {
                SettingsVarType::Int { min, max } => Some(format!("{g}.{v} = Clamp({g}.{v}, {min}, {max});", 
                                                            g = group.var_name, v = var.var_name)),
                SettingsVarType::Float { min, max } => Some(format!("{g}.{v} = ClampF({g}.{v}, {min}, {max});", 
                                                              g = group.var_name, v = var.var_name)),
                SettingsVarType::Enum (settings_enum) => Some(format!("{g}.{v} = ({t})Clamp((int){g}.{v}, {min}, {max});",
                                                                         g = group.var_name, v = var.var_name, 
                                                                         t = settings_enum.type_name,
                                                                         min = 0, max = settings_enum.values.len() - 1)),
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

    buffer.pop_indent("}");
}

fn read_settings_function(master: &SettingsMaster, buffer: &mut WitcherScript) {
    buffer.push_line(&format!("public /* override */ function {}() : void", MASTER_READ_SETTINGS_FUNC_NAME));
    buffer.push_indent("{");

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
                SettingsVarType::Enum (settings_enum) => format!("({})StringToInt({}, 0)", settings_enum.type_name, get_var_value),
            };

            buffer.push_line(&format!("{}.{} = {};", group.var_name, var.var_name, get_var_value));
        }
        buffer.new_line();
    }

    if master.validate_values {
        buffer.push_line(&format!("this.{}();", MASTER_VALIDATE_VALUES_FUNC_NAME));
    }
    buffer.push_line(&format!("super.{}();", MASTER_READ_SETTINGS_FUNC_NAME));

    buffer.pop_indent("}");
}

fn write_settings_function(master: &SettingsMaster, buffer: &mut WitcherScript) {
    buffer.push_line(&format!("public /* override */ function {}() : void", MASTER_WRITE_SETTINGS_FUNC_NAME))
          .push_indent("{");

    buffer.push_line("var config : CInGameConfigWrapper;")
          .push_line("config = theGame.GetInGameConfigWrapper();")
          .new_line();

    if master.validate_values {
        buffer.push_line(&format!("this.{}();", MASTER_VALIDATE_VALUES_FUNC_NAME))
              .new_line();
    }

    for group in &master.groups {
        for var in &group.vars {
            let var_value_str = match var.var_type {
                SettingsVarType::Bool => format!("BoolToString({}.{})", group.var_name, var.var_name),
                SettingsVarType::Int {..} => format!("IntToString({}.{})", group.var_name, var.var_name),
                SettingsVarType::Float {..} => format!("FloatToString({}.{})", group.var_name, var.var_name),
                SettingsVarType::Enum {..} => format!("IntToString((int){}.{})", group.var_name, var.var_name),
            };

            buffer.push_line(&format!("{}(config, '{}', '{}', {});", MASTER_WRITE_SETTING_VALUE_FUNC_NAME, group.id, var.id, var_value_str));
        }
        buffer.new_line();
    }

    buffer.push_line(&format!("super.{}();", MASTER_WRITE_SETTINGS_FUNC_NAME));
          
    buffer.pop_indent("}");
}

fn reset_settings_to_default_function(master: &SettingsMaster, buffer: &mut WitcherScript) {
    buffer.push_line(&format!("public /* override */ function {}() : void", MASTER_RESET_SETTINGS_TO_DEFAULT_FUNC_NAME))
          .push_indent("{");

    for group in &master.groups {
        buffer.push_line(&format!("{}.{}();", group.var_name, GROUP_RESET_SETTINGS_TO_DEFAULT_FUNC_NAME));
    }

    buffer.pop_indent("}");
}

fn should_reset_to_default_on_init_function(master: &SettingsMaster, buffer: &mut WitcherScript) {
    let group_id = &master.groups[0].id;
    let var_id = &master.groups[0].vars[0].id;

    buffer.push_line(&format!("public /* override */ function {}() : bool", MASTER_SHOULD_RESET_TO_DEFAULT_ON_INIT_FUNC_NAME))
          .push_indent("{");
    
    buffer.push_line("var config : CInGameConfigWrapper;")
          .push_line("config = theGame.GetInGameConfigWrapper();")
          .new_line()
          .push_line(&format!("return config.GetVarValue('{}','{}') == \"\";", group_id, var_id));
    
    buffer.pop_indent("}");
}